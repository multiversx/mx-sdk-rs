use crate::api::{EndpointArgumentApi, ErrorApi};
use crate::*;
use elrond_codec::*;

#[inline]
pub fn load_single_arg<AA, T>(api: AA, index: i32, arg_id: ArgId) -> T
where
	T: TopDecode,
	AA: EndpointArgumentApi + ErrorApi + Clone + 'static,
{
	T::top_decode_or_exit(
		ArgDecodeInput::new(api.clone(), index),
		(api, arg_id),
		load_single_arg_exit,
	)
}

#[inline(always)]
fn load_single_arg_exit<AA>(ctx: (AA, ArgId), de_err: DecodeError) -> !
where
	AA: EndpointArgumentApi + ErrorApi + 'static,
{
	let (api, arg_id) = ctx;
	signal_arg_de_error(&api, arg_id, de_err)
}

/// It's easier to generate code from macros using this function, instead of the DynArg method.
#[inline]
pub fn load_dyn_arg<I, D, T>(loader: &mut D, arg_id: ArgId) -> T
where
	I: TopDecodeInput,
	D: DynArgInput<I>,
	T: DynArg<I, D>,
{
	T::dyn_load(loader, arg_id)
}
