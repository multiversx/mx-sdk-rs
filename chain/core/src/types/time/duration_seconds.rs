use core::ops::{Add, Sub};

use super::DurationMillis;

use crate::codec::*;

/// Represents a duration in seconds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DurationSeconds(pub(crate) u64);

impl DurationSeconds {
    pub const fn new(seconds: u64) -> Self {
        DurationSeconds(seconds)
    }

    pub fn as_u64_seconds(&self) -> u64 {
        self.0
    }

    /// Explicit conversion to milliseconds
    pub fn to_millis(&self) -> DurationMillis {
        DurationMillis::new(self.0 * 1000)
    }

    pub const fn zero() -> Self {
        DurationSeconds(0)
    }
}

// DurationSeconds + DurationSeconds = DurationSeconds
impl Add<DurationSeconds> for DurationSeconds {
    type Output = DurationSeconds;

    fn add(self, rhs: DurationSeconds) -> Self::Output {
        DurationSeconds(self.0 + rhs.0)
    }
}

// DurationSeconds - DurationSeconds = DurationSeconds
impl Sub<DurationSeconds> for DurationSeconds {
    type Output = DurationSeconds;

    fn sub(self, rhs: DurationSeconds) -> Self::Output {
        DurationSeconds(self.0 - rhs.0)
    }
}

impl NestedEncode for DurationSeconds {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for DurationSeconds {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for DurationSeconds {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(DurationSeconds(u64::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for DurationSeconds {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(DurationSeconds(u64::top_decode_or_handle_err(input, h)?))
    }
}

impl core::fmt::Display for DurationSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} s", self.0)
    }
}
