use crate::codec::{self, DecodeError, EncodeError, TopEncodeMulti, TryStaticCast};

use crate::api::{EndpointFinishApi, ErrorApiImpl};

use super::SCError;

/// Contains a smart contract execution error message.
///
/// The simplest implementation: a static byte slice.
/// Should be enough for most scenarios.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticSCError(&'static [u8]);

impl SCError for StaticSCError {
    fn finish_err<FA: EndpointFinishApi>(&self) -> ! {
        FA::error_api_impl().signal_error(self.0)
    }
}

impl TryStaticCast for StaticSCError {}

impl StaticSCError {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }
}

impl From<&'static [u8]> for StaticSCError {
    #[inline]
    fn from(byte_slice: &'static [u8]) -> Self {
        StaticSCError(byte_slice)
    }
}

impl From<&'static str> for StaticSCError {
    #[inline]
    fn from(s: &'static str) -> Self {
        StaticSCError(s.as_bytes())
    }
}

impl From<EncodeError> for StaticSCError {
    #[inline]
    fn from(err: EncodeError) -> Self {
        StaticSCError(err.message_bytes())
    }
}

impl From<DecodeError> for StaticSCError {
    #[inline]
    fn from(err: DecodeError) -> Self {
        StaticSCError(err.message_bytes())
    }
}

impl From<!> for StaticSCError {
    fn from(_: !) -> Self {
        unreachable!()
    }
}

impl TopEncodeMulti for StaticSCError {
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: codec::TopEncodeMultiOutput,
        H: codec::EncodeErrorHandler,
    {
        output.push_multi_specialized(self, h)
    }
}
