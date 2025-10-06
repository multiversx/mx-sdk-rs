use core::ops::{Add, Sub};

use crate::types::DurationSeconds;

/// Represents a duration in milliseconds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DurationMillis(pub(crate) u64);

impl DurationMillis {
    pub fn new(millis: u64) -> Self {
        DurationMillis(millis)
    }

    #[inline]
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Explicit conversion to seconds, truncating any millisecond precision
    pub fn to_seconds(&self) -> DurationSeconds {
        DurationSeconds(self.0 / 1000)
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
