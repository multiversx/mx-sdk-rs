use super::H256;
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;
const NUM_INIT_CHARS_FOR_SC: usize = 10;
const NUM_INIT_CHARS_FOR_METACHAIN_SC: usize = 15;
pub const VM_TYPE_LEN: usize = 2;
pub const DEFAULT_VM_TYPE: &[u8] = &[5, 0];

/// An Address is just a H256 with a different name.
/// Has a different ABI name than H256.
///
/// Note: we are currently using ManagedAddress in contracts.
/// While this also works, its use in contracts is discouraged.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Address(H256);

impl Address {
    /// Creates a new `Address` from a 32-byte array.
    pub const fn new(bytes: [u8; 32]) -> Self {
        Address(H256::new(bytes))
    }

    /// Constructs an Address from a hex string at compile time.
    /// The hex string can optionally start with "0x" or "0X".
    /// The hex string must represent exactly 32 bytes (64 hex characters).
    ///
    /// # Panics
    ///
    /// Panics at compile time if:
    /// - The hex string is not exactly 64 characters (excluding optional "0x" prefix)
    /// - Any character is not a valid hex digit (0-9, a-f, A-F)
    ///
    /// # Example
    ///
    /// ```
    /// # use multiversx_chain_core::types::Address;
    /// const ADDR: Address = Address::from_hex("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    /// const ADDR_WITH_PREFIX: Address = Address::from_hex("0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    /// ```
    pub const fn from_hex(hex_str: &str) -> Self {
        Address(H256::from_hex(hex_str))
    }

    /// Generates a mock smart contract address deterministically from a creator address and nonce.
    /// Used in testing environments to simulate address generation without a real VM.
    pub fn generate_mock_address(creator_address: &[u8], creator_nonce: u64) -> Self {
        let mut result = [0x00; 32];

        result[10] = 0x11;
        result[11] = 0x11;
        result[12] = 0x11;
        result[13] = 0x11;

        result[14..29].copy_from_slice(&creator_address[..15]);
        result[29] = creator_nonce as u8;
        result[30..].copy_from_slice(&creator_address[30..]);

        let start_index = NUM_INIT_CHARS_FOR_SC - VM_TYPE_LEN;
        result[start_index..(start_index + DEFAULT_VM_TYPE.len())].copy_from_slice(DEFAULT_VM_TYPE);

        Address::from(result)
    }

    /// Returns the shard ID of this address in a 3-shard configuration.
    pub fn shard_of_3(&self) -> ShardId {
        ShardConfig::THREE_SHARDS.compute_id(self)
    }

    /// Encodes the address as a bech32 string using the given human-readable part (HRP).
    #[cfg(feature = "std")]
    pub fn to_bech32(&self, hrp: &str) -> crate::std::Bech32Address {
        crate::std::Bech32Address::encode_address(hrp, self.clone())
    }

    /// Encodes the address as a bech32 string using the default HRP (`erd`).
    #[cfg(feature = "std")]
    pub fn to_bech32_default(&self) -> crate::std::Bech32Address {
        crate::std::Bech32Address::encode_address_default_hrp(self.clone())
    }

    /// Returns the address as a lowercase hex string (64 characters, no prefix).
    #[cfg(feature = "std")]
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
}

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
    /// Constructs an `Address` from a byte slice. Pads with zeros if shorter than 32 bytes,
    /// truncates if longer.
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

    /// Returns a reference to the underlying 32-byte array.
    #[inline]
    pub fn as_array(&self) -> &[u8; 32] {
        self.0.as_array()
    }

    /// Copies the address bytes into the provided 32-byte array.
    #[inline]
    pub fn copy_to_array(&self, target: &mut [u8; 32]) {
        self.0.copy_to_array(target)
    }

    /// Returns the address bytes as a heap-allocated `Vec<u8>`.
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

    /// Returns true if this address belongs to a smart contract.
    /// A smart contract address has its first 8 bytes set to zero.
    pub fn is_smart_contract_address(&self) -> bool {
        self.as_bytes()
            .iter()
            .take(SC_ADDRESS_NUM_LEADING_ZEROS.into())
            .all(|item| item == &0u8)
    }

    /// Returns true if this is a smart contract address deployed on the metachain.
    /// Conditions:
    ///   1. The last two bytes are both 0xFF (metachain identifier).
    ///   2. The address is a smart contract address (first 8 bytes are zero).
    ///   3. Bytes [10..25] (the "on-meta" region) are all zero.
    pub fn is_smart_contract_on_metachain(&self) -> bool {
        let bytes = self.as_bytes();
        if bytes[30] != 0xFF || bytes[31] != 0xFF {
            return false;
        }
        if !self.is_smart_contract_address() {
            return false;
        }
        bytes[NUM_INIT_CHARS_FOR_SC..NUM_INIT_CHARS_FOR_SC + NUM_INIT_CHARS_FOR_METACHAIN_SC]
            .iter()
            .all(|&b| b == 0)
    }
}

use crate::{
    codec::*,
    types::{ShardConfig, ShardId},
};

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

