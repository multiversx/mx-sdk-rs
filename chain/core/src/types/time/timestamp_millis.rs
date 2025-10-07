use core::ops::{Add, Sub};

use super::DurationMillis;

use crate::{codec::*, types::TimestampSeconds};

/// Represents a point in time as milliseconds since the Unix epoch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TimestampMillis(pub(crate) u64);

impl TimestampMillis {
    pub const fn new(millis: u64) -> Self {
        TimestampMillis(millis)
    }

    pub fn as_u64_millis(&self) -> u64 {
        self.0
    }

    /// Explicit conversion to seconds, truncating any millisecond precision
    pub fn to_seconds(&self) -> TimestampSeconds {
        TimestampSeconds(self.0 / 1000)
    }

    pub const fn zero() -> Self {
        TimestampMillis(0)
    }

    pub const fn max() -> Self {
        TimestampMillis(u64::MAX)
    }
}

// TimestampMillis - TimestampMillis = DurationMillis
impl Sub<TimestampMillis> for TimestampMillis {
    type Output = DurationMillis;

    fn sub(self, rhs: TimestampMillis) -> Self::Output {
        DurationMillis(self.0 - rhs.0)
    }
}

// TimestampMillis + DurationMillis = TimestampMillis
impl Add<DurationMillis> for TimestampMillis {
    type Output = TimestampMillis;

    fn add(self, rhs: DurationMillis) -> Self::Output {
        TimestampMillis(self.0 + rhs.0)
    }
}

// TimestampMillis - DurationMillis = TimestampMillis
impl Sub<DurationMillis> for TimestampMillis {
    type Output = TimestampMillis;

    fn sub(self, rhs: DurationMillis) -> Self::Output {
        TimestampMillis(self.0 - rhs.0)
    }
}

impl NestedEncode for TimestampMillis {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for TimestampMillis {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for TimestampMillis {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(TimestampMillis(u64::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for TimestampMillis {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(TimestampMillis(u64::top_decode_or_handle_err(input, h)?))
    }
}

impl core::fmt::Display for TimestampMillis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ms", self.0)
    }
}
