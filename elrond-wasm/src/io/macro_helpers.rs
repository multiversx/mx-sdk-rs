use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi},
    contract_base::ExitCodecErrorHandler,
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

/// Inserted everywhere an endpoint has a `#[var_args]` annotation.
pub fn load_dyn_arg<AA, I, T>(arg_input: &mut I, arg_id: ArgId) -> T
where
    AA: ManagedTypeApi + EndpointArgumentApi + ErrorApi,
    I: TopDecodeMultiInput,
    T: TopDecodeMulti,
{
    let h = ArgErrorHandler::<AA>::from(arg_id);
    let result = T::multi_decode_or_handle_err(arg_input, h);
    let Ok(value) = result;
    value
}

pub fn assert_no_more_args<A, I>(input: &I)
where
    A: ManagedTypeApi + ErrorApi,
    I: TopDecodeMultiInput,
{
    let h = ExitCodecErrorHandler::<A>::from(&[][..]);
    let Ok(()) = input.assert_no_more_args(h);
}
