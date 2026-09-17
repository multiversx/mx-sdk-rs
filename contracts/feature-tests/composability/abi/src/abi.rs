#![no_std]

//! Pure ABI descriptions of the contracts under `contracts/feature-tests/composability`, with no
//! dependency on their Rust crates — for testing that contracts can call each other purely by
//! ABI (e.g. `forwarder` calling `vault` this way, see `fwd_call_sync_abi.rs`).

pub mod vault_abi;
