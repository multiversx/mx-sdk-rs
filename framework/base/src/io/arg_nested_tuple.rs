use super::{EndpointDynArgLoader, EndpointSingleArgLoader, ManagedResultArgLoader};
use crate::{
    api::{
        const_handles, use_raw_handle, EndpointArgumentApi, EndpointArgumentApiImpl, ErrorApi,
        ErrorApiImpl, ManagedTypeApi, StaticVarApiImpl, VMApi,
    },
    codec::{DecodeError, TopDecodeMulti, TopDecodeMultiInput},
    err_msg,
    io::{ArgErrorHandler, ArgId},
    types::{ManagedArgBuffer, ManagedBuffer, ManagedType},
};

/// Argument count cannot change during execution, and it can get queried multiple times,
/// so it makes sense to save it statically.
///
/// Especially the `EndpointDynArgLoader` repeatedly needs this value, keeping statically means it is no longer carried around.
fn init_arguments_static_data<'a, AA>()
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
{
    AA::static_var_api_impl().set_num_arguments(AA::argument_api_impl().get_num_arguments());
}

/// Check that number of arguments is equal to value.
///
/// Since in this scenario this will be the only check, there is no need to load the argument count to static.
///
/// Inline prevented following an investigation.
#[inline(never)]
fn check_num_arguments_eq<'a, AA>(expected: i32)
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
{
    if AA::argument_api_impl().get_num_arguments() != expected {
        AA::error_api_impl().signal_error(err_msg::ARG_WRONG_NUMBER.as_bytes());
    }
}

/// Check that number of arguments is greater or equal than value.
///
/// Condition occurs when single args are followed by var-args.
///
/// Inline prevented following an investigation.
#[inline(never)]
fn check_num_arguments_ge<'a, AA>(expected: i32)
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
{
    if AA::static_var_api_impl().get_num_arguments() < expected {
        AA::error_api_impl().signal_error(DecodeError::MULTI_TOO_FEW_ARGS.message_bytes());
    }
}

/// Check that loader went through all existing arguments.
#[inline(never)]
fn check_no_more_args<'a, AA, L>(loader: L)
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
    L: TopDecodeMultiInput,
{
    if loader.has_next() {
        AA::error_api_impl().signal_error(DecodeError::MULTI_TOO_MANY_ARGS.message_bytes());
    }
}

#[inline(never)]
fn load_single_arg<'a, AA, T>(index: i32, arg_id: ArgId) -> T
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
    T: TopDecodeMulti,
{
    let mut arg_loader = EndpointSingleArgLoader::<'a, AA>::new(index);
    let h = ArgErrorHandler::<'a, AA>::from(arg_id);
    let Ok(value) = T::multi_decode_or_handle_err(&mut arg_loader, h);
    value
}

#[inline(never)]
fn load_multi_arg<'a, AA, L, T>(loader: &mut L, arg_id: ArgId) -> T
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
    L: TopDecodeMultiInput,
    T: TopDecodeMulti,
{
    let h = ArgErrorHandler::<'a, AA>::from(arg_id);
    let result = T::multi_decode_or_handle_err(loader, h);
    let Ok(value) = result;
    value
}

/// Models an argument tree of the form `(arg1, (arg2, ... (argn, ())))`, used for retrieving endpoint arguments.
///
/// It translates to a small algorithm determined at compile-time. That is why all methods are inlined.
pub trait ArgNestedTuple<'a, AA>
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
{
    type ArgNames;

    fn check_num_single_args(index: i32);
    fn next_single_arg(index: i32, arg_names: Self::ArgNames) -> Self;
    fn next_multi_arg<L: TopDecodeMultiInput>(loader: L, arg_names: Self::ArgNames) -> Self;
}

