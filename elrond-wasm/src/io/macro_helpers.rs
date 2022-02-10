use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi},
    *,
};
use elrond_codec::*;

pub fn load_single_arg<AA, T>(index: i32, arg_id: ArgId) -> T
where
    AA: ManagedTypeApi + EndpointArgumentApi + ErrorApi,
    T: TopDecode,
{
    let arg_input = ArgDecodeInput::<AA>::new(index);
    let h = ArgErrorHandler::<AA>::from(arg_id);
    let result = T::top_decode_or_handle_err(arg_input, h);
    let Ok(value) = result;
    value
}

/// It's easier to generate code from macros using this function, instead of the DynArg method.
pub fn load_dyn_arg<AA, I, T>(arg_input: &mut I, arg_id: ArgId) -> T
where
    AA: ManagedTypeApi + EndpointArgumentApi + ErrorApi,
    I: TopDecodeMultiInput,
    T: TopDecodeMulti,
{
    // let arg_input = ArgDecodeInput::<AA>::new(index);
    let h = ArgErrorHandler::<AA>::from(arg_id);
    let result = T::multi_decode_or_handle_err(arg_input, h);
    let Ok(value) = result;
    value

    // T::dyn_load(loader, arg_id)
}
