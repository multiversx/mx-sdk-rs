use super::{heap_h256::HeapH256, BoxedBytes};

use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;

/// Old smart contracts were using this Address implementation,
/// which was explicitly relying on the heap, to avoid large data copies on the stack.
///
/// It is no longer used, kept for reference.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct HeapAddress(HeapH256);

impl From<HeapH256> for HeapAddress {
    #[inline]
    fn from(hash: HeapH256) -> Self {
        HeapAddress(hash)
    }
}

impl From<HeapAddress> for HeapH256 {
    #[inline]
    fn from(address: HeapAddress) -> Self {
        address.0
    }
}

impl<'a> From<&'a HeapAddress> for &'a HeapH256 {
    #[inline]
    fn from(address: &'a HeapAddress) -> Self {
        &address.0
    }
}

impl From<[u8; 32]> for HeapAddress {
    #[inline]
    fn from(arr: [u8; 32]) -> Self {
        HeapAddress(HeapH256::from(arr))
    }
}

impl<'a> From<&'a [u8; 32]> for HeapAddress {
    #[inline]
    fn from(bytes: &'a [u8; 32]) -> Self {
        HeapAddress(HeapH256::from(bytes))
    }
}

impl<'a> From<&'a mut [u8; 32]> for HeapAddress {
    #[inline]
    fn from(bytes: &'a mut [u8; 32]) -> Self {
        HeapAddress(HeapH256::from(bytes))
    }
}

impl From<Box<[u8; 32]>> for HeapAddress {
    #[inline]
    fn from(bytes: Box<[u8; 32]>) -> Self {
        HeapAddress(HeapH256::from(bytes))
    }
}

impl HeapAddress {
    pub fn from_slice(slice: &[u8]) -> Self {
        HeapAddress(HeapH256::from_slice(slice))
    }
}

impl From<HeapAddress> for [u8; 32] {
    #[inline]
    fn from(addr: HeapAddress) -> Self {
        addr.0.into()
    }
}

impl AsRef<[u8]> for HeapAddress {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for HeapAddress {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl HeapAddress {
    /// Returns a new address of 32 zeros.
    /// Allocates directly in heap.
    /// Minimal resulting wasm code (14 bytes if not inlined).
    pub fn zero() -> Self {
        HeapAddress(HeapH256::zero())
    }

    /// Returns the size of an address in bytes.
    #[inline]
    pub fn len_bytes() -> usize {
        HeapH256::len_bytes()
    }

    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn as_array(&self) -> &[u8; 32] {
        self.0.as_array()
    }

    #[inline]
    pub fn copy_to_array(&self, target: &mut [u8; 32]) {
        self.0.copy_to_array(target)
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Pointer to the data on the heap.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the data on the heap.
    /// Used by the API to populate data.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    /// True if all 32 bytes of the hash are zero.
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Transmutes self to an (in principle) variable length boxed bytes object.
    /// Both BoxedBytes and H256 keep the data on the heap, so only the pointer to that data needs to be transmuted.
    /// Does not reallocate or copy data, the data on the heap remains untouched.
    pub fn into_boxed_bytes(self) -> BoxedBytes {
        self.0.into_boxed_bytes()
    }

    pub fn is_smart_contract_address(&self) -> bool {
        self.as_bytes()
            .iter()
            .take(SC_ADDRESS_NUM_LEADING_ZEROS.into())
            .all(|item| item == &0u8)
    }
}

use crate::codec::*;

impl NestedEncode for HeapAddress {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for HeapAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for HeapAddress {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(HeapAddress(HeapH256::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for HeapAddress {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(HeapAddress(HeapH256::top_decode_or_handle_err(input, h)?))
    }
}

#[cfg(test)]
mod address_tests {
    use super::*;
    use crate::codec::test_util::{check_top_encode, check_top_encode_decode};
    use alloc::vec::Vec;

    #[test]
    fn test_address() {
        let addr = HeapAddress::from([4u8; 32]);
        check_top_encode_decode(addr, &[4u8; 32]);
    }

    #[test]
    fn test_opt_address() {
        let addr = HeapAddress::from([4u8; 32]);
        let mut expected: Vec<u8> = Vec::new();
        expected.push(1u8);
        expected.extend_from_slice(&[4u8; 32]);
        check_top_encode_decode(Some(addr), expected.as_slice());
    }

    #[test]
    fn test_ser_address_ref() {
        let addr = HeapAddress::from([4u8; 32]);
        let expected_bytes: &[u8] = &[4u8; 32 * 3];

        let tuple = (&addr, &&&addr, addr.clone());
        let serialized_bytes = check_top_encode(&tuple);
        assert_eq!(serialized_bytes.as_slice(), expected_bytes);
    }

    #[test]
    fn test_is_zero() {
        assert!(HeapAddress::zero().is_zero());
    }

    #[test]
    fn test_size_of() {
        use core::mem::size_of;
        assert_eq!(size_of::<HeapAddress>(), size_of::<usize>());
        assert_eq!(size_of::<Option<HeapAddress>>(), size_of::<usize>());
    }
}
