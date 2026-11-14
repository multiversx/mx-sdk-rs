use ledger_apdu::APDUCommand;
use multiversx_chain_core::std::crypto;

use super::{LedgerError, LedgerTransport};
use crate::wallet::Wallet;

const INS_GET_APP_CONFIG: u8 = 0x02;
const INS_GET_ADDRESS: u8 = 0x03;
const INS_SET_ADDRESS: u8 = 0x05;
const INS_SIGN_MESSAGE: u8 = 0x06;
const INS_SIGN_HASH_TX: u8 = 0x07;

/// A [`LedgerTransport`] backed by a software [`Wallet`].
///
/// Simulates the MultiversX Ledger app at the APDU level so that code written
/// against [`LedgerApp`](super::LedgerApp) can be tested end-to-end without
/// physical hardware.  Signing behaviour matches the real device:
///
/// - `INS_SIGN_HASH_TX`: signs `keccak256(payload)` with the wallet key.
/// - `INS_SIGN_MESSAGE`: signs the payload bytes directly.
///
/// Multi-chunk payloads are accumulated across calls (p1 `0x00` = first chunk,
/// `0x80` = continuation) and signed after each exchange, so the caller always
/// receives a valid signature response — only the last one matters.
pub struct WalletTransport {
    wallet: Wallet,
    sign_buffer: Vec<u8>,
}

impl WalletTransport {
    pub fn new(wallet: Wallet) -> Self {
        WalletTransport {
            wallet,
            sign_buffer: Vec::new(),
        }
    }
}

impl LedgerTransport for WalletTransport {
    fn exchange(&mut self, command: &APDUCommand<Vec<u8>>) -> Result<Vec<u8>, LedgerError> {
        match command.ins {
            INS_GET_APP_CONFIG => {
                // data_activated=true, account=0, address=0, version=0.0.0
                Ok(vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00])
            }
            INS_GET_ADDRESS => {
                let bech32 = self.wallet.to_bech32().to_string();
                let bytes = bech32.as_bytes();
                let mut response = Vec::with_capacity(1 + bytes.len());
                response.push(bytes.len() as u8);
                response.extend_from_slice(bytes);
                Ok(response)
            }
            INS_SET_ADDRESS => Ok(vec![]),
            INS_SIGN_HASH_TX | INS_SIGN_MESSAGE => {
                if command.p1 == 0x00 {
                    self.sign_buffer.clear();
                }
                self.sign_buffer.extend_from_slice(&command.data);

                let pk = self.wallet.private_key().ok_or_else(|| {
                    LedgerError::Transport(
                        "WalletTransport requires a private-key wallet, not a Ledger wallet".into(),
                    )
                })?;

                let data_to_sign: Vec<u8> = if command.ins == INS_SIGN_HASH_TX {
                    crypto::keccak256(&self.sign_buffer).to_vec()
                } else {
                    self.sign_buffer.clone()
                };

                let sig = pk.sign(&data_to_sign).to_bytes();
                let mut response = Vec::with_capacity(65);
                response.push(0x40);
                response.extend_from_slice(&sig);
                Ok(response)
            }
            ins => Err(LedgerError::InvalidResponse(format!(
                "WalletTransport: unknown instruction 0x{ins:02X}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::{PrivateKey, WalletSignature};
    use multiversx_chain_core::std::crypto;

    fn test_wallet() -> Wallet {
        // Known test key: all-zero seed.
        Wallet::from(PrivateKey::from_seed_bytes(&[0u8; 32]))
    }

    #[test]
    fn test_get_app_config() {
        let mut transport = WalletTransport::new(test_wallet());
        let cmd = APDUCommand {
            cla: 0xED,
            ins: INS_GET_APP_CONFIG,
            p1: 0,
            p2: 0,
            data: vec![],
        };
        let response = transport.exchange(&cmd).unwrap();
        assert_eq!(response.len(), 6);
        assert_eq!(response[0], 0x01); // data_activated
    }

    #[test]
    fn test_get_address_returns_bech32() {
        let mut transport = WalletTransport::new(test_wallet());
        let cmd = APDUCommand {
            cla: 0xED,
            ins: INS_GET_ADDRESS,
            p1: 0,
            p2: 0,
            data: vec![],
        };
        let response = transport.exchange(&cmd).unwrap();
        let len = response[0] as usize;
        let addr = std::str::from_utf8(&response[1..1 + len]).unwrap();
        assert!(
            addr.starts_with("erd1"),
            "expected bech32 with 'erd1' prefix, got {addr}"
        );
    }

    #[test]
    fn test_sign_hash_tx_matches_wallet_keccak() {
        let wallet = test_wallet();
        let payload = b"hello tx";

        let mut transport = WalletTransport::new(wallet.clone());
        let cmd = APDUCommand {
            cla: 0xED,
            ins: INS_SIGN_HASH_TX,
            p1: 0x00,
            p2: 0,
            data: payload.to_vec(),
        };
        let response = transport.exchange(&cmd).unwrap();
        assert_eq!(response[0], 0x40);
        let ledger_sig = WalletSignature::from_bytes(response[1..].try_into().unwrap());

        // The wallet signs keccak256(payload) for hash-based transactions.
        let hash = crypto::keccak256(payload);
        let expected_sig = wallet.private_key().unwrap().sign(hash);
        assert_eq!(ledger_sig.to_bytes(), expected_sig.to_bytes());
    }

    #[test]
    fn test_sign_message_matches_wallet_direct() {
        let wallet = test_wallet();
        let payload = b"hello message";

        let mut transport = WalletTransport::new(wallet.clone());
        let cmd = APDUCommand {
            cla: 0xED,
            ins: INS_SIGN_MESSAGE,
            p1: 0x00,
            p2: 0,
            data: payload.to_vec(),
        };
        let response = transport.exchange(&cmd).unwrap();
        assert_eq!(response[0], 0x40);
        let ledger_sig = WalletSignature::from_bytes(response[1..].try_into().unwrap());

        let expected_sig = wallet.private_key().unwrap().sign(payload);
        assert_eq!(ledger_sig.to_bytes(), expected_sig.to_bytes());
    }

    #[test]
    fn test_multi_chunk_accumulates_correctly() {
        let wallet = test_wallet();
        let chunk1 = vec![0xAAu8; 150];
        let chunk2 = vec![0xBBu8; 50];
        let full_payload = [chunk1.as_slice(), chunk2.as_slice()].concat();

        let mut transport = WalletTransport::new(wallet.clone());
        let cmd1 = APDUCommand {
            cla: 0xED,
            ins: INS_SIGN_HASH_TX,
            p1: 0x00,
            p2: 0,
            data: chunk1,
        };
        transport.exchange(&cmd1).unwrap(); // intermediate — discard
        let cmd2 = APDUCommand {
            cla: 0xED,
            ins: INS_SIGN_HASH_TX,
            p1: 0x80,
            p2: 0,
            data: chunk2,
        };
        let response = transport.exchange(&cmd2).unwrap();

        let ledger_sig = WalletSignature::from_bytes(response[1..].try_into().unwrap());
        let hash = crypto::keccak256(&full_payload);
        let expected_sig = wallet.private_key().unwrap().sign(hash);
        assert_eq!(ledger_sig.to_bytes(), expected_sig.to_bytes());
    }
}
