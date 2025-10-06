use core::ops::{Add, Sub};

use super::DurationMillis;

/// Represents a duration in seconds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DurationSeconds(pub(crate) u64);

impl DurationSeconds {
    pub fn new(seconds: u64) -> Self {
        DurationSeconds(seconds)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Explicit conversion to milliseconds
    pub fn to_millis(&self) -> DurationMillis {
        DurationMillis::new(self.0 * 1000)
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
