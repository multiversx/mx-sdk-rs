use core::{borrow::Borrow, marker::PhantomData};

use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{ManagedBuffer, ManagedSCError, ManagedType},
};

#[derive(Default)]
pub struct ErrorHelper<M: ManagedTypeApi> {
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ErrorHelper<M> {
    pub(crate) fn new_instance() -> Self {
        ErrorHelper {
            _phantom: PhantomData,
        }
    }

    pub fn new_error(&self) -> ManagedSCError<M> {
        ManagedSCError::new_empty()
    }

    pub fn signal_error_with_message<T>(message: T) -> !
    where
        T: IntoSignalErrorMessage<M>,
    {
        message.signal_error_with_message()
    }

    pub fn signal_error_with_buffer_handle<T>(handle: i32) -> ! {
        M::error_api_impl().signal_error_from_buffer(handle)
    }
}

pub trait IntoSignalErrorMessage<M: ManagedTypeApi> {
    fn signal_error_with_message(self) -> !;
}

impl<M: ManagedTypeApi> IntoSignalErrorMessage<M> for &str {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.as_bytes())
    }
}

impl<M: ManagedTypeApi> IntoSignalErrorMessage<M> for &[u8] {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self)
    }
}

impl<M, B> IntoSignalErrorMessage<M> for B
where
    M: ManagedTypeApi,
    B: Borrow<ManagedBuffer<M>>,
{
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error_from_buffer(self.borrow().get_raw_handle())
    }
}
