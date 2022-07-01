#![no_std]
#![feature(generic_associated_types)]

pub mod default_issue_callbacks;
pub mod dns;
pub mod esdt;
pub mod features;
pub mod pause;
pub mod staking;
pub mod token_merge;
pub mod transfer_role_proxy;

// TODO: remove alloc feature from the following, after they have been cleaned

pub mod bonding_curve;

pub mod governance;

pub mod users;
