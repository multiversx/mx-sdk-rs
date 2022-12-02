use crate::{DecodeError, EncodeError};

pub trait EncodeErrorHandler: Copy {
    type HandledErr: 'static;

    fn handle_error(&self, err: EncodeError) -> Self::HandledErr;
}

pub trait DecodeErrorHandler: Copy {
    type HandledErr: 'static;

    fn handle_error(&self, err: DecodeError) -> Self::HandledErr;
}

impl EncodeErrorHandler for DefaultErrorHandler {
    type HandledErr = EncodeError;

    #[inline]
    fn handle_error(&self, err: EncodeError) -> Self::HandledErr {
        err
    }
}

/// The simplest error handler, it simply passes the error on.
#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl DecodeErrorHandler for DefaultErrorHandler {
    type HandledErr = DecodeError;

    #[inline]
    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        err
    }
}

/// An error handler that panics immediately, instead of returning a `Result`.
#[derive(Clone, Copy)]
pub struct PanicErrorHandler;

impl EncodeErrorHandler for PanicErrorHandler {
    type HandledErr = !;

    #[inline]
    fn handle_error(&self, err: EncodeError) -> Self::HandledErr {
        panic!("Encode error occured: {}", err.message_str())
    }
}

impl DecodeErrorHandler for PanicErrorHandler {
    type HandledErr = !;

    #[inline]
    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        panic!("Decode error occured: {}", err.message_str())
    }
}
