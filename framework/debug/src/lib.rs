#![allow(clippy::type_complexity)]
#![feature(exhaustive_patterns)]

pub mod api;
pub mod bech32;
mod display_util;
mod managed_test_util;
pub mod mandos_system;
pub mod testing_framework;
pub mod tx_execution;
pub mod tx_mock;
pub mod world_mock;

pub use crate::mandos_system::{executor::*, scenario_go, scenario_rs};
pub use display_util::*;
pub use managed_test_util::*;

pub use tx_mock::DebugApi;
pub use world_mock::BlockchainMock;

// Re-exporting the whole mandos crate for easier use in tests.
pub use mx_chain_scenario_format as scenario_format;

// Re-exporting for convenience. Using the crate as imported in the codec to make sure the save version is used everywhere.
pub use mx_sc::codec::num_bigint;

#[macro_use]
extern crate alloc;
pub use alloc::{boxed::Box, vec::Vec};

pub use mx_sc;

pub use std::collections::HashMap;
