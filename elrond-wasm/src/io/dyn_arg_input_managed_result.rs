use crate::{
    api::{ErrorApi, ManagedTypeApi},
    err_msg,
    types::{ManagedBuffer, ManagedVec},
    DynArgInput,
};

pub struct ManagedResultArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    data: ManagedVec<A, ManagedBuffer<A>>,
    data_len: usize,
    next_index: usize,
    api: A,
}

impl<A> ManagedResultArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    pub fn new(api: A, data: ManagedVec<A, ManagedBuffer<A>>) -> Self {
        let data_len = data.len();
        ManagedResultArgLoader {
            data,
            data_len,
            next_index: 0,
            api,
        }
    }
}

impl<A> DynArgInput for ManagedResultArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    type ItemInput = ManagedBuffer<A>;

    type ErrorApi = A;

    #[inline]
    fn error_api(&self) -> Self::ErrorApi {
        self.api.clone()
    }

    #[inline]
    fn has_next(&self) -> bool {
        self.next_index < self.data_len
    }

    fn next_arg_input(&mut self) -> ManagedBuffer<A> {
        if let Some(buffer) = self.data.get(self.next_index) {
            self.next_index += 1;
            buffer
        } else {
            self.error_api().signal_error(err_msg::ARG_WRONG_NUMBER)
        }
    }
}
