#[derive(Debug)]
pub struct BlockchainMockError(&'static str);

impl From<&'static str> for BlockchainMockError {
	fn from(s: &'static str) -> BlockchainMockError {
		BlockchainMockError(s)
	}
}

impl BlockchainMockError {
	pub fn as_str(&self) -> &str {
		self.0
	}

	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
