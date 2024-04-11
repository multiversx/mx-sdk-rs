use core::{borrow::Borrow, marker::PhantomData};

use crate::codec::{DecodeError, EncodeError};

use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{heap::BoxedBytes, ManagedBuffer, ManagedSCError, ManagedType},
};

#[derive(Default)]
pub struct ErrorHelper<'a, M: ManagedTypeApi<'a>> {
    _phantom: PhantomData<M>,
}

impl<'a, M: ManagedTypeApi<'a>> ErrorHelper<'a, M> {
    pub fn new() -> Self {
        ErrorHelper {
            _phantom: PhantomData,
        }
    }

    pub fn new_error(&self) -> ManagedSCError<'a, M> {
        ManagedSCError::new_empty()
    }

    pub fn signal_error_with_message<T>(message: T) -> !
    where
        T: IntoSignalError<'a, M>,
    {
        message.signal_error_with_message()
    }
}

/// Indicates how an object can be used as the basis for performing `signal_error` with itself as message.
pub trait IntoSignalError<'a, M: ManagedTypeApi<'a>> {
    fn signal_error_with_message(self) -> !;
}

impl<'a, M: ManagedTypeApi<'a>> IntoSignalError<'a, M> for &str {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.as_bytes())
    }
}

impl<'a, M: ManagedTypeApi<'a>> IntoSignalError<'a, M> for &[u8] {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self)
    }
}

impl<'a, M: ManagedTypeApi<'a>> IntoSignalError<'a, M> for BoxedBytes {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.as_slice())
    }
}

impl<'a, M: ManagedTypeApi<'a>> IntoSignalError<'a, M> for EncodeError {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.message_bytes())
    }
}

impl<'a, M: ManagedTypeApi<'a>> IntoSignalError<'a, M> for DecodeError {
    #[inline]
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error(self.message_bytes())
    }
}

// Handles `ManagedBuffer`, `&ManagedBuffer` and `ManagedRef<ManagedBuffer>`.
impl<'a, M, B> IntoSignalError<'a, M> for B
where
    M: ManagedTypeApi<'a>,
    B: Borrow<ManagedBuffer<'a, M>>,
{
    fn signal_error_with_message(self) -> ! {
        M::error_api_impl().signal_error_from_buffer(self.borrow().get_handle())
    }
}
