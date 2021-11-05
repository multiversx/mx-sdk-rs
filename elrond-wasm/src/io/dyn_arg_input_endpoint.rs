use crate::{
    api::{EndpointArgumentApi, ManagedTypeApi},
    err_msg, ArgDecodeInput, DynArgInput,
};

pub struct EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    api: AA,
    current_index: i32,
    num_arguments: i32,
}

impl<AA> EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    pub fn new(api: AA) -> Self {
        let num_arguments = api.get_num_arguments();
        EndpointDynArgLoader {
            api,
            current_index: 0,
            num_arguments,
        }
    }
}

impl<AA> DynArgInput for EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    type ItemInput = ArgDecodeInput<AA>;

    type ErrorApi = AA;

    #[inline]
    fn dyn_arg_vm_api(&self) -> Self::ErrorApi {
        self.api.clone()
    }

    fn has_next(&self) -> bool {
        self.current_index < self.num_arguments
    }

    fn next_arg_input(&mut self) -> ArgDecodeInput<AA> {
        if self.current_index >= self.num_arguments {
            self.dyn_arg_vm_api()
                .signal_error(err_msg::ARG_WRONG_NUMBER)
        } else {
            let arg_input = ArgDecodeInput::new(self.api.clone(), self.current_index);
            self.current_index += 1;
            arg_input
        }
    }

    fn flush_ignore(&mut self) {
        self.current_index = self.num_arguments;
    }
}
