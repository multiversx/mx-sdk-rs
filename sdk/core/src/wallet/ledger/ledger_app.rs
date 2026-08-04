use ledger_apdu::APDUCommand;
use ledger_transport_hid::{TransportNativeHID, hidapi::HidApi};

use super::{LedgerAppConfiguration, LedgerError, LedgerTransport};

const CLA: u8 = 0xED;
const INS_GET_APP_CONFIG: u8 = 0x02;
const INS_GET_ADDRESS: u8 = 0x03;
const INS_SET_ADDRESS: u8 = 0x05;
const INS_SIGN_MESSAGE: u8 = 0x06;
const INS_SIGN_HASH_TX: u8 = 0x07;

// Account index is always 0 for MultiversX.
const DEFAULT_ACCOUNT_INDEX: u32 = 0;

const MAX_CHUNK_SIZE: usize = 150;
// Ledger responses: first byte is the length prefix (0x40 = 64), followed by 64 signature bytes.
const EXPECTED_SIG_FIRST_BYTE: u8 = 0x40;
const SIG_RESPONSE_LEN: usize = 65;
const SIG_LEN: usize = 64;

/// Communicates with the MultiversX app on a connected Ledger hardware device.
///
/// Generic over [`LedgerTransport`] so that unit tests can inject a
/// [`MockTransport`] instead of a real HID device.
///
/// The default type parameter is the production [`TransportNativeHID`].
/// Outside of tests, construct with [`LedgerApp::new`].
///
/// Only available when the `ledger` feature is enabled.
pub struct LedgerApp<T: LedgerTransport = TransportNativeHID> {
    transport: T,
}

impl LedgerApp<TransportNativeHID> {
    /// Opens a connection to the first available Ledger device.
    ///
    /// Returns [`LedgerError::DeviceNotFound`] if no device is found or if the
    /// MultiversX app is not open on it.
    pub fn new() -> Result<Self, LedgerError> {
        let hidapi = HidApi::new().map_err(|e| LedgerError::Transport(e.to_string()))?;
        let transport =
            TransportNativeHID::new(&hidapi).map_err(|_| LedgerError::DeviceNotFound)?;
        Ok(LedgerApp { transport })
    }
}

impl<T: LedgerTransport> LedgerApp<T> {
    /// Returns the app version string (`"MAJOR.MINOR.PATCH"`).
    pub fn get_version(&self) -> Result<String, LedgerError> {
        Ok(self.get_app_configuration()?.version)
    }

