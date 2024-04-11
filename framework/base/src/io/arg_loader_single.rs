use core::marker::PhantomData;

use crate::codec::{DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi},
    io::ArgDecodeInput,
};

/// Loads a single-value argument. Behaves as if only the argument at `current_index` exists, nothing after.
///
/// Only used in `ArgNestedTuple`, do not use directly.
#[derive(Default)]
pub(super) struct EndpointSingleArgLoader<'a, AA>
where
    AA: ManagedTypeApi<'a> + ErrorApi + EndpointArgumentApi<'a>,
{
    _phantom: PhantomData<AA>,
    current_index: i32,
}

impl<'a, AA> EndpointSingleArgLoader<'a, AA>
where
    AA: ManagedTypeApi<'a> + ErrorApi + EndpointArgumentApi<'a>,
{
    pub fn new(index: i32) -> Self {
        EndpointSingleArgLoader {
            _phantom: PhantomData,
            current_index: index,
        }
    }
}

impl<'a, AA> TopDecodeMultiInput for EndpointSingleArgLoader<'a, AA>
where
    AA: ManagedTypeApi<'a> + ErrorApi + EndpointArgumentApi<'a>,
{
    type ValueInput = ArgDecodeInput<'a, AA>;

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
