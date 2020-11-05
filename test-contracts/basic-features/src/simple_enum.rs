use elrond_wasm::elrond_codec::*;

/// Copied from elrond-wasm serialization tests.
pub enum SimpleEnum {
	Variant0,
	Variant1,
	Variant2,
}

impl SimpleEnum {
	fn to_i64(&self) -> i64 {
		match self {
			SimpleEnum::Variant0 => 0,
			SimpleEnum::Variant1 => 1,
			SimpleEnum::Variant2 => 2,
		}
	}

	fn from_i64(i: i64) -> Result<Self, DecodeError> {
		match i {
			0 => Ok(SimpleEnum::Variant0),
			1 => Ok(SimpleEnum::Variant1),
			2 => Ok(SimpleEnum::Variant2),
			_ => Err(DecodeError::INPUT_OUT_OF_RANGE),
		}
	}
}

impl NestedEncode for SimpleEnum {
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.to_i64().dep_encode(dest)?;
		Ok(())
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(&self, dest: &mut O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
		self.to_i64().dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for SimpleEnum {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		output.set_i64(self.to_i64());
		Ok(())
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, _: ExitCtx, _: fn(ExitCtx, EncodeError) -> !) {
		output.set_i64(self.to_i64());
	}
}

impl NestedDecode for SimpleEnum {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		SimpleEnum::from_i64(i64::dep_decode(input)?)
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(input: &mut I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
		match u32::dep_decode_or_exit(input, c.clone(), exit) {
			0 => SimpleEnum::Variant0,
			1 => SimpleEnum::Variant1,
			2 => SimpleEnum::Variant2,
			_ => exit(c, DecodeError::INVALID_VALUE),
		}
	}
}

impl TopDecode for SimpleEnum {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		top_decode_from_nested(input)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(input: I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
		top_decode_from_nested_or_exit(input, c, exit)
	}
}
