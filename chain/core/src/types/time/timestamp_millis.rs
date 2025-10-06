use core::ops::{Add, Sub};

use super::DurationMillis;

/// Represents a point in time as milliseconds since the Unix epoch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TimestampMillis(pub(crate) u64);

impl TimestampMillis {
    pub fn new(millis: u64) -> Self {
        TimestampMillis(millis)
    }

    pub fn as_millis(&self) -> u64 {
        self.0
    }

    /// Explicit conversion to seconds, truncating any millisecond precision
    pub fn as_u64(&self) -> u64 {
        self.0 / 1000
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
