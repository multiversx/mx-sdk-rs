use crate::api::{ErrorApi, ManagedTypeApi};
use crate::hex_call_data::*;
use crate::*;

pub struct CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    deser: HexCallDataDeserializer<'a>,
    signal_error: A,
}

impl<'a, A> CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    pub fn new(deser: HexCallDataDeserializer<'a>, signal_error: A) -> Self {
        CallDataArgLoader {
            deser,
            signal_error,
        }
    }
}

impl<'a, A> ErrorApi for CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn signal_error(&self, message: &[u8]) -> ! {
        self.signal_error.signal_error(message)
    }
}

impl<'a, A> DynArgInput<A, Vec<u8>> for CallDataArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn has_next(&self) -> bool {
        self.deser.has_next()
    }

    fn next_arg_input(&mut self) -> Vec<u8> {
        match self.deser.next_argument() {
            Ok(Some(arg_bytes)) => arg_bytes,
            Ok(None) => self.signal_error(err_msg::ARG_WRONG_NUMBER),
            Err(sc_err) => self.signal_error(sc_err.as_bytes()),
        }
    }
}
