#[derive(Debug, Default, Clone)]
/// The status of a transaction.
pub struct TxResponseStatus {
    /// The status of the transaction.
    pub status: u64,
    /// The message of the transaction.
    pub message: String,
}

impl TxResponseStatus {
    /// Creates a [`TxResponseStatus`]
    pub(crate) fn new(status: u64, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }

    /// Creates a [`TxResponseStatus`] that signals an error.
    pub(crate) fn signal_error(message: &str) -> Self {
        Self::new(4, message)
    }

    /// Checks if the transaction was successful.
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
