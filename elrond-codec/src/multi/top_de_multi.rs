use crate::{DecodeError, TopDecodeMultiInput};

pub trait TopDecodeMulti: Sized {
    /// Attempt to deserialize the value from input.
    fn multi_decode<I: TopDecodeMultiInput>(input: I) -> Result<Self, DecodeError>;

    /// Version of `multi_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller implementations
    /// in cases where the application is supposed to exit directly on decode error.
    #[inline]
    fn multi_decode_or_exit<I: TopDecodeMultiInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match Self::multi_decode(input) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }
}

/// All top-deserializable types can be endpoint arguments.
impl<T> TopDecodeMulti for T
where
    T: TopDecode,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        if let TypeInfo::Unit = <T as TopDecode>::TYPE_INFO {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return cast_unit;
        }

        let arg_input = loader.next_arg_input();
        T::top_decode_or_exit(arg_input, arg_id, dyn_load_exit::<I::ManagedTypeErrorApi>)
    }
}