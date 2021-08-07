use crate::{
    api::ManagedTypeApi,
    managed_codec::{ManagedTopDecode, ManagedTopDecodeInput},
    signal_arg_de_error, ArgId, DynArgInput,
};
use elrond_codec::*;

/// Any type that is used as an endpoint argument must implement this trait.
pub trait DynArg<M: ManagedTypeApi>: Sized {
    fn dyn_load<I, D>(loader: &mut D, arg_id: ArgId) -> Self
    where
        I: ManagedTopDecodeInput<M>,
        D: DynArgInput<M, I>;
}

/// All top-deserializable types can be endpoint arguments.
impl<M, T> DynArg<M> for T
where
    M: ManagedTypeApi,
    T: ManagedTopDecode<M>,
{
    fn dyn_load<I, D>(loader: &mut D, arg_id: ArgId) -> Self
    where
        I: ManagedTopDecodeInput<M>,
        D: DynArgInput<M, I>,
    {
        if T::is_unit() {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return cast_unit;
        }

        let arg_input = loader.next_arg_input();
        T::top_decode_or_exit(arg_input, &(&*loader, arg_id), dyn_load_exit)
    }
}

#[inline(always)]
fn dyn_load_exit<M, I, D>(ctx: &(&D, ArgId), de_err: DecodeError) -> !
where
    M: ManagedTypeApi,
    I: ManagedTopDecodeInput<M>,
    D: DynArgInput<M, I>,
{
    let (loader, arg_id) = ctx;
    signal_arg_de_error(*loader, *arg_id, de_err)
}
