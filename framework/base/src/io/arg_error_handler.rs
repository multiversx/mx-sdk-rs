use core::marker::PhantomData;

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    codec::*,
    io::{signal_arg_de_error, ArgId},
};

#[derive(Clone)]
pub struct ArgErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    _phantom: PhantomData<M>,
    pub arg_id: ArgId,
}

impl<M> Copy for ArgErrorHandler<M> where M: ManagedTypeApi + ErrorApi {}

impl<M> From<ArgId> for ArgErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    fn from(arg_id: ArgId) -> Self {
        ArgErrorHandler {
            _phantom: PhantomData,
            arg_id,
        }
    }
}

impl<M> DecodeErrorHandler for ArgErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    type HandledErr = !;

    #[inline(always)]
    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        signal_arg_de_error::<M>(self.arg_id, err)
    }
}
