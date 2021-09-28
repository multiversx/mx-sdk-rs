use crate::{
    api::{ErrorApi, ManagedTypeApi},
    err_msg,
    types::ManagedBytesTopDecodeInput,
    DynArgInput, HexCallDataDeserializer,
};

pub struct CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    deser: HexCallDataDeserializer<'a>,
    api: A,
}

impl<'a, A> CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    pub fn new(deser: HexCallDataDeserializer<'a>, api: A) -> Self {
        CallDataArgLoader { deser, api }
    }
}

impl<'a, A> DynArgInput for CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    type ItemInput = ManagedBytesTopDecodeInput<A>;

    type ErrorApi = A;

    #[inline]
    fn dyn_arg_vm_api(&self) -> Self::ErrorApi {
        self.api.clone()
    }

    #[inline]
    fn has_next(&self) -> bool {
        self.deser.has_next()
    }

    fn next_arg_input(&mut self) -> ManagedBytesTopDecodeInput<A> {
        match self.deser.next_argument() {
            Ok(Some(arg_bytes)) => {
                ManagedBytesTopDecodeInput::new(self.api.clone(), arg_bytes.into())
            },
            Ok(None) => self
                .dyn_arg_vm_api()
                .signal_error(err_msg::ARG_WRONG_NUMBER),
            Err(sc_err) => self.dyn_arg_vm_api().signal_error(sc_err.as_bytes()),
        }
    }
}
