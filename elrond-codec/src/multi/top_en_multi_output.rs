use crate::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

pub trait TopEncodeMultiOutput {
    type ValueOutput: TopEncodeOutput;

    fn push_single_value<T, H>(&mut self, arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TopEncode,
        H: EncodeErrorHandler;
}
