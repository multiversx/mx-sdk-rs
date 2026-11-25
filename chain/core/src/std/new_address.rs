use crate::{std::Bech32Address, types::Address};

use super::crypto::keccak256;

/// A 2-byte VM type identifier embedded in every smart-contract address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VMType(pub [u8; 2]);

impl VMType {
    /// The default MultiversX VM type: `[5, 0]`.
    pub const DEFAULT: VMType = VMType([5, 0]);

    /// Returns the raw bytes.
    pub fn as_bytes(&self) -> &[u8; 2] {
        &self.0
    }
}

impl From<[u8; 2]> for VMType {
    fn from(bytes: [u8; 2]) -> Self {
        VMType(bytes)
    }
}

/// Deterministically compute the deployed contract address from the deployer address, nonce,
/// and an explicit VM type.
///
/// `address = 8 bytes zero || 2 bytes vm_type || keccak256(owner ++ nonce)[10..30] || owner[30..32]`
pub fn compute_new_address_with_vm_type(
    deployer: &Address,
    deployment_nonce: u64,
    vm_type: VMType,
) -> Address {
    let owner_bytes = deployer.as_bytes();
    let nonce_bytes = deployment_nonce.to_le_bytes();
    let bytes_to_hash = [owner_bytes, &nonce_bytes].concat();
    let hash = keccak256(&bytes_to_hash);

    let mut address_bytes = [0u8; 32];
    // bytes [0..8] stay zero (initial padding)
    address_bytes[8..10].copy_from_slice(vm_type.as_bytes()); // vm_type
    address_bytes[10..30].copy_from_slice(&hash[10..30]);
    address_bytes[30..32].copy_from_slice(&owner_bytes[30..32]);

    Address::from(address_bytes)
}

/// Deterministically compute the deployed contract address from the deployer address and nonce.
///
/// Uses the default MultiversX WASM VM type (`[5, 0]`).
pub fn compute_new_address(deployer: &Address, deployment_nonce: u64) -> Address {
    compute_new_address_with_vm_type(deployer, deployment_nonce, VMType::DEFAULT)
}

/// Deterministically compute the deployed contract address, preserving the deployer's HRP.
pub fn compute_new_address_bech32(
    deployer: &Bech32Address,
    deployment_nonce: u64,
) -> Bech32Address {
    let address = compute_new_address(&deployer.address, deployment_nonce);
    Bech32Address::encode_address(deployer.hrp, address)
}

/// Generates a mock smart contract address deterministically from a creator address and nonce.
/// Used in testing environments to simulate address generation without a real VM.
pub fn generate_mock_address(creator_address: &[u8], creator_nonce: u64) -> Address {
    let mut result = [0x00u8; 32];

    result[10] = 0x11;
    result[11] = 0x11;
    result[12] = 0x11;
    result[13] = 0x11;

    result[14..29].copy_from_slice(&creator_address[..15]);
    result[29] = creator_nonce as u8;
    result[30..].copy_from_slice(&creator_address[30..]);

    result[8..10].copy_from_slice(VMType::DEFAULT.as_bytes());

    Address::from(result)
}