    /// Returns the full app configuration (version, data flag, account/address indexes).
    pub fn get_app_configuration(&self) -> Result<LedgerAppConfiguration, LedgerError> {
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
    pub fn get_address(&self, address_index: u32) -> Result<String, LedgerError> {
        let data = build_account_address_data(address_index);
        let response = self.exchange(INS_GET_ADDRESS, 0x00, 0x00, &data)?;
        // First byte is the length of the address string.
        let address_bytes = response
            .get(1..)
            .ok_or_else(|| LedgerError::InvalidResponse("empty GET_ADDRESS response".into()))?;
        String::from_utf8(address_bytes.to_vec())
            .map_err(|e| LedgerError::InvalidResponse(format!("non-UTF8 address: {e}")))
    }

    /// Sets the active address index on the device.
    pub fn set_address(&self, address_index: u32) -> Result<(), LedgerError> {
        let data = build_account_address_data(address_index);
        self.exchange(INS_SET_ADDRESS, 0x00, 0x00, &data)?;
        Ok(())
    }

    /// Signs a serialised transaction (raw JSON bytes) using the hash-based
    /// signing instruction (`0x07`).  The device hashes the payload internally.
    ///
    /// Returns the 64-byte ed25519 signature.
    pub fn sign_transaction(&self, tx_bytes: &[u8]) -> Result<[u8; SIG_LEN], LedgerError> {
        self.do_sign(tx_bytes, INS_SIGN_HASH_TX)
    }

    /// Signs an arbitrary message using the message signing instruction (`0x06`).
    ///
    /// Prepend the 4-byte big-endian length prefix before calling this function
    /// (the caller is responsible, matching the Python SDK).
    ///
    /// Returns the 64-byte ed25519 signature.
    pub fn sign_message(&self, message_bytes: &[u8]) -> Result<[u8; SIG_LEN], LedgerError> {
        self.do_sign(message_bytes, INS_SIGN_MESSAGE)
    }

    // ── Internal helpers ────────────────────────────────────────────────────

    /// Sends `data` to the device in chunks (≤150 bytes each) using `ins` as
    /// the signing instruction.  Returns the 64-byte signature.
    fn do_sign(&self, data: &[u8], ins: u8) -> Result<[u8; SIG_LEN], LedgerError> {
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

    /// Delegates to the transport, building the [`APDUCommand`] from the
    /// separate fields.
    fn exchange(&self, ins: u8, p1: u8, p2: u8, data: &[u8]) -> Result<Vec<u8>, LedgerError> {
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

fn build_account_address_data(address_index: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity(8);
    data.extend_from_slice(&DEFAULT_ACCOUNT_INDEX.to_be_bytes());
    data.extend_from_slice(&address_index.to_be_bytes());
    data
}

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
pub mod mock {
    use std::cell::RefCell;
    use std::collections::VecDeque;

    use ledger_apdu::APDUCommand;

    use super::super::LedgerError;
    use super::{LedgerApp, LedgerTransport, SIG_LEN};

    /// A deterministic mock transport for unit testing.
    ///
    /// Responses are queued up at construction time and consumed one-by-one as
    /// [`exchange`](LedgerTransport::exchange) is called.  Exhausting the queue
    /// returns a `Transport` error.
    ///
    /// The mock also records every command it receives so tests can assert on
    /// which APDUs were sent.
    pub struct MockTransport {
        responses: RefCell<VecDeque<Result<Vec<u8>, LedgerError>>>,
        pub commands: RefCell<Vec<APDUCommand<Vec<u8>>>>,
    }

    impl MockTransport {
        pub fn new(responses: impl IntoIterator<Item = Result<Vec<u8>, LedgerError>>) -> Self {
            MockTransport {
                responses: RefCell::new(responses.into_iter().collect()),
                commands: RefCell::new(Vec::new()),
            }
        }

        /// Convenience: queue a success response with the given payload bytes.
        pub fn ok(bytes: impl Into<Vec<u8>>) -> Result<Vec<u8>, LedgerError> {
            Ok(bytes.into())
        }

        /// Convenience: queue a status-word error response.
        pub fn err(sw: u16) -> Result<Vec<u8>, LedgerError> {
            Err(LedgerError::from_status_word(sw))
        }
    }

    impl LedgerTransport for MockTransport {
        fn exchange(&self, command: &APDUCommand<Vec<u8>>) -> Result<Vec<u8>, LedgerError> {
            self.commands.borrow_mut().push(command.clone());
            self.responses
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| Err(LedgerError::Transport("mock: no more responses".into())))
        }
    }

    /// Creates a valid 65-byte mock signature response for a given 64-byte
    /// signature payload.
    pub fn sig_response(sig: [u8; SIG_LEN]) -> Vec<u8> {
        let mut r = vec![0x40u8];
        r.extend_from_slice(&sig);
        r
    }

    /// Returns a fixed 64-byte dummy signature (all `0xAB`).
    pub fn dummy_sig() -> [u8; SIG_LEN] {
        [0xABu8; SIG_LEN]
    }

    pub fn app_with(responses: impl IntoIterator<Item = Result<Vec<u8>, LedgerError>>) -> LedgerApp<MockTransport> {
        LedgerApp {
            transport: MockTransport::new(responses),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mock::*;
    use super::*;

    // ── get_app_configuration ────────────────────────────────────────────────

    #[test]
    fn test_get_app_configuration_parses_response() {
        // Response: data_activated=true, account=0, address=0, version=1.2.5
        let app = app_with([MockTransport::ok(vec![0x01, 0x00, 0x00, 0x01, 0x02, 0x05])]);
        let config = app.get_app_configuration().unwrap();
        assert!(config.data_activated);
        assert_eq!(config.account_index, 0);
        assert_eq!(config.address_index, 0);
        assert_eq!(config.version, "1.2.5");
    }

    #[test]
    fn test_get_app_configuration_data_not_activated() {
        let app = app_with([MockTransport::ok(vec![0x00, 0x00, 0x00, 0x02, 0x00, 0x01])]);
        let config = app.get_app_configuration().unwrap();
        assert!(!config.data_activated);
        assert_eq!(config.version, "2.0.1");
    }

    #[test]
    fn test_get_app_configuration_too_short_returns_error() {
        let app = app_with([MockTransport::ok(vec![0x01, 0x00])]);
        assert!(matches!(
            app.get_app_configuration(),
            Err(LedgerError::InvalidResponse(_))
        ));
    }

    // ── get_address ──────────────────────────────────────────────────────────

    #[test]
    fn test_get_address_parses_response() {
        let address = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";
        // Response: [length_byte, address_bytes...]
        let mut response = vec![address.len() as u8];
        response.extend_from_slice(address.as_bytes());

        let app = app_with([MockTransport::ok(response)]);
        assert_eq!(app.get_address(0).unwrap(), address);
    }

    #[test]
    fn test_get_address_sends_correct_apdu() {
        let address = "erd1abc";
        let mut response = vec![address.len() as u8];
        response.extend_from_slice(address.as_bytes());

        let transport = MockTransport::new([MockTransport::ok(response)]);
        let app = LedgerApp { transport };
        let _ = app.get_address(3).unwrap();

        let cmd = &app.transport.commands.borrow()[0];
        assert_eq!(cmd.cla, CLA);
        assert_eq!(cmd.ins, INS_GET_ADDRESS);
        assert_eq!(cmd.p1, 0x00);
        assert_eq!(cmd.p2, 0x00);
        // data = account_index(4 BE) + address_index(4 BE) = 0x00000000 + 0x00000003
        assert_eq!(cmd.data, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03]);
    }

    // ── sign_transaction chunking ────────────────────────────────────────────

    #[test]
    fn test_sign_single_chunk() {
        let data = vec![0x01u8; 100]; // under 150 bytes → one chunk
        let app = app_with([MockTransport::ok(sig_response(dummy_sig()))]);
        let sig = app.sign_transaction(&data).unwrap();
        assert_eq!(sig, dummy_sig());
    }

    #[test]
    fn test_sign_sends_correct_p1_for_first_chunk() {
        let data = vec![0x01u8; 100];
        let transport = MockTransport::new([MockTransport::ok(sig_response(dummy_sig()))]);
        let app = LedgerApp { transport };
        let _ = app.sign_transaction(&data).unwrap();

        let cmd = &app.transport.commands.borrow()[0];
        assert_eq!(cmd.ins, INS_SIGN_HASH_TX);
        assert_eq!(cmd.p1, 0x00); // first chunk
    }

    #[test]
    fn test_sign_two_chunks_correct_p1() {
        // 200 bytes → chunk 1: 150 bytes (p1=0x00), chunk 2: 50 bytes (p1=0x80)
        let data = vec![0x42u8; 200];
        let transport = MockTransport::new([
            MockTransport::ok(vec![]),                    // intermediate chunk
            MockTransport::ok(sig_response(dummy_sig())), // final chunk with signature
        ]);
        let app = LedgerApp { transport };
        let sig = app.sign_transaction(&data).unwrap();
        assert_eq!(sig, dummy_sig());

        let cmds = app.transport.commands.borrow();
        assert_eq!(cmds.len(), 2);
        assert_eq!(cmds[0].p1, 0x00); // first chunk
        assert_eq!(cmds[0].data.len(), 150);
        assert_eq!(cmds[1].p1, 0x80); // continuation chunk
        assert_eq!(cmds[1].data.len(), 50);
    }

    #[test]
    fn test_sign_three_chunks() {
        // 400 bytes → 3 chunks: 150 + 150 + 100
        let data = vec![0xFFu8; 400];
        let transport = MockTransport::new([
            MockTransport::ok(vec![]),
            MockTransport::ok(vec![]),
            MockTransport::ok(sig_response(dummy_sig())),
        ]);
        let app = LedgerApp { transport };
        let sig = app.sign_transaction(&data).unwrap();
        assert_eq!(sig, dummy_sig());

        let cmds = app.transport.commands.borrow();
        assert_eq!(cmds[0].p1, 0x00);
        assert_eq!(cmds[1].p1, 0x80);
        assert_eq!(cmds[2].p1, 0x80);
        assert_eq!(cmds[0].data.len(), 150);
        assert_eq!(cmds[1].data.len(), 150);
        assert_eq!(cmds[2].data.len(), 100);
    }

    #[test]
    fn test_sign_empty_data_returns_error() {
        let app = app_with([] as [Result<Vec<u8>, LedgerError>; 0]);
        assert!(matches!(
            app.sign_transaction(&[]),
            Err(LedgerError::InvalidResponse(_))
        ));
    }

    // ── sign_message ─────────────────────────────────────────────────────────

    #[test]
    fn test_sign_message_uses_correct_ins() {
        let data = b"hello";
        let transport = MockTransport::new([MockTransport::ok(sig_response(dummy_sig()))]);
        let app = LedgerApp { transport };
        let _ = app.sign_message(data).unwrap();

        assert_eq!(app.transport.commands.borrow()[0].ins, INS_SIGN_MESSAGE);
    }

    // ── error mapping ────────────────────────────────────────────────────────

    #[test]
    fn test_user_denied_error() {
        let app = app_with([MockTransport::err(0x6985)]);
        assert!(matches!(
            app.get_version(),
            Err(LedgerError::StatusWord(0x6985))
        ));
    }

    #[test]
    fn test_wrong_tx_options_error() {
        let app = app_with([MockTransport::err(0x6E0F)]);
        assert!(matches!(
            app.sign_transaction(b"test"),
            Err(LedgerError::StatusWord(0x6E0F))
        ));
    }

    // ── build_account_address_data ───────────────────────────────────────────

    #[test]
    fn test_address_data_encoding() {
        let data = build_account_address_data(7);
        // account index 0 (4 bytes BE) + address index 7 (4 bytes BE)
        assert_eq!(data, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07]);
    }
}

