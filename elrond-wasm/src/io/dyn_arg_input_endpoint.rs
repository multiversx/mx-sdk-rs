use core::marker::PhantomData;

use crate::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl, ErrorApi, ErrorApiImpl, ManagedTypeApi},
    err_msg, ArgDecodeInput, DynArgInput,
};

#[derive(Default)]
pub struct EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    _phantom: PhantomData<AA>,
    current_index: i32,
    num_arguments: i32,
}

impl<AA> EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    pub fn new() -> Self {
        let num_arguments = AA::argument_api_impl().get_num_arguments();
        EndpointDynArgLoader {
            _phantom: PhantomData,
            current_index: 0,
            num_arguments,
        }
    }
}

impl<AA> DynArgInput for EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    type ItemInput = ArgDecodeInput<AA>;

    type ManagedTypeErrorApi = AA;

    fn has_next(&self) -> bool {
        self.current_index < self.num_arguments
    }

    fn next_arg_input(&mut self) -> ArgDecodeInput<AA> {
        if self.current_index >= self.num_arguments {
            AA::error_api_impl().signal_error(err_msg::ARG_WRONG_NUMBER)
        } else {
            let arg_input = ArgDecodeInput::new(self.current_index);
            self.current_index += 1;
            arg_input
        }
    }

    fn flush_ignore(&mut self) {
        self.current_index = self.num_arguments;
    }
}
