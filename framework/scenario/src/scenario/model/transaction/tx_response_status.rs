#[derive(Debug, Default, Clone)]
pub struct TxResponseStatus {
    pub status: u64,
    pub message: String,
}

impl TxResponseStatus {
    pub fn is_success(&self) -> bool {
        self.status == 0
    }
}

impl std::fmt::Display for TxResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_success() {
            write!(f, "transaction successful")
        } else {
            write!(f, "transaction error: {}", self.message)
        }
    }
}
