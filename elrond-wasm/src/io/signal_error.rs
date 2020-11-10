use crate::BoxedBytes;
use crate::*;
use core::marker::PhantomData;
use elrond_codec::DecodeError;

/// Some info to display in endpoint argument deserialization error messages,
/// to help users identify the faulty argument.
/// Generated automatically.
/// Current version uses argument names,
/// but in principle it could be changed to argument index to save some bytes from the wasm output.
#[derive(Clone, Copy)]
pub struct ArgId(&'static [u8]);

impl From<&'static [u8]> for ArgId {
	#[inline]
	fn from(static_bytes: &'static [u8]) -> Self {
		ArgId(static_bytes)
	}
}

impl ArgId {
	fn as_bytes(&self) -> &'static [u8] {
		self.0
	}

	#[inline]
	pub fn empty() -> Self {
		ArgId::from(&[][..])
	}
}

pub trait SignalError {
	fn signal_error(&self, message: &[u8]) -> !;

	fn signal_arg_de_error(&self, arg_id: ArgId, de_err: DecodeError) -> ! {
		let decode_err_message = BoxedBytes::from_concat(
			&[
				err_msg::ARG_DECODE_ERROR_1,
				arg_id.as_bytes(),
				err_msg::ARG_DECODE_ERROR_2,
				de_err.message_bytes(),
			][..],
		);
		self.signal_error(decode_err_message.as_slice())
	}

	#[inline]
	fn signal_arg_wrong_number(&self) -> ! {
		self.signal_error(err_msg::ARG_WRONG_NUMBER)
	}
}

pub struct ApiSignalError<A, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractIOApi<BigInt, BigUint> + 'static,
{
	api: A,
	_phantom1: PhantomData<BigInt>,
	_phantom2: PhantomData<BigUint>,
}

impl<A, BigInt, BigUint> ApiSignalError<A, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractIOApi<BigInt, BigUint> + 'static,
{
	pub fn new(api: A) -> Self {
		ApiSignalError {
			api,
			_phantom1: PhantomData,
			_phantom2: PhantomData,
		}
	}
}

impl<A, BigInt, BigUint> SignalError for ApiSignalError<A, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractIOApi<BigInt, BigUint> + 'static,
{
	fn signal_error(&self, message: &[u8]) -> ! {
		self.api.signal_error(message)
	}
}

/// An error handler that simply panics whenever `signal_error` is called.
/// Especially useful for unit tests.
pub struct PanickingSignalError;

impl SignalError for PanickingSignalError {
	fn signal_error(&self, message: &[u8]) -> ! {
		panic!(
			"PanickingDynArgErrHandler panicked: {}",
			core::str::from_utf8(message).unwrap()
		)
	}
}
