use core::marker::PhantomData;

use elrond_codec::{DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi},
    io::ArgDecodeInput,
};

#[derive(Default)]
pub struct EndpointSingleArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    _phantom: PhantomData<AA>,
    current_index: i32,
}

impl<AA> EndpointSingleArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    pub fn new(index: i32) -> Self {
        EndpointSingleArgLoader {
            _phantom: PhantomData,
            current_index: index,
        }
    }
}

impl<AA> TopDecodeMultiInput for EndpointSingleArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    type ValueInput = ArgDecodeInput<AA>;

    fn has_next(&self) -> bool {
        false
    }

    fn next_value_input<H>(&mut self, _h: H) -> Result<Self::ValueInput, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let arg_input = ArgDecodeInput::new(self.current_index);
        Ok(arg_input)
    }

    fn flush_ignore<H>(&mut self, _h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        Ok(())
    }
}
