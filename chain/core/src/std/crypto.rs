//! Standard crypto functions and constants used across many MultiversX crates.

pub mod ed25519;
mod keccak256;
mod sha256;

pub use keccak256::{KECCAK256_RESULT_LEN, keccak256};
pub use sha256::{SHA256_RESULT_LEN, sha256};
