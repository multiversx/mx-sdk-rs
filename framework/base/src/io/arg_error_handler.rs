use core::marker::PhantomData;

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    codec::*,
    io::{signal_arg_de_error, ArgId},
};

#[derive(Clone)]
pub struct ArgErrorHandler<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    _phantom: PhantomData<M>,
    pub arg_id: ArgId,
}

impl<'a, M> Copy for ArgErrorHandler<'a, M> where M: ManagedTypeApi<'a> + ErrorApi {}

impl<'a, M> From<ArgId> for ArgErrorHandler<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    fn from(arg_id: ArgId) -> Self {
        ArgErrorHandler {
            _phantom: PhantomData,
            arg_id,
        }
    }
}

impl<'a, M> DecodeErrorHandler for ArgErrorHandler<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    type HandledErr = !;

    #[inline(always)]
    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        signal_arg_de_error::<'a, M>(self.arg_id, err)
    }
}
