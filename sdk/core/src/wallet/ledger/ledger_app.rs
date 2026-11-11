use ledger_apdu::APDUCommand;
use ledger_transport_hid::{TransportNativeHID, hidapi::HidApi};

use super::{LedgerAppConfiguration, LedgerError, LedgerTransport};

pub(super) const CLA: u8 = 0xED;
pub(super) const INS_GET_APP_CONFIG: u8 = 0x02;
pub(super) const INS_GET_ADDRESS: u8 = 0x03;
pub(super) const INS_SET_ADDRESS: u8 = 0x05;
pub(super) const INS_SIGN_MESSAGE: u8 = 0x06;
pub(super) const INS_SIGN_HASH_TX: u8 = 0x07;

// Account index is always 0 for MultiversX.
const DEFAULT_ACCOUNT_INDEX: u32 = 0;

pub(super) const MAX_CHUNK_SIZE: usize = 150;
// Ledger responses: first byte is the length prefix (0x40 = 64), followed by 64 signature bytes.
pub(super) const EXPECTED_SIG_FIRST_BYTE: u8 = 0x40;
pub(super) const SIG_RESPONSE_LEN: usize = 65;
pub(super) const SIG_LEN: usize = 64;

/// Communicates with the MultiversX app on a connected Ledger hardware device.
///
/// Uses dynamic dispatch over [`LedgerTransport`] so different transports
/// (real HID device, [`WalletTransport`], or a test mock) can be plugged in
/// without changing the type signature of callers.
///
/// Only available when the `ledger` feature is enabled.
pub struct LedgerApp {
    transport: Box<dyn LedgerTransport>,
}

impl LedgerApp {
    /// Opens a connection to the first available Ledger device.
    ///
    /// Returns [`LedgerError::DeviceNotFound`] if no device is found or if the
    /// MultiversX app is not open on it.
    pub fn new() -> Result<Self, LedgerError> {
        let hidapi = HidApi::new().map_err(|e| LedgerError::Transport(e.to_string()))?;
        let transport =
            TransportNativeHID::new(&hidapi).map_err(|_| LedgerError::DeviceNotFound)?;
        Ok(LedgerApp {
            transport: Box::new(transport),
        })
    }

    /// Creates a [`LedgerApp`] backed by any [`LedgerTransport`] implementation.
    pub fn with_transport(transport: Box<dyn LedgerTransport>) -> Self {
        LedgerApp { transport }
    }
    /// Returns the app version string (`"MAJOR.MINOR.PATCH"`).
    pub fn get_version(&mut self) -> Result<String, LedgerError> {
        Ok(self.get_app_configuration()?.version)
    }

    /// Returns the full app configuration (version, data flag, account/address indexes).
    pub fn get_app_configuration(&mut self) -> Result<LedgerAppConfiguration, LedgerError> {
        let response = self.exchange(INS_GET_APP_CONFIG, 0x00, 0x00, &[])?;
        if response.len() < 6 {
            return Err(LedgerError::InvalidResponse(format!(
                "GET_APP_CONFIG: expected ≥6 bytes, got {}",
                response.len()
            )));
        }
        let data_activated = response[0] == 0x01;
        let account_index = response[1];
        let address_index = response[2];
        let version = format!("{}.{}.{}", response[3], response[4], response[5]);
        Ok(LedgerAppConfiguration {
            data_activated,
            account_index,
            address_index,
            version,
        })
    }

    /// Returns the bech32 address for the given `address_index` (account 0).
    pub fn get_address(&mut self, address_index: u32) -> Result<String, LedgerError> {
        let data = build_account_address_data(address_index);
        let response = self.exchange(INS_GET_ADDRESS, 0x00, 0x00, &data)?;
        let len = *response
            .first()
            .ok_or_else(|| LedgerError::InvalidResponse("empty GET_ADDRESS response".into()))?
            as usize;
        let address_bytes = response.get(1..1 + len).ok_or_else(|| {
            LedgerError::InvalidResponse(format!(
                "GET_ADDRESS: response too short for length prefix {len}"
            ))
        })?;
        String::from_utf8(address_bytes.to_vec())
            .map_err(|e| LedgerError::InvalidResponse(format!("non-UTF8 address: {e}")))
    }

    /// Sets the active address index on the device.
    pub fn set_address(&mut self, address_index: u32) -> Result<(), LedgerError> {
        let data = build_account_address_data(address_index);
        self.exchange(INS_SET_ADDRESS, 0x00, 0x00, &data)?;
        Ok(())
    }

    /// Signs a serialised transaction (raw JSON bytes) using the hash-based
    /// signing instruction (`0x07`).  The device hashes the payload internally.
    ///
    /// Returns the 64-byte ed25519 signature.
    pub fn sign_transaction(&mut self, tx_bytes: &[u8]) -> Result<[u8; SIG_LEN], LedgerError> {
        self.do_sign(tx_bytes, INS_SIGN_HASH_TX)
    }

    /// Signs an arbitrary message using the message signing instruction (`0x06`).
    ///
    /// Prepend the 4-byte big-endian length prefix before calling this function
    /// (the caller is responsible, matching the Python SDK).
    ///
    /// Returns the 64-byte ed25519 signature.
    pub fn sign_message(&mut self, message_bytes: &[u8]) -> Result<[u8; SIG_LEN], LedgerError> {
        self.do_sign(message_bytes, INS_SIGN_MESSAGE)
    }

    // ── Internal helpers ────────────────────────────────────────────────────

    /// Sends `data` to the device in chunks (≤150 bytes each) using `ins` as
    /// the signing instruction.  Returns the 64-byte signature.
    fn do_sign(&mut self, data: &[u8], ins: u8) -> Result<[u8; SIG_LEN], LedgerError> {
        let chunks: Vec<&[u8]> = data.chunks(MAX_CHUNK_SIZE).collect();
        let chunk_count = chunks.len();

        let mut last_response = Vec::new();
        for (i, chunk) in chunks.into_iter().enumerate() {
            let p1: u8 = if i == 0 { 0x00 } else { 0x80 };
            last_response = self.exchange(ins, p1, 0x00, chunk)?;
        }

        // After all chunks the device sends the signature: [0x40, sig[0..64]]
        if last_response.len() != SIG_RESPONSE_LEN || last_response[0] != EXPECTED_SIG_FIRST_BYTE {
            if chunk_count == 0 {
                return Err(LedgerError::InvalidResponse("empty data to sign".into()));
            }
            return Err(LedgerError::InvalidResponse(format!(
                "unexpected signature response length {} or first byte 0x{:02X}",
                last_response.len(),
                last_response.first().copied().unwrap_or(0)
            )));
        }

        let mut sig = [0u8; SIG_LEN];
        sig.copy_from_slice(&last_response[1..]);
        Ok(sig)
    }

    fn exchange(&mut self, ins: u8, p1: u8, p2: u8, data: &[u8]) -> Result<Vec<u8>, LedgerError> {
        let command = APDUCommand {
            cla: CLA,
            ins,
            p1,
            p2,
            data: data.to_vec(),
        };
        self.transport.exchange(&command)
    }
}

pub(super) fn build_account_address_data(address_index: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity(8);
    data.extend_from_slice(&DEFAULT_ACCOUNT_INDEX.to_be_bytes());
    data.extend_from_slice(&address_index.to_be_bytes());
    data
}
