derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone, Copy, PartialEq)]
pub enum UserRole {
	None,
	Proposer,
	BoardMember,
}

impl UserRole {
	pub fn can_propose(&self) -> bool {
		matches!(*self, UserRole::BoardMember | UserRole::Proposer)
	}

	pub fn can_perform_action(&self) -> bool {
		self.can_propose()
	}

	pub fn can_sign(&self) -> bool {
<<<<<<< HEAD
		match *self {
			UserRole::BoardMember => true,
			_ => false,
		}
	}
}

impl TopEncode for UserRole {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, _: O) -> Result<(), EncodeError> {
		Err(EncodeError::UNSUPPORTED_OPERATION)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		match self {
			UserRole::None => 0u32.top_encode_or_exit(output, c, exit),
			UserRole::Proposer => 1u32.top_encode_or_exit(output, c, exit),
			UserRole::BoardMember => 2u32.top_encode_or_exit(output, c, exit),
		}
	}
}

impl TopDecode for UserRole {
	fn top_decode<I: TopDecodeInput>(_: I) -> Result<Self, DecodeError> {
		Err(DecodeError::UNSUPPORTED_OPERATION)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match u32::top_decode_or_exit(input, c.clone(), exit) {
			0 => UserRole::None,
			1 => UserRole::Proposer,
			2 => UserRole::BoardMember,
			_ => exit(c, DecodeError::INVALID_VALUE),
		}
=======
		matches!(*self, UserRole::BoardMember)
>>>>>>> master
	}
}
