use elrond_wasm::elrond_codec::*;
use elrond_wasm::{Address, BigUintApi, BoxedBytes, CodeMetadata, Vec};

pub enum Action<BigUint: BigUintApi> {
	Nothing,
	AddBoardMember(Address),
	AddProposer(Address),
	RemoveUser(Address),
	ChangeQuorum(usize),
	SendEgld {
		to: Address,
		amount: BigUint,
	},
	SCDeploy {
		amount: BigUint,
		code: BoxedBytes,
		code_metadata: CodeMetadata,
		arguments: Vec<BoxedBytes>,
	},
	SCCall {
		to: Address,
		amount: BigUint,
		function: BoxedBytes,
		arguments: Vec<BoxedBytes>,
	},
}

impl<BigUint: BigUintApi> NestedEncode for Action<BigUint> {
	fn dep_encode<O: NestedEncodeOutput>(&self, _: &mut O) -> Result<(), EncodeError> {
		Err(EncodeError::UNSUPPORTED_OPERATION)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		match self {
			Action::Nothing => {
				0u8.dep_encode_or_exit(dest, c.clone(), exit);
			},
			Action::AddBoardMember(address) => {
				1u8.dep_encode_or_exit(dest, c.clone(), exit);
				address.dep_encode_or_exit(dest, c.clone(), exit);
			},
			Action::AddProposer(address) => {
				2u8.dep_encode_or_exit(dest, c.clone(), exit);
				address.dep_encode_or_exit(dest, c.clone(), exit);
			},
			Action::RemoveUser(address) => {
				3u8.dep_encode_or_exit(dest, c.clone(), exit);
				address.dep_encode_or_exit(dest, c.clone(), exit);
			},
			Action::ChangeQuorum(new_quorum) => {
				4u8.dep_encode_or_exit(dest, c.clone(), exit);
				new_quorum.dep_encode_or_exit(dest, c.clone(), exit);
			},
			Action::SendEgld { to, amount } => {
				5u8.dep_encode_or_exit(dest, c.clone(), exit);
				to.dep_encode_or_exit(dest, c.clone(), exit);
				amount.dep_encode_or_exit(dest, c.clone(), exit);
			},
			Action::SCDeploy {
				amount,
				code,
				code_metadata,
				arguments,
			} => {
				6u8.dep_encode_or_exit(dest, c.clone(), exit);
				amount.dep_encode_or_exit(dest, c.clone(), exit);
				code.dep_encode_or_exit(dest, c.clone(), exit);
				code_metadata.dep_encode_or_exit(dest, c.clone(), exit);
				arguments.dep_encode_or_exit(dest, c.clone(), exit);
			},
			Action::SCCall {
				to,
				amount,
				function,
				arguments,
			} => {
				7u8.dep_encode_or_exit(dest, c.clone(), exit);
				to.dep_encode_or_exit(dest, c.clone(), exit);
				amount.dep_encode_or_exit(dest, c.clone(), exit);
				function.dep_encode_or_exit(dest, c.clone(), exit);
				arguments.dep_encode_or_exit(dest, c.clone(), exit);
			},
		}
	}
}

impl<BigUint: BigUintApi> TopEncode for Action<BigUint> {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		top_encode_from_nested(self, output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		if let Action::Nothing = self {
			output.set_u64(0);
		} else {
			top_encode_from_nested_or_exit(self, output, c, exit);
		}
	}
}

impl<BigUint: BigUintApi> NestedDecode for Action<BigUint> {
	fn dep_decode<I: NestedDecodeInput>(_: &mut I) -> Result<Self, DecodeError> {
		Err(DecodeError::UNSUPPORTED_OPERATION)
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match u8::dep_decode_or_exit(input, c.clone(), exit) {
			0 => Action::Nothing,
			1 => Action::AddBoardMember(Address::dep_decode_or_exit(input, c.clone(), exit)),
			2 => Action::AddProposer(Address::dep_decode_or_exit(input, c.clone(), exit)),
			3 => Action::RemoveUser(Address::dep_decode_or_exit(input, c.clone(), exit)),
			4 => Action::ChangeQuorum(usize::dep_decode_or_exit(input, c.clone(), exit)),
			5 => Action::SendEgld {
				to: Address::dep_decode_or_exit(input, c.clone(), exit),
				amount: BigUint::dep_decode_or_exit(input, c.clone(), exit),
			},
			6 => Action::SCDeploy {
				amount: BigUint::dep_decode_or_exit(input, c.clone(), exit),
				code: BoxedBytes::dep_decode_or_exit(input, c.clone(), exit),
				code_metadata: CodeMetadata::dep_decode_or_exit(input, c.clone(), exit),
				arguments: Vec::<BoxedBytes>::dep_decode_or_exit(input, c.clone(), exit),
			},
			7 => Action::SCCall {
				to: Address::dep_decode_or_exit(input, c.clone(), exit),
				amount: BigUint::dep_decode_or_exit(input, c.clone(), exit),
				function: BoxedBytes::dep_decode_or_exit(input, c.clone(), exit),
				arguments: Vec::<BoxedBytes>::dep_decode_or_exit(input, c.clone(), exit),
			},
			_ => exit(c, DecodeError::INVALID_VALUE),
		}
	}
}

impl<BigUint: BigUintApi> TopDecode for Action<BigUint> {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		top_decode_from_nested(input)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		if input.byte_len() == 0 {
			Action::Nothing
		} else {
			top_decode_from_nested_or_exit(input, c, exit)
		}
	}
}
