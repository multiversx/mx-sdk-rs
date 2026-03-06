use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::abi::TypeAbiFrom;

/// Type that ignores TypeAbiFrom restrictions.
///
/// Can be used to pass values to any argument, in any endpoint, regardless of type.
///
/// The any value type can be used, as long as it can be encoded.
/// Typically it is used with BytesValue, when converting from Mandos scenarios.
pub struct TypeAbiUniversalInput<T> {
    object: T,
}

impl<T> TypeAbiUniversalInput<T> {
    pub fn new<U>(value: U) -> Self
    where
        T: From<U>,
    {
        TypeAbiUniversalInput {
            object: T::from(value),
        }
    }
}

impl<T, U> TypeAbiFrom<TypeAbiUniversalInput<T>> for U {}

impl<T> TopEncode for TypeAbiUniversalInput<T>
where
    T: TopEncode,
{
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.object.top_encode_or_handle_err(output, h)
    }
}
