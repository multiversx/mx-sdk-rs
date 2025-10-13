use core::ops::{Add, Sub};

use crate::types::DurationSeconds;

use crate::codec::*;

/// Represents a duration in milliseconds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DurationMillis(pub(crate) u64);

impl DurationMillis {
    pub const fn new(millis: u64) -> Self {
        DurationMillis(millis)
    }

    #[inline]
    pub fn as_u64_millis(self) -> u64 {
        self.0
    }

    /// Explicit conversion to seconds, truncating any millisecond precision
    pub fn to_seconds(&self) -> DurationSeconds {
        DurationSeconds(self.0 / 1000)
    }

    pub const fn zero() -> Self {
        DurationMillis(0)
    }
}

// DurationMillis + DurationMillis = DurationMillis
impl Add<DurationMillis> for DurationMillis {
    type Output = DurationMillis;

    fn add(self, rhs: DurationMillis) -> Self::Output {
        DurationMillis(self.0 + rhs.0)
    }
}

// DurationMillis - DurationMillis = DurationMillis
impl Sub<DurationMillis> for DurationMillis {
    type Output = DurationMillis;

    fn sub(self, rhs: DurationMillis) -> Self::Output {
        DurationMillis(self.0 - rhs.0)
    }
}

impl NestedEncode for DurationMillis {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for DurationMillis {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for DurationMillis {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(DurationMillis(u64::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for DurationMillis {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(DurationMillis(u64::top_decode_or_handle_err(input, h)?))
    }
}

impl core::fmt::Display for DurationMillis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ms", self.0)
    }
}
