use core::marker::PhantomData;

use elrond_codec::{DecodeError, DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl, ErrorApi, ManagedTypeApi},
    err_msg, ArgDecodeInput,
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

impl<AA> TopDecodeMultiInput for EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    type ValueInput = ArgDecodeInput<AA>;

    fn has_next(&self) -> bool {
        self.current_index < self.num_arguments
    }

    fn next_value_input<H>(&mut self, h: H) -> Result<Self::ValueInput, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if self.current_index >= self.num_arguments {
            Err(h.handle_error(DecodeError::from(err_msg::ARG_WRONG_NUMBER)))
        } else {
            let arg_input = ArgDecodeInput::new(self.current_index);
            self.current_index += 1;
            Ok(arg_input)
        }
    }

    fn flush_ignore<H>(&mut self, _h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.current_index = self.num_arguments;
        Ok(())
    }
}
