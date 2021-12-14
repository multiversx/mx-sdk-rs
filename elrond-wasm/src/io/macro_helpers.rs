use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi},
    *,
};
use elrond_codec::*;

pub fn load_single_arg<AA, T>(api: AA, index: i32, arg_id: ArgId) -> T
where
    T: TopDecode,
    AA: ManagedTypeApi + EndpointArgumentApi + ErrorApi,
{
    let input = ArgDecodeInput::new(api.clone(), index);
    let result = T::top_decode_err_closure(input, |e| -> ! {
        signal_arg_de_error(api.clone(), arg_id, e)
    });
    let Ok(value) = result;
    value
}

#[inline(always)]
fn load_single_arg_exit<AA>(ctx: (AA, ArgId), de_err: DecodeError) -> !
where
    AA: ManagedTypeApi + EndpointArgumentApi + ErrorApi,
{
    let (api, arg_id) = ctx;
    signal_arg_de_error(api, arg_id, de_err)
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
