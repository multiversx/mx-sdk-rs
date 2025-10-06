use super::{DurationSeconds, TimestampMillis};
use core::ops::{Add, Sub};

/// Represents a point in time as seconds since the Unix epoch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TimestampSeconds(pub(super) u64);

impl TimestampSeconds {
    pub fn new(seconds: u64) -> Self {
        TimestampSeconds(seconds)
    }

    pub fn as_seconds(&self) -> u64 {
        self.0
    }

    /// Explicit conversion to milliseconds
    pub fn to_millis(&self) -> TimestampMillis {
        TimestampMillis::new(self.0 * 1000)
    }
}

// TimestampSeconds + DurationSeconds = TimestampSeconds
impl Add<DurationSeconds> for TimestampSeconds {
    type Output = TimestampSeconds;

    fn add(self, rhs: DurationSeconds) -> Self::Output {
        TimestampSeconds(self.0 + rhs.0)
    }
}

// TimestampSeconds - TimestampSeconds = DurationSeconds
impl Sub<TimestampSeconds> for TimestampSeconds {
    type Output = DurationSeconds;

    fn sub(self, rhs: TimestampSeconds) -> Self::Output {
        DurationSeconds(self.0 - rhs.0)
    }
}
