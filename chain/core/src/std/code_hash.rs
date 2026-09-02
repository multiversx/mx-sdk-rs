use blake2::{Blake2b, Digest, digest::consts::U32};

pub const CODE_HASH_LEN: usize = 32;

/// Computes the code hash of a smart contract.
///
/// Uses Blake2b with a 256-bit (32-byte) digest.
/// This is the standard way to identify a contract's code on the MultiversX blockchain.
pub fn code_hash(code: &[u8]) -> [u8; CODE_HASH_LEN] {
    Blake2b::<U32>::digest(code).into()
}
