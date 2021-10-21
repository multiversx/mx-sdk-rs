use crate::{
    api::{ErrorApi, ManagedTypeApi},
    err_msg,
    types::{ManagedBuffer, ManagedType, ManagedVec},
    DynArgInput,
};

pub struct ManagedResultArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    data: ManagedVec<A, ManagedBuffer<A>>,
    data_len: usize,
    next_index: usize,
}

impl<A> ManagedResultArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    pub fn new(data: ManagedVec<A, ManagedBuffer<A>>) -> Self {
        let data_len = data.len();
        ManagedResultArgLoader {
            data,
            data_len,
            next_index: 0,
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
    fn dyn_arg_vm_api(&self) -> Self::ErrorApi {
        self.data.type_manager()
    }

    #[inline]
    fn has_next(&self) -> bool {
        self.next_index < self.data_len
    }

    fn next_arg_input(&mut self) -> Self::ItemInput {
        if let Some(buffer) = self.data.get(self.next_index) {
            self.next_index += 1;
            buffer
        } else {
            self.dyn_arg_vm_api()
                .signal_error(err_msg::ARG_WRONG_NUMBER)
        }
    }
}
