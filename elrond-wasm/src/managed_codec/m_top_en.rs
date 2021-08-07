// use crate::codec_err::EncodeError;
// use crate::nested_ser::NestedEncode;
// use crate::top_ser_output::TopEncodeOutput;
// use crate::TypeInfo;
// use alloc::vec::Vec;
use elrond_codec::{EncodeError, TopEncode};

use crate::api::ManagedTypeApi;

use super::{ManagedTopEncodeOutput, TopEncodeOutputAdapter};

pub trait ManagedTopEncode<M: ManagedTypeApi>: Sized {
    fn top_encode_or_exit<O: ManagedTopEncodeOutput<M>, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    );
}

impl<M, T> ManagedTopEncode<M> for T
where
    M: ManagedTypeApi,
    T: TopEncode,
{
    fn top_encode_or_exit<O: ManagedTopEncodeOutput<M>, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        <T as TopEncode>::top_encode_or_exit(self, TopEncodeOutputAdapter::new(output), c, exit);
    }
}
