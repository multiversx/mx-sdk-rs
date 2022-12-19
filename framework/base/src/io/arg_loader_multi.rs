use core::marker::PhantomData;

use crate::codec::{DecodeError, DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi, StaticVarApiImpl},
    io::ArgDecodeInput,
};

/// Does not keep the total number of arguments, it relies on it being saved statically,
/// i.e. `init_arguments_static_data` to be called before.
///
/// Only used in `ArgNestedTuple`, do not use directly.
#[derive(Default)]
pub(super) struct EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    _phantom: PhantomData<AA>,
    current_index: i32,
}

impl<AA> EndpointDynArgLoader<AA>
where
    AA: ManagedTypeApi + ErrorApi + EndpointArgumentApi,
{
    pub fn new_at_index(current_index: i32) -> Self {
        EndpointDynArgLoader {
            _phantom: PhantomData,
            current_index,
        }
    }

    fn num_arguments() -> i32 {
        AA::static_var_api_impl().get_num_arguments()
    }

    fn check_current_index<H>(&self, h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if self.current_index < Self::num_arguments() {
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
        self.current_index < Self::num_arguments()
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
        self.current_index = Self::num_arguments();
        Ok(())
    }
}
