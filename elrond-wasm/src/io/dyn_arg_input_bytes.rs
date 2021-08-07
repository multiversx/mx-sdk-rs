use crate::api::ManagedTypeApi;
use crate::*;
use crate::{api::ErrorApi, types::BoxedBytes};

pub struct BytesArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    bytes: &'a [BoxedBytes],
    api: A,
}

impl<'a, A> BytesArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    pub fn new(bytes: &'a [BoxedBytes], api: A) -> Self {
        BytesArgLoader {
            bytes,
            api,
        }
    }
}

impl<'a, A> ErrorApi for BytesArgLoader<'a, A>
where
    A: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn signal_error(&self, message: &[u8]) -> ! {
        self.api.signal_error(message)
    }
}

impl<'a, A> DynArgInput<A, &'a [u8]> for BytesArgLoader<'a, A>
where
    A: ErrorApi,
{
    #[inline]
    fn has_next(&self) -> bool {
        !self.bytes.is_empty()
    }

    fn next_arg_input(&mut self) -> &'a [u8] {
        if self.bytes.is_empty() {
            self.signal_error(err_msg::ARG_WRONG_NUMBER);
        }
        let result = self.bytes[0].as_slice();
        self.bytes = &self.bytes[1..];
        result
    }
}
