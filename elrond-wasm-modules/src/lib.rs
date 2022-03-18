#![no_std]

pub mod default_issue_callbacks;
pub mod dns;
pub mod esdt;
pub mod features;
pub mod pause;

// TODO: remove alloc feature from the following, after they have been cleaned

#[cfg(feature = "alloc")]
pub mod bonding_curve;

#[cfg(feature = "alloc")]
pub mod governance;

#[cfg(feature = "alloc")]
pub mod users;
