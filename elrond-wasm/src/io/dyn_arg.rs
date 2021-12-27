use crate::{api::ManagedTypeApi, signal_arg_de_error, ArgId, DynArgInput};
use elrond_codec::*;

/// Any type that is used as an endpoint argument must implement this trait.
pub trait DynArg: Sized {
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self;
}

/// All top-deserializable types can be endpoint arguments.
impl<T> DynArg for T
where
    T: TopEncode + TopDecode,
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

#[inline(always)]
fn dyn_load_exit<EA>(arg_id: ArgId, de_err: DecodeError) -> !
where
    EA: ManagedTypeApi,
{
    signal_arg_de_error::<EA>(arg_id, de_err)
}
