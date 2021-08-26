use crate::api::{ErrorApi, ManagedTypeApi};
use crate::types::ManagedBytesTopDecodeInput;
use crate::{err_msg, DynArgInput, HexCallDataDeserializer};

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

impl<'a, A> ErrorApi for CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn signal_error(&self, message: &[u8]) -> ! {
        self.api.signal_error(message)
    }
}

impl<'a, A> DynArgInput<ManagedBytesTopDecodeInput<A>> for CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn has_next(&self) -> bool {
        self.deser.has_next()
    }

    fn next_arg_input(&mut self) -> ManagedBytesTopDecodeInput<A> {
        match self.deser.next_argument() {
            Ok(Some(arg_bytes)) => {
                ManagedBytesTopDecodeInput::new(arg_bytes.into(), self.api.clone())
            },
            Ok(None) => self.signal_error(err_msg::ARG_WRONG_NUMBER),
            Err(sc_err) => self.signal_error(sc_err.as_bytes()),
        }
    }
}
