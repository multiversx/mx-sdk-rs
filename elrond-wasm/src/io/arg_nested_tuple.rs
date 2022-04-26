use elrond_codec::{TopDecodeMulti, TopDecodeMultiInput};

use crate::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl, ErrorApi, ManagedTypeApi},
    contract_base::ExitCodecErrorHandler,
    io::{ArgErrorHandler, ArgId, EndpointDynArgLoader, EndpointSingleArgLoader},
};

fn load_single_arg<AA, T>(index: i32, arg_id: ArgId) -> T
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    let mut arg_loader = EndpointSingleArgLoader::<AA>::new(index);
    let h = ArgErrorHandler::<AA>::from(arg_id);
    let Ok(value) = T::multi_decode_or_handle_err(&mut arg_loader, h);
    value
}

fn load_multi_arg<AA, L, T>(loader: &mut L, arg_id: ArgId) -> T
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
    L: TopDecodeMultiInput,
    T: TopDecodeMulti,
{
    let h = ArgErrorHandler::<AA>::from(arg_id);
    let result = T::multi_decode_or_handle_err(loader, h);
    let Ok(value) = result;
    value
}

pub trait ArgNestedTuple<AA>
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
{
    type ArgNames;

    fn check_num_single_args(index: i32);
    fn next_single_arg(index: i32, arg_names: Self::ArgNames) -> Self;
    fn next_multi_arg<L: TopDecodeMultiInput>(loader: L, arg_names: Self::ArgNames) -> Self;
}

impl<AA, Head, Tail> ArgNestedTuple<AA> for (Head, Tail)
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
    Head: TopDecodeMulti,
    Tail: ArgNestedTuple<AA>,
{
    type ArgNames = (&'static str, Tail::ArgNames);

    #[inline(always)]
    fn check_num_single_args(index: i32) {
        if Head::IS_SINGLE_VALUE {
            Tail::check_num_single_args(index + 1);
        } else {
            AA::argument_api_impl().check_num_arguments_ge(index);
        }
    }

    #[inline(always)]
    fn next_single_arg(index: i32, arg_names: Self::ArgNames) -> Self {
        if Head::IS_SINGLE_VALUE {
            let (arg_name, tail_names) = arg_names;
            let value = load_single_arg::<AA, Head>(index, ArgId::from(arg_name));
            (value, Tail::next_single_arg(index + 1, tail_names))
        } else {
            let loader = EndpointDynArgLoader::<AA>::new_at_index(index);
            Self::next_multi_arg(loader, arg_names)
        }
    }

    #[inline(always)]
    fn next_multi_arg<L: TopDecodeMultiInput>(mut loader: L, arg_names: Self::ArgNames) -> Self {
        let (arg_name, tail_names) = arg_names;
        let value = load_multi_arg::<AA, L, Head>(&mut loader, ArgId::from(arg_name));
        (value, Tail::next_multi_arg(loader, tail_names))
    }
}

impl<AA> ArgNestedTuple<AA> for ()
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
{
    type ArgNames = ();

    #[inline(always)]
    fn check_num_single_args(index: i32) {
        AA::argument_api_impl().check_num_arguments_eq(index);
    }

    #[inline]
    fn next_single_arg(_index: i32, _arg_names: Self::ArgNames) -> Self {}

    #[inline]
    fn next_multi_arg<L: TopDecodeMultiInput>(loader: L, _arg_names: Self::ArgNames) -> Self {
        let h = ExitCodecErrorHandler::<AA>::from(&[][..]);
        let Ok(()) = loader.assert_no_more_args(h);
    }
}

/// Used for loading all regular endpoint arguments. Call to it gets generated for all endpoints and callbacks.
#[inline(always)]
pub fn load_endpoint_args<AA, N>(arg_names: N::ArgNames) -> N
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
    N: ArgNestedTuple<AA>,
{
    N::check_num_single_args(0);
    N::next_single_arg(0, arg_names)
}

/// Currently used for the callback closure.
#[inline(always)]
pub fn load_multi_args_custom_loader<AA, L, N>(loader: L, arg_names: N::ArgNames) -> N
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
    L: TopDecodeMultiInput,
    N: ArgNestedTuple<AA>,
{
    N::next_multi_arg(loader, arg_names)
}
