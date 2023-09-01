use super::H256;

use core::fmt::Debug;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;

/// Address type being used in the VM only.
///
/// Its implementation is similar to that of the heap Address in the framework,
/// but we have a separate implementation for the VM, because it is a separate component.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct VMAddress(H256);

impl VMAddress {
    pub const fn new(bytes: [u8; 32]) -> Self {
        VMAddress(H256::new(bytes))
    }
}

impl From<H256> for VMAddress {
    fn from(hash: H256) -> Self {
        VMAddress(hash)
    }
}

impl From<VMAddress> for H256 {
    fn from(address: VMAddress) -> Self {
        address.0
    }
}

impl<'a> From<&'a VMAddress> for &'a H256 {
    fn from(address: &'a VMAddress) -> Self {
        &address.0
    }
}

impl From<[u8; 32]> for VMAddress {
    fn from(arr: [u8; 32]) -> Self {
        VMAddress(H256::from(arr))
    }
}

impl From<&[u8; 32]> for VMAddress {
    fn from(bytes: &[u8; 32]) -> Self {
        VMAddress(H256::from(bytes))
    }
}

impl From<&mut [u8; 32]> for VMAddress {
    fn from(bytes: &mut [u8; 32]) -> Self {
        VMAddress(H256::from(bytes))
    }
}

impl From<Box<[u8; 32]>> for VMAddress {
    fn from(bytes: Box<[u8; 32]>) -> Self {
        VMAddress(H256::from(bytes))
    }
}

impl VMAddress {
    pub fn from_slice(slice: &[u8]) -> Self {
        VMAddress(H256::from_slice(slice))
    }
}

impl From<VMAddress> for [u8; 32] {
    fn from(addr: VMAddress) -> Self {
        addr.0.into()
    }
}

impl AsRef<[u8]> for VMAddress {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for VMAddress {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl VMAddress {
    /// Returns a new address of 32 zeros.
    /// Allocates directly in heap.
    /// Minimal resulting wasm code (14 bytes if not inlined).
    pub fn zero() -> Self {
        VMAddress(H256::zero())
    }

    /// Extracts a byte slice containing the entire fixed hash.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn as_array(&self) -> &[u8; 32] {
        self.0.as_array()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn is_smart_contract_address(&self) -> bool {
        self.as_bytes()
            .iter()
            .take(SC_ADDRESS_NUM_LEADING_ZEROS.into())
            .all(|item| item == &0u8)
    }
}
