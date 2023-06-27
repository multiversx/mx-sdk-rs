use super::H256;

use core::fmt::Debug;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;

/// An Address is just a H256 with a different name.
/// Has a different ABI name than H256.
///
/// Note: we are currently using ManagedAddress in contracts.
/// While this also works, its use in contracts is discouraged.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct VMAddress(H256);

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

    /// Returns the size of an address in bytes.
    pub fn len_bytes() -> usize {
        H256::len_bytes()
    }

    /// Extracts a byte slice containing the entire fixed hash.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn as_array(&self) -> &[u8; 32] {
        self.0.as_array()
    }

    pub fn copy_to_array(&self, target: &mut [u8; 32]) {
        self.0.copy_to_array(target)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Pointer to the data on the heap.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the data on the heap.
    /// Used by the API to populate data.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    /// True if all 32 bytes of the hash are zero.
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    // /// Transmutes self to an (in principle) variable length boxed bytes object.
    // /// Both BoxedBytes and H256 keep the data on the heap, so only the pointer to that data needs to be transmuted.
    // /// Does not reallocate or copy data, the data on the heap remains untouched.
    // pub fn into_boxed_bytes(self) -> BoxedBytes {
    //     self.0.into_boxed_bytes()
    // }

    pub fn is_smart_contract_address(&self) -> bool {
        self.as_bytes()
            .iter()
            .take(SC_ADDRESS_NUM_LEADING_ZEROS.into())
            .all(|item| item == &0u8)
    }
}
