/// Configuration returned by the `GET_APP_CONFIG` (`0x02`) instruction.
#[derive(Debug, Clone)]
pub struct LedgerAppConfiguration {
    pub data_activated: bool,
    pub account_index: u8,
    pub address_index: u8,
    /// Version string in the form `"MAJOR.MINOR.PATCH"`.
    pub version: String,
}
