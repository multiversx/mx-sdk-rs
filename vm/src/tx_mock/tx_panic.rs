#[derive(Debug, Clone)]
pub struct TxPanic {
    pub status: u64,
    pub message: String,
}

impl TxPanic {
    pub fn new(status: u64, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }

    pub fn user_error(message: &str) -> TxPanic {
        TxPanic::new(4, message)
    }

    pub fn vm_error(message: &str) -> TxPanic {
        TxPanic::new(10, message)
    }
}
