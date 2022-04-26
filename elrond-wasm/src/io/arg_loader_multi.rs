use core::marker::PhantomData;

use elrond_codec::{DecodeError, DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl, ErrorApi, ManagedTypeApi},
    io::ArgDecodeInput,
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
    pub fn new_at_index(current_index: i32) -> Self {
        let num_arguments = AA::argument_api_impl().get_num_arguments();
        EndpointDynArgLoader {
            _phantom: PhantomData,
            current_index,
            num_arguments,
        }
    }

    /// For backwards compatibility. TODO: remove.
    pub fn new() -> Self {
        Self::new_at_index(0)
    }

    fn check_current_index<H>(&self, h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if self.current_index < self.num_arguments {
            Ok(())
        } else {
            Err(h.handle_error(DecodeError::MULTI_TOO_FEW_ARGS))
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
        self.check_current_index(h)?;

        let arg_input = ArgDecodeInput::new(self.current_index);
        self.current_index += 1;
        Ok(arg_input)
    }

    fn flush_ignore<H>(&mut self, _h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.current_index = self.num_arguments;
        Ok(())
    }
}
