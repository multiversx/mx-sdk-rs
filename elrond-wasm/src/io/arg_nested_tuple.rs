use elrond_codec::{DecodeError, TopDecodeMulti, TopDecodeMultiInput};

use crate::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl, ErrorApi, ErrorApiImpl, ManagedTypeApi},
    contract_base::ExitCodecErrorHandler,
    ArgErrorHandler, ArgId, EndpointDynArgLoader, EndpointSingleArgLoader,
};

pub trait ArgNestedTuple<AA>
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
{
    type ArgNames;

    fn check_num_single_args(index: i32);
    fn load_single_arg(index: i32, arg_names: Self::ArgNames) -> Self;
    fn load_multi_arg(loader: EndpointDynArgLoader<AA>, arg_names: Self::ArgNames) -> Self;
}

impl<AA, Head, Tail> ArgNestedTuple<AA> for (Head, Tail)
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
    Head: TopDecodeMulti,
    Tail: ArgNestedTuple<AA>,
{
    type ArgNames = (&'static str, Tail::ArgNames);

    fn check_num_single_args(index: i32) {
        if Head::IS_SINGLE_VALUE {
            Tail::check_num_single_args(index + 1);
        } else {
            let num_args = AA::argument_api_impl().get_num_arguments();
            if num_args < index {
                AA::error_api_impl().signal_error(DecodeError::MULTI_TOO_FEW_ARGS.message_bytes());
            }
        }
    }

    fn load_single_arg(index: i32, arg_names: Self::ArgNames) -> Self {
        if Head::IS_SINGLE_VALUE {
            let (arg_name, tail_names) = arg_names;
            let mut arg_loader = EndpointSingleArgLoader::<AA>::new(index);
            let h = ArgErrorHandler::<AA>::from(ArgId::from(arg_name));
            let Ok(value) = Head::multi_decode_or_handle_err(&mut arg_loader, h);
            (value, Tail::load_single_arg(index + 1, tail_names))
        } else {
            let loader = EndpointDynArgLoader::new_at_index(index);
            Self::load_multi_arg(loader, arg_names)
        }
    }
    fn load_multi_arg(mut loader: EndpointDynArgLoader<AA>, arg_names: Self::ArgNames) -> Self {
        let (arg_name, tail_names) = arg_names;
        let h = ArgErrorHandler::<AA>::from(ArgId::from(arg_name));
        let result = Head::multi_decode_or_handle_err(&mut loader, h);
        let Ok(value) = result;
        (value, Tail::load_multi_arg(loader, tail_names))
    }
}

impl<AA> ArgNestedTuple<AA> for ()
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
{
    type ArgNames = ();

    fn check_num_single_args(index: i32) {
        AA::argument_api_impl().check_num_arguments(index);
    }
    fn load_single_arg(_index: i32, _arg_names: Self::ArgNames) -> Self {}
    fn load_multi_arg(loader: EndpointDynArgLoader<AA>, _arg_names: Self::ArgNames) -> Self {
        let h = ExitCodecErrorHandler::<AA>::from(&[][..]);
        let Ok(()) = loader.assert_no_more_args(h);
    }
}

pub fn load_endpoint_args<AA, N>(arg_names: N::ArgNames) -> N
where
    AA: EndpointArgumentApi + ManagedTypeApi + ErrorApi,
    N: ArgNestedTuple<AA>,
{
    N::check_num_single_args(0);
    N::load_single_arg(0, arg_names)
}