#[cfg(feature = "std")]
impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.to_hex(), f)
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
    fn test_from_hex() {
        let hex_str = "e32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60";
        let addr = Address::from_hex(hex_str);
        let expected = [
            0xe3, 0x2a, 0xfe, 0xdc, 0x90, 0x4f, 0xe1, 0x93, 0x97, 0x46, 0xad, 0x97, 0x3b, 0xeb,
            0x38, 0x35, 0x63, 0xcf, 0x63, 0x64, 0x2b, 0xa6, 0x69, 0xb3, 0x04, 0x0f, 0x9b, 0x94,
            0x28, 0xa5, 0xed, 0x60,
        ];
        assert_eq!(addr.as_array(), &expected);
    }

    #[test]
    fn test_from_hex_with_prefix() {
        let hex_str = "0xE32AFEDC904FE1939746AD973BEB383563CF63642BA669B3040F9B9428A5ED60";
        let addr = Address::from_hex(hex_str);
        let expected = [
            0xE3, 0x2A, 0xFE, 0xDC, 0x90, 0x4F, 0xE1, 0x93, 0x97, 0x46, 0xAD, 0x97, 0x3B, 0xEB,
            0x38, 0x35, 0x63, 0xCF, 0x63, 0x64, 0x2B, 0xA6, 0x69, 0xB3, 0x04, 0x0F, 0x9B, 0x94,
            0x28, 0xA5, 0xED, 0x60,
        ];
        assert_eq!(addr.as_array(), &expected);
    }

    #[test]
    fn test_from_hex_const() {
        const ADDR: Address =
            Address::from_hex("e32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60");
        assert!(!ADDR.is_zero());
    }

    // --- is_smart_contract_on_metachain tests ---

    /// ESDT system SC: canonical metachain SC address — all conditions satisfied.
    #[test]
    fn test_metachain_sc_esdt_system_sc() {
        let addr =
            Address::from_hex("000000000000000000010000000000000000000000000000000000000002ffff");
        assert!(addr.is_smart_contract_on_metachain());
    }

    /// A manually constructed metachain SC address:
    /// bytes 0-7 zero, bytes 10-24 zero, bytes 30-31 = 0xFF.
    #[test]
    fn test_metachain_sc_custom_address() {
        let mut arr = [0u8; 32];
        arr[30] = 0xFF;
        arr[31] = 0xFF;
        let addr = Address::new(arr);
        assert!(addr.is_smart_contract_on_metachain());
    }

    /// Regular user address: first bytes are non-zero, so not a SC at all.
    #[test]
    fn test_metachain_sc_user_address() {
        let addr =
            Address::from_hex("e32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60");
        assert!(!addr.is_smart_contract_on_metachain());
    }

    /// Regular SC address: first 8 bytes zero, but bytes 30-31 are not 0xFF.
    #[test]
    fn test_metachain_sc_regular_sc_not_metachain() {
        let addr =
            Address::from_hex("0000000000000000000100000000000000000000000000000000000000020001");
        assert!(addr.is_smart_contract_address());
        assert!(!addr.is_smart_contract_on_metachain());
    }

    /// Fails condition 1: only byte 30 is 0xFF, byte 31 is not.
    #[test]
    fn test_metachain_sc_only_byte30_is_ff() {
        let mut arr = [0u8; 32];
        arr[30] = 0xFF;
        arr[31] = 0x00;
        let addr = Address::new(arr);
        assert!(!addr.is_smart_contract_on_metachain());
    }

    /// Fails condition 1: only byte 31 is 0xFF, byte 30 is not.
    #[test]
    fn test_metachain_sc_only_byte31_is_ff() {
        let mut arr = [0u8; 32];
        arr[30] = 0x00;
        arr[31] = 0xFF;
        let addr = Address::new(arr);
        assert!(!addr.is_smart_contract_on_metachain());
    }

    /// Fails condition 2: bytes 30-31 are 0xFF but first 8 bytes are not all zero.
    #[test]
    fn test_metachain_sc_not_a_sc_address() {
        let mut arr = [0u8; 32];
        arr[0] = 0x01; // breaks SC condition
        arr[30] = 0xFF;
        arr[31] = 0xFF;
        let addr = Address::new(arr);
        assert!(!addr.is_smart_contract_on_metachain());
    }

    /// Fails condition 3: bytes 30-31 are 0xFF, it IS a SC, but the metachain
    /// region (bytes 10-24) contains a non-zero byte.
    #[test]
    fn test_metachain_sc_nonzero_in_metachain_region() {
        let mut arr = [0u8; 32];
        arr[10] = 0x01; // breaks metachain-region condition
        arr[30] = 0xFF;
        arr[31] = 0xFF;
        let addr = Address::new(arr);
        assert!(addr.is_smart_contract_address());
        assert!(!addr.is_smart_contract_on_metachain());
    }

    /// All-zero address: IS a SC address, but bytes 30-31 are 0x00, not 0xFF.
    #[test]
    fn test_metachain_sc_zero_address() {
        assert!(!Address::zero().is_smart_contract_on_metachain());
    }
}
