#[derive(Debug, Default, Clone)]
pub struct TxResponseStatus {
    pub(crate) status: u64,
    pub message: String,
}

impl TxResponseStatus {
    pub(crate) fn new(status: u64, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }

    pub(crate) fn signal_error(message: &str) -> Self {
        Self::new(4, message)
    }

    pub fn is_success(&self) -> bool {
        self.status == 0
    }
}

impl std::fmt::Display for TxResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_success() {
            write!(f, "transaction successful")
        } else {
            write!(
                f,
                "transaction failed: (status: {}, message: {})",
                self.status, self.message
            )
        }
    }
}
