use ledger_apdu::APDUCommand;

use super::LedgerError;

/// Abstraction over the physical transport layer.
///
/// Implementors send a single APDU command and return the response payload
/// (already status-word-checked).  The only production implementation is
/// [`TransportNativeHID`]; tests provide a [`MockTransport`].
pub trait LedgerTransport {
    fn exchange(&mut self, command: &APDUCommand<Vec<u8>>) -> Result<Vec<u8>, LedgerError>;
}