impl<'a, AA, Head, Tail> ArgNestedTuple<'a, AA> for (Head, Tail)
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
    Head: TopDecodeMulti,
    Tail: ArgNestedTuple<'a, AA>,
{
    type ArgNames = (&'static str, Tail::ArgNames);

    #[inline(always)]
    fn check_num_single_args(index: i32) {
        if Head::IS_SINGLE_VALUE {
            Tail::check_num_single_args(index + 1);
        } else {
            // both check_num_arguments_ge and EndpointDynArgLoader need it in the future
            init_arguments_static_data::<'a, AA>();

            check_num_arguments_ge::<'a, AA>(index);
        }
    }

    #[inline(always)]
    fn next_single_arg(index: i32, arg_names: Self::ArgNames) -> Self {
        if Head::IS_SINGLE_VALUE {
            let (arg_name, tail_names) = arg_names;
            let value = load_single_arg::<'a, AA, Head>(index, ArgId::from(arg_name));
            (value, Tail::next_single_arg(index + 1, tail_names))
        } else {
            let loader = EndpointDynArgLoader::<'a, AA>::new_at_index(index);
            Self::next_multi_arg(loader, arg_names)
        }
    }

    #[inline(always)]
    fn next_multi_arg<L: TopDecodeMultiInput>(mut loader: L, arg_names: Self::ArgNames) -> Self {
        let (arg_name, tail_names) = arg_names;
        let value = load_multi_arg::<'a, AA, L, Head>(&mut loader, ArgId::from(arg_name));
        (value, Tail::next_multi_arg(loader, tail_names))
    }
}

impl<'a, AA> ArgNestedTuple<'a, AA> for ()
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
{
    type ArgNames = ();

    #[inline(always)]
    fn check_num_single_args(index: i32) {
        check_num_arguments_eq::<'a, AA>(index);
    }

    #[inline(always)]
    fn next_single_arg(_index: i32, _arg_names: Self::ArgNames) -> Self {}

    #[inline(always)]
    fn next_multi_arg<L: TopDecodeMultiInput>(loader: L, _arg_names: Self::ArgNames) -> Self {
        check_no_more_args::<'a, AA, L>(loader);
    }
}

/// Used for loading all regular endpoint arguments. A call to this gets generated for all endpoints and callbacks.
#[inline(always)]
pub fn load_endpoint_args<'a, AA, N>(arg_names: N::ArgNames) -> N
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
    N: ArgNestedTuple<'a, AA>,
{
    N::check_num_single_args(0);
    N::next_single_arg(0, arg_names)
}

#[inline(always)]
pub fn load_callback_closure_args<'a, AA, N>(arg_names: N::ArgNames) -> N
where
    AA: VMApi<'a>,
    N: ArgNestedTuple<'a, AA>,
{
    let loader = callback_closure_args_loader::<'a, AA>();
    N::next_multi_arg(loader, arg_names)
}

/// Currently used for the callback closure. No distinction there for single values.
#[inline(always)]
pub fn load_multi_args_custom_loader<'a, AA, L, N>(loader: L, arg_names: N::ArgNames) -> N
where
    AA: EndpointArgumentApi<'a> + ManagedTypeApi<'a> + ErrorApi,
    L: TopDecodeMultiInput,
    N: ArgNestedTuple<'a, AA>,
{
    init_arguments_static_data::<'a, AA>();
    N::next_multi_arg(loader, arg_names)
}

fn callback_closure_args_loader<'a, AA>() -> ManagedResultArgLoader<'a, AA>
where
    AA: VMApi<'a>,
{
    AA::argument_api_impl()
        .load_callback_closure_buffer(use_raw_handle(const_handles::MBUF_TEMPORARY_1));
    let cb_closure_args_serialized =
        ManagedBuffer::<'a, AA>::from_raw_handle(const_handles::MBUF_TEMPORARY_1);
    let mut cb_closure_args_buffer =
        ManagedArgBuffer::<'a, AA>::from_raw_handle(const_handles::CALLBACK_CLOSURE_ARGS_BUFFER);
    cb_closure_args_buffer.deserialize_overwrite(cb_closure_args_serialized);

    ManagedResultArgLoader::new(cb_closure_args_buffer.into_vec_of_buffers())
}
