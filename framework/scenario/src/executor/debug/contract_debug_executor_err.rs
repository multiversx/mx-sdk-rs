use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ContractDebugExecutorNotRegisteredError(String);

impl ContractDebugExecutorNotRegisteredError {
    pub fn new(contract_identifier: &[u8]) -> Self {
        ContractDebugExecutorNotRegisteredError(format_error_message(contract_identifier))
    }
}

fn format_error_message(contract_identifier: &[u8]) -> String {
    if let Ok(s) = std::str::from_utf8(contract_identifier) {
        format!("Unknown contract: {s}")
    } else {
        format!(
            "Unknown contract of length {} bytes",
            contract_identifier.len()
        )
    }
}

impl fmt::Display for ContractDebugExecutorNotRegisteredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for ContractDebugExecutorNotRegisteredError {}
