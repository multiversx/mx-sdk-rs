use crate::api::ErrorApi;
use crate::types::*;
use elrond_codec::{TopEncode, TopEncodeOutput};

pub fn serialize_contract_call_arg<I, A>(arg: I, arg_buffer: &mut ArgBuffer, error_api: A)
where
	I: ContractCallArg,
	A: ErrorApi,
{
	// TODO: convert to fast exit
	if let Result::Err(sc_err) = arg.push_async_arg(arg_buffer) {
		error_api.signal_error(sc_err.as_bytes());
	}
}

/// Trait that specifies how arguments are serialized in contract calls.
///
/// TODO: unite with DynArg trait when reorganizing argument handling.
pub trait ContractCallArg: Sized {
	fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError>;
}

/// Local adapter the connects the ArgBuffer to the TopEncode trait.
struct ContractCallArgOutput<'s> {
	arg_buffer: &'s mut ArgBuffer,
}

impl<'c> ContractCallArgOutput<'c> {
	#[inline]
	fn new(arg_buffer: &'c mut ArgBuffer) -> Self {
		ContractCallArgOutput { arg_buffer }
	}
}

impl<'c> TopEncodeOutput for ContractCallArgOutput<'c> {
	fn set_slice_u8(self, bytes: &[u8]) {
		self.arg_buffer.push_argument_bytes(bytes);
	}
}

impl<T> ContractCallArg for T
where
	T: TopEncode,
{
	#[inline]
	#[allow(clippy::redundant_closure)]
	fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
		self.top_encode(ContractCallArgOutput::new(serializer))
			.map_err(|err| SCError::from(err))
	}
}

impl<T> ContractCallArg for VarArgs<T>
where
	T: ContractCallArg,
{
	fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
		for elem in self.0.iter() {
			elem.push_async_arg(serializer)?;
		}
		Ok(())
	}
}

impl<T> ContractCallArg for OptionalArg<T>
where
	T: ContractCallArg,
{
	#[inline]
	fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
		if let OptionalArg::Some(t) = self {
			t.push_async_arg(serializer)?;
		}
		Ok(())
	}
}

macro_rules! multi_arg_result_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            impl<$($name),+> ContractCallArg for $mr<$($name,)+>
            where
                $($name: ContractCallArg,)+
            {
                #[inline]
                fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
                    $(
                        (self.0).$n.push_async_arg(serializer)?;
                    )+
                    Ok(())
                }
            }
        )+
    }
}

multi_arg_result_impls! {
	(MultiArg2  0 T0 1 T1)
	(MultiArg3  0 T0 1 T1 2 T2)
	(MultiArg4  0 T0 1 T1 2 T2 3 T3)
	(MultiArg5  0 T0 1 T1 2 T2 3 T3 4 T4)
	(MultiArg6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
	(MultiArg7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
	(MultiArg8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
	(MultiArg9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
	(MultiArg10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
	(MultiArg11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
	(MultiArg12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
	(MultiArg13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
	(MultiArg14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
	(MultiArg15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
	(MultiArg16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

multi_arg_result_impls! {
	(MultiResult1  0 T0)
	(MultiResult2  0 T0 1 T1)
	(MultiResult3  0 T0 1 T1 2 T2)
	(MultiResult4  0 T0 1 T1 2 T2 3 T3)
	(MultiResult5  0 T0 1 T1 2 T2 3 T3 4 T4)
	(MultiResult6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
	(MultiResult7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
	(MultiResult8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
	(MultiResult9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
	(MultiResult10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
	(MultiResult11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
	(MultiResult12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
	(MultiResult13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
	(MultiResult14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
	(MultiResult15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
	(MultiResult16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
