use crate::{
    api::{ErrorApi, ErrorApiImpl, ManagedTypeApi, ManagedTypeErrorApi},
    err_msg,
    types::ManagedBytesTopDecodeInput,
    DynArgInput, HexCallDataDeserializer,
};

pub struct CallDataArgLoader<'a, A>
where
    A: ManagedTypeErrorApi,
{
    deser: HexCallDataDeserializer<'a>,
    api: A,
}

impl<'a, A> CallDataArgLoader<'a, A>
where
    A: ManagedTypeErrorApi,
{
    pub fn new(deser: HexCallDataDeserializer<'a>, api: A) -> Self {
        CallDataArgLoader { deser, api }
    }
}

impl<'a, A> DynArgInput for CallDataArgLoader<'a, A>
where
    A: ManagedTypeErrorApi,
{
    type ItemInput = ManagedBytesTopDecodeInput<A>;

    type ManagedTypeErrorApi = A;

    // #[inline]
    // fn dyn_arg_vm_api(&self) -> Self::ErrorApi {
    //     self.api.clone()
    // }

    #[inline]
    fn has_next(&self) -> bool {
        self.deser.has_next()
    }

    fn next_arg_input(&mut self) -> ManagedBytesTopDecodeInput<A> {
        match self.deser.next_argument() {
            Ok(Some(arg_bytes)) => ManagedBytesTopDecodeInput::<A>::new(arg_bytes.into()),
            Ok(None) => A::error_api_impl().signal_error(err_msg::ARG_WRONG_NUMBER),
            Err(sc_err) => A::error_api_impl().signal_error(sc_err.as_bytes()),
        }
    }

    // fn assert_no_more_args(&self) {
    //     if self.has_next() {
    //         A::error_api_impl().signal_error(err_msg::ARG_WRONG_NUMBER);
    //     }
    // }
}
