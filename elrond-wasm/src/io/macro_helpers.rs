use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi},
    *,
};
use elrond_codec::*;

pub fn load_single_arg<AA, T>(index: i32, arg_id: ArgId) -> T
where
    T: TopDecode,
    AA: ManagedTypeApi + EndpointArgumentApi + ErrorApi,
{
    let arg_input = ArgDecodeInput::<AA>::new(index);
    let h = ArgErrorHandler::<AA>::from(arg_id);
    let result = T::top_decode_or_handle_err(arg_input, h);
    let Ok(value) = result;
    value
}

/// It's easier to generate code from macros using this function, instead of the DynArg method.
#[inline]
pub fn load_dyn_arg<I, T>(loader: &mut I, arg_id: ArgId) -> T
where
    I: DynArgInput,
    T: DynArg,
{
    T::dyn_load(loader, arg_id)
}
