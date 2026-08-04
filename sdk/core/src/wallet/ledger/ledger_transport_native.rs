use ledger_apdu::APDUCommand;
use ledger_transport_hid::TransportNativeHID;

use super::LedgerError;


impl LedgerTransport for TransportNativeHID {
    fn exchange(&mut self, command: &APDUCommand<Vec<u8>>) -> Result<Vec<u8>, LedgerError> {
        let answer = TransportNativeHID::exchange(self, command)
            .map_err(|e| LedgerError::Transport(e.to_string()))?;

        let sw = answer.retcode();
        if sw != 0x9000 {
            return Err(LedgerError::from_status_word(sw));
        }

        Ok(answer.data().to_vec())
    }
}
