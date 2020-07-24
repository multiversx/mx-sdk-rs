
use core::fmt::Debug;
use alloc::vec::Vec;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct H256([u8;32]);

pub type Address = H256;

impl From<[u8; 32]> for H256 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }
}

impl<'a> From<&'a [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 32]) -> Self {
        H256(*bytes)
    }
}

impl<'a> From<&'a mut [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 32]) -> Self {
        H256(*bytes)
    }
}

impl H256 {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut i = 0;
        let mut arr = [0u8; 32];
        while i < 32 && i < slice.len() {
            arr[i] = slice[i];
            i += 1;
        }
        H256(arr)
    }
}

impl From<H256> for [u8; 32] {
    #[inline]
    fn from(s: H256) -> Self {
        s.0
    }
}

impl AsRef<[u8]> for H256 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for H256 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}


impl H256 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub fn repeat_byte(byte: u8) -> H256 {
        H256([byte; 32])
    }

    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub fn zero() -> H256 {
        H256::repeat_byte(0u8)
    }

    /// Returns the size of this hash in bytes.
    #[inline]
    pub fn len_bytes() -> usize {
        32
    }

    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    #[inline]
    pub fn copy_to_array(&self, target: &mut [u8; 32]) {
        target.copy_from_slice(&self.0[..]);
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0[..].to_vec()
    }
}

use elrond_codec::*;

impl Encode for H256 {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.write(&self.0[..]);
        Ok(())
    }
}

impl Decode for H256 {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let mut arr = [0u8; 32];
        input.read_into(&mut arr)?;
        Ok(H256(arr))
    }
}

#[cfg(test)]
mod esd_light_tests {
    use super::*;
    use alloc::vec::Vec;
    use elrond_codec::test_util::ser_deser_ok;

    #[test]
    fn test_address() {
        let addr = Address::from([4u8; 32]);
        ser_deser_ok(addr, &[4u8; 32]);
    }

    #[test]
    fn test_opt_address() {
        let addr = Address::from([4u8; 32]);
        let mut expected: Vec<u8> = Vec::new();
        expected.push(1u8);
        expected.extend_from_slice(&[4u8; 32]);
        ser_deser_ok(Some(addr), expected.as_slice());
    }

    // #[test]
    // fn test_ser_address_ref() {
    //     let addr = Address::from([4u8; 32]);
    //     let expected_bytes: &[u8] = &[4u8; 32*3];

    //     let tuple = (&addr, &&&addr, addr.clone());
    //     let serialized_bytes = tuple.top_encode();
    //     assert_eq!(serialized_bytes.as_slice(), expected_bytes);
    // }
}
