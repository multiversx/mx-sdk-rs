use crate::{DecodeError, EncodeError};

pub trait DecodeErrorHandler: Copy {
    type HandledErr: 'static;

    fn handle_error(&self, err: DecodeError) -> Self::HandledErr;
}

#[derive(Clone, Copy)]
pub struct DefaultDecodeErrorHandler;

impl DecodeErrorHandler for DefaultDecodeErrorHandler {
    type HandledErr = DecodeError;

    #[inline]
    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        err
    }
}

#[derive(Clone, Copy)]
pub struct PanicDecodeErrorHandler;

impl DecodeErrorHandler for PanicDecodeErrorHandler {
    type HandledErr = !;

    #[inline]
    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        panic!("Decode error occured: {}", err.message_str())
    }
}

pub trait EncodeErrorHandler: Copy {
    type HandledErr: 'static;

    fn handle_error(&self, err: EncodeError) -> Self::HandledErr;
}

#[derive(Clone, Copy)]
pub struct DefaultEncodeErrorHandler;

impl EncodeErrorHandler for DefaultEncodeErrorHandler {
    type HandledErr = EncodeError;

    #[inline]
    fn handle_error(&self, err: EncodeError) -> Self::HandledErr {
        err
    }
}

#[derive(Clone, Copy)]
pub struct PanicEncodeErrorHandler;

impl EncodeErrorHandler for PanicEncodeErrorHandler {
    type HandledErr = !;

    #[inline]
    fn handle_error(&self, err: EncodeError) -> Self::HandledErr {
        panic!("Encode error occured: {}", err.message_str())
    }
}
