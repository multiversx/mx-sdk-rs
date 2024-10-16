use multiversx_sc::chain_core::types::ReturnCode;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// The status of a transaction.
pub struct TxResponseStatus {
    /// The status of the transaction.
    pub status: ReturnCode,
    /// The message of the transaction.
    pub message: String,
}

impl TxResponseStatus {
    /// Creates a [`TxResponseStatus`]
    pub fn new(status: ReturnCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }

    /// Creates a [`TxResponseStatus`] that signals an error.
    pub fn signal_error(message: &str) -> Self {
        Self::new(ReturnCode::UserError, message)
    }

    /// Checks if the transaction was successful.
    pub fn is_success(&self) -> bool {
        self.status.is_success()
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
