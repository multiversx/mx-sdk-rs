use multiversx_chain_core::types::ReturnCode;

#[derive(Debug, Clone)]
pub struct TxPanic {
    pub status: ReturnCode,
    pub message: String,
}

impl TxPanic {
    pub fn new(status: ReturnCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }

    pub fn user_error(message: &str) -> TxPanic {
        TxPanic::new(ReturnCode::UserError, message)
    }

    pub fn vm_error(message: &str) -> TxPanic {
        TxPanic::new(ReturnCode::ExecutionFailed, message)
    }
}
