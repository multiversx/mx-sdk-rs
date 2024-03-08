use super::h256::H256;
use crate::{
    abi::{TypeAbi, TypeName},
    types::heap::BoxedBytes,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;

/// An Address is just a H256 with a different name.
/// Has a different ABI name than H256.
///
/// Note: we are currently using ManagedAddress in contracts.
/// While this also works, its use in contracts is discouraged.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Address(H256);

impl From<H256> for Address {
    #[inline]
    fn from(hash: H256) -> Self {
        Address(hash)
    }
}

impl From<Address> for H256 {
    #[inline]
    fn from(address: Address) -> Self {
        address.0
    }
}

impl<'a> From<&'a Address> for &'a H256 {
    #[inline]
    fn from(address: &'a Address) -> Self {
        &address.0
    }
}

impl From<[u8; 32]> for Address {
    #[inline]
    fn from(arr: [u8; 32]) -> Self {
        Address(H256::from(arr))
    }
}

impl<'a> From<&'a [u8; 32]> for Address {
    #[inline]
    fn from(bytes: &'a [u8; 32]) -> Self {
        Address(H256::from(bytes))
    }
}

impl<'a> From<&'a mut [u8; 32]> for Address {
    #[inline]
    fn from(bytes: &'a mut [u8; 32]) -> Self {
        Address(H256::from(bytes))
    }
}

impl From<Box<[u8; 32]>> for Address {
    #[inline]
    fn from(bytes: Box<[u8; 32]>) -> Self {
        Address(H256::from(bytes))
    }
}

impl Address {
    pub fn from_slice(slice: &[u8]) -> Self {
        Address(H256::from_slice(slice))
    }
}

impl From<Address> for [u8; 32] {
    #[inline]
    fn from(addr: Address) -> Self {
        addr.0.into()
    }
}

impl AsRef<[u8]> for Address {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for Address {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl Address {
    /// Returns a new address of 32 zeros.
    /// Allocates directly in heap.
    /// Minimal resulting wasm code (14 bytes if not inlined).
    pub fn zero() -> Self {
        Address(H256::zero())
    }

    /// Returns the size of an address in bytes.
    #[inline]
    pub fn len_bytes() -> usize {
        H256::len_bytes()
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

impl NestedEncode for Address {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for Address {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for Address {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Address(H256::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for Address {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Address(H256::top_decode_or_handle_err(input, h)?))
    }
}

impl TypeAbi for Address {
    fn type_name() -> TypeName {
        "Address".into()
    }
}

#[cfg(test)]
mod address_tests {
    use super::*;
    use crate::codec::test_util::{check_top_encode, check_top_encode_decode};
    use alloc::vec::Vec;

    #[test]
    fn test_address() {
        let addr = Address::from([4u8; 32]);
        check_top_encode_decode(addr, &[4u8; 32]);
    }

    #[test]
    fn test_opt_address() {
        let addr = Address::from([4u8; 32]);
        let mut expected: Vec<u8> = Vec::new();
        expected.push(1u8);
        expected.extend_from_slice(&[4u8; 32]);
        check_top_encode_decode(Some(addr), expected.as_slice());
    }

    #[test]
    fn test_ser_address_ref() {
        let addr = Address::from([4u8; 32]);
        let expected_bytes: &[u8] = &[4u8; 32 * 3];

        let tuple = (&addr, &&&addr, addr.clone());
        let serialized_bytes = check_top_encode(&tuple);
        assert_eq!(serialized_bytes.as_slice(), expected_bytes);
    }

    #[test]
    fn test_is_zero() {
        assert!(Address::zero().is_zero());
    }

    #[test]
    fn test_size_of() {
        use core::mem::size_of;
        assert_eq!(size_of::<Address>(), size_of::<usize>());
        assert_eq!(size_of::<Option<Address>>(), size_of::<usize>());
    }
}
