use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use colored::Colorize;
use wasmparser::BinaryReaderError;

#[derive(Debug)]
pub enum WasmError {
    WasmParserError(BinaryReaderError),
    ForbiddenOpcode(String),
}

impl Error for WasmError {}

impl Display for WasmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message: String = match self {
            WasmError::WasmParserError(error) => error.message().to_owned(),
            WasmError::ForbiddenOpcode(op) => {
                let op_vec: Vec<&str> = op.split_whitespace().collect();

                format!(
                    "{} {} {}",
                    "Operator".to_string().red().bold(),
                    op_vec[0].red().bold(),
                    "not supported for VM execution".to_string().red().bold(),
                )
            },
        };
        write!(f, "{} {}", "WasmError:".red().bold(), message)
    }
}
