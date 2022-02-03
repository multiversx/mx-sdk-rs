use crate::DecodeError;

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
