use elrond_wasm::elrond_codec::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
	Inactive,
	Running,
	Ended,
	DistributingPrizes,
}

impl Status {
	pub fn to_u8(&self) -> u8 {
		match self {
			Status::Inactive => 0,
			Status::Running => 1,
			Status::Ended => 2,
			Status::DistributingPrizes => 3,
		}
	}

	pub fn from_u8(v: u8) -> Result<Self, DecodeError> {
		match v {
			0 => core::result::Result::Ok(Status::Inactive),
			1 => core::result::Result::Ok(Status::Running),
			2 => core::result::Result::Ok(Status::Ended),
			3 => core::result::Result::Ok(Status::DistributingPrizes),
			_ => core::result::Result::Err(DecodeError::INVALID_VALUE),
		}
	}
}

impl TopEncode for Status {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.to_u8().top_encode(output)
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
		self.to_u8().top_encode_or_exit(output, c, exit)
	}
}

impl TopDecode for Status {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Status::from_u8(u8::top_decode(input)?)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(input: I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
		match u8::top_decode_or_exit(input, c.clone(), exit) {
			0 => Status::Inactive,
			1 => Status::Running,
			2 => Status::Ended,
			3 => Status::DistributingPrizes,
			_ => exit(c, DecodeError::INVALID_VALUE),
		}
	}
}
