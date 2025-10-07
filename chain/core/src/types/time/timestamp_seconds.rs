use super::{DurationSeconds, TimestampMillis};
use core::ops::{Add, Sub};

use crate::codec::*;

/// Represents a point in time as seconds since the Unix epoch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TimestampSeconds(pub(crate) u64);

impl TimestampSeconds {
    pub const fn new(seconds: u64) -> Self {
        TimestampSeconds(seconds)
    }

    pub fn as_u64_seconds(&self) -> u64 {
        self.0
    }

    /// Explicit conversion to milliseconds
    pub fn to_millis(&self) -> TimestampMillis {
        TimestampMillis::new(self.0 * 1000)
    }

    pub const fn zero() -> Self {
        TimestampSeconds(0)
    }

    pub const fn max() -> Self {
        TimestampSeconds(u64::MAX)
    }
}

// TimestampSeconds - TimestampSeconds = DurationSeconds
impl Sub<TimestampSeconds> for TimestampSeconds {
    type Output = DurationSeconds;

    fn sub(self, rhs: TimestampSeconds) -> Self::Output {
        DurationSeconds(self.0 - rhs.0)
    }
}

// TimestampSeconds + DurationSeconds = TimestampSeconds
impl Add<DurationSeconds> for TimestampSeconds {
    type Output = TimestampSeconds;

    fn add(self, rhs: DurationSeconds) -> Self::Output {
        TimestampSeconds(self.0 + rhs.0)
    }
}

// TimestampSeconds - DurationSeconds = TimestampSeconds
impl Sub<DurationSeconds> for TimestampSeconds {
    type Output = TimestampSeconds;

    fn sub(self, rhs: DurationSeconds) -> Self::Output {
        TimestampSeconds(self.0 - rhs.0)
    }
}

impl NestedEncode for TimestampSeconds {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for TimestampSeconds {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for TimestampSeconds {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(TimestampSeconds(u64::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for TimestampSeconds {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(TimestampSeconds(u64::top_decode_or_handle_err(input, h)?))
    }
}

impl core::fmt::Display for TimestampSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} s", self.0)
    }
}
