// use core::ops::Try;

use crate::{
    codec_err::DecodeError, nested_de_input::NestedDecodeInput, DecodeErrorHandler, TypeInfo,
};

// pub enum EarlyExit{}

// pub trait ResultProvider {
//     type Res: Try;

//     fn result_ok(&self, v: <Self::Res as Try>::Output) -> <Self::Res as Try>::Output;

//     fn result_err(&self, e: <Self::Res as Try>::Residual) -> <Self::Res as Try>::Residual;
// }

// pub struct DefaultResultProvider;

/// Trait that allows zero-copy read of value-references from slices in LE format.
pub trait NestedDecode: Sized {
    // !INTERNAL USE ONLY!
    // This const helps elrond-wasm to optimize the encoding/decoding by doing fake specialization.
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// Attempt to deserialise the value from input,
    /// using the format of an object nested inside another structure.
    /// In case of success returns the deserialized value and the number of bytes consumed during the operation.
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Self::dep_decode_or_err(input, |e| e)
    }

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller implementations
    /// in cases where the application is supposed to exit directly on decode error.
    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let result = Self::dep_decode_or_err(input, |e| exit(c.clone(), e));
        if let Ok(t) = result {
            t
        } else {
            unreachable!()
        }
    }

    fn dep_decode_or_err<I, EC, Err>(input: &mut I, err_closure: EC) -> Result<Self, Err>
    where
        I: NestedDecodeInput,
        EC: Fn(DecodeError) -> Err + Clone,
    {
        match Self::dep_decode(input) {
            Ok(v) => Ok(v),
            Err(e) => Err(err_closure(e)),
        }
    }

    fn dep_decode_or_handle_err<I, H>(input: &mut I, err_handler: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        match Self::dep_decode(input) {
            Ok(v) => Ok(v),
            Err(e) => Err(err_handler.handle_error(e)),
        }
    }
}
