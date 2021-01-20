pub trait EncodeDefault {
	fn is_default(&self) -> bool;
}

pub trait DecodeDefault {
	fn default() -> Self;
}
