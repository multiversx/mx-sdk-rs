use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use ledger_apdu::APDUCommand;

use super::{LedgerError, LedgerTransport};

/// A deterministic mock transport for unit testing.
///
/// Responses are queued up at construction time and consumed one-by-one as
/// [`exchange`](LedgerTransport::exchange) is called. Exhausting the queue
/// returns a `Transport` error.
///
/// Call [`commands_spy`](MockTransport::commands_spy) before boxing to
/// retain a handle for asserting on recorded APDUs after the transport has
/// been moved into a [`LedgerApp`].
pub struct MockTransport {
    responses: VecDeque<Result<Vec<u8>, LedgerError>>,
    commands: Rc<RefCell<Vec<APDUCommand<Vec<u8>>>>>,
}

impl MockTransport {
    pub fn new(responses: impl IntoIterator<Item = Result<Vec<u8>, LedgerError>>) -> Self {
        MockTransport {
            responses: responses.into_iter().collect(),
            commands: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Returns a clone of the shared command recorder for post-hoc assertion.
    pub fn commands_spy(&self) -> Rc<RefCell<Vec<APDUCommand<Vec<u8>>>>> {
        Rc::clone(&self.commands)
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
    fn exchange(&mut self, command: &APDUCommand<Vec<u8>>) -> Result<Vec<u8>, LedgerError> {
        self.commands.borrow_mut().push(command.clone());
        self.responses
            .pop_front()
            .unwrap_or_else(|| Err(LedgerError::Transport("mock: no more responses".into())))
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::ledger::{LedgerApp, ledger_app};

    use super::*;

    /// Creates a valid 65-byte mock signature response for a given 64-byte
    /// signature payload.
    pub fn sig_response(sig: [u8; ledger_app::SIG_LEN]) -> Vec<u8> {
        let mut r = vec![0x40u8];
        r.extend_from_slice(&sig);
        r
    }

    /// Returns a fixed 64-byte dummy signature (all `0xAB`).
    pub fn dummy_sig() -> [u8; ledger_app::SIG_LEN] {
        [0xABu8; ledger_app::SIG_LEN]
    }

    pub fn app_with(
        responses: impl IntoIterator<Item = Result<Vec<u8>, LedgerError>>,
    ) -> LedgerApp {
        LedgerApp::with_transport(Box::new(MockTransport::new(responses)))
    }

    // ── get_app_configuration ────────────────────────────────────────────────

    #[test]
    fn test_get_app_configuration_parses_response() {
        // Response: data_activated=true, account=0, address=0, version=1.2.5
        let mut app = app_with([MockTransport::ok(vec![0x01, 0x00, 0x00, 0x01, 0x02, 0x05])]);
        let config = app.get_app_configuration().unwrap();
        assert!(config.data_activated);
        assert_eq!(config.account_index, 0);
        assert_eq!(config.address_index, 0);
        assert_eq!(config.version, "1.2.5");
    }

    #[test]
    fn test_get_app_configuration_data_not_activated() {
        let mut app = app_with([MockTransport::ok(vec![0x00, 0x00, 0x00, 0x02, 0x00, 0x01])]);
        let config = app.get_app_configuration().unwrap();
        assert!(!config.data_activated);
        assert_eq!(config.version, "2.0.1");
    }

    #[test]
    fn test_get_app_configuration_too_short_returns_error() {
        let mut app = app_with([MockTransport::ok(vec![0x01, 0x00])]);
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

        let mut app = app_with([MockTransport::ok(response)]);
        assert_eq!(app.get_address(0).unwrap(), address);
    }

    #[test]
    fn test_get_address_sends_correct_apdu() {
        let address = "erd1abc";
        let mut response = vec![address.len() as u8];
        response.extend_from_slice(address.as_bytes());

        let transport = MockTransport::new([MockTransport::ok(response)]);
        let commands = transport.commands_spy();
        let mut app = LedgerApp::with_transport(Box::new(transport));
        let _ = app.get_address(3).unwrap();

        let cmds = commands.borrow();
        let cmd = &cmds[0];
        assert_eq!(cmd.cla, ledger_app::CLA);
        assert_eq!(cmd.ins, ledger_app::INS_GET_ADDRESS);
        assert_eq!(cmd.p1, 0x00);
        assert_eq!(cmd.p2, 0x00);
        // data = account_index(4 BE) + address_index(4 BE) = 0x00000000 + 0x00000003
        assert_eq!(cmd.data, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03]);
    }

    // ── sign_transaction chunking ────────────────────────────────────────────

    #[test]
    fn test_sign_single_chunk() {
        let data = vec![0x01u8; 100]; // under 150 bytes → one chunk
        let mut app = app_with([MockTransport::ok(sig_response(dummy_sig()))]);
        let sig = app.sign_transaction(&data).unwrap();
        assert_eq!(sig, dummy_sig());
    }

    #[test]
    fn test_sign_sends_correct_p1_for_first_chunk() {
        let data = vec![0x01u8; 100];
        let transport = MockTransport::new([MockTransport::ok(sig_response(dummy_sig()))]);
        let commands = transport.commands_spy();
        let mut app = LedgerApp::with_transport(Box::new(transport));
        let _ = app.sign_transaction(&data).unwrap();

        let cmds = commands.borrow();
        let cmd = &cmds[0];
        assert_eq!(cmd.ins, ledger_app::INS_SIGN_HASH_TX);
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
        let commands = transport.commands_spy();
        let mut app = LedgerApp::with_transport(Box::new(transport));
        let sig = app.sign_transaction(&data).unwrap();
        assert_eq!(sig, dummy_sig());

        let cmds = commands.borrow();
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
        let commands = transport.commands_spy();
        let mut app = LedgerApp::with_transport(Box::new(transport));
        let sig = app.sign_transaction(&data).unwrap();
        assert_eq!(sig, dummy_sig());

        let cmds = commands.borrow();
        assert_eq!(cmds[0].p1, 0x00);
        assert_eq!(cmds[1].p1, 0x80);
        assert_eq!(cmds[2].p1, 0x80);
        assert_eq!(cmds[0].data.len(), 150);
        assert_eq!(cmds[1].data.len(), 150);
        assert_eq!(cmds[2].data.len(), 100);
    }

    #[test]
    fn test_sign_empty_data_returns_error() {
        let mut app = app_with([] as [Result<Vec<u8>, LedgerError>; 0]);
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
        let commands = transport.commands_spy();
        let mut app = LedgerApp::with_transport(Box::new(transport));
        let _ = app.sign_message(data).unwrap();

        assert_eq!(commands.borrow()[0].ins, ledger_app::INS_SIGN_MESSAGE);
    }

    // ── error mapping ────────────────────────────────────────────────────────

    #[test]
    fn test_user_denied_error() {
        let mut app = app_with([MockTransport::err(0x6985)]);
        assert!(matches!(
            app.get_version(),
            Err(LedgerError::StatusWord(0x6985))
        ));
    }

    #[test]
    fn test_wrong_tx_options_error() {
        let mut app = app_with([MockTransport::err(0x6E0F)]);
        assert!(matches!(
            app.sign_transaction(b"test"),
            Err(LedgerError::StatusWord(0x6E0F))
        ));
    }

    // ── build_account_address_data ───────────────────────────────────────────

    #[test]
    fn test_address_data_encoding() {
        let data = ledger_app::build_account_address_data(7);
        // account index 0 (4 bytes BE) + address index 7 (4 bytes BE)
        assert_eq!(data, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07]);
    }
}
