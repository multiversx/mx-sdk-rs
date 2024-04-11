use crate::codec::{DecodeError, DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    types::{ManagedBuffer, ManagedVec},
};

pub struct ManagedResultArgLoader<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi,
{
    data: ManagedVec<'a, A, ManagedBuffer<'a, A>>,
    data_len: usize,
    next_index: usize,
}

impl<'a, A> ManagedResultArgLoader<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi,
{
    pub fn new(data: ManagedVec<'a, A, ManagedBuffer<'a, A>>) -> Self {
        let data_len = data.len();
        ManagedResultArgLoader {
            data,
            data_len,
            next_index: 0,
        }
    }
}

impl<'a, A> TopDecodeMultiInput for ManagedResultArgLoader<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi,
{
    type ValueInput = ManagedBuffer<'a, A>;

    fn has_next(&self) -> bool {
        self.next_index < self.data_len
    }

    fn next_value_input<H>(&mut self, h: H) -> Result<Self::ValueInput, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if let Some(buffer) = self.data.try_get(self.next_index) {
            self.next_index += 1;
            Ok((*buffer).clone())
        } else {
            Err(h.handle_error(DecodeError::MULTI_TOO_FEW_ARGS))
        }
    }
}
