
use core::fmt::Debug;
use alloc::boxed::Box;
use alloc::vec::Vec;

const ERR_BAD_H256_LENGTH: &[u8] = b"bad H256 length";

/// Type that holds 32 bytes of data.
/// Data is kept on the heap to keep wasm size low and avoid copies.
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct H256(Box<[u8;32]>);

/// Alias for H256, just to make smart contract code more readable.
pub type Address = H256;

impl From<[u8; 32]> for H256 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(arr: [u8; 32]) -> Self {
        H256(Box::new(arr))
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
        H256(Box::new(*bytes))
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
        H256(Box::new(*bytes))
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
        H256(Box::new(arr))
    }
}

impl From<H256> for [u8; 32] {
    #[inline]
    fn from(s: H256) -> Self {
        *(s.0)
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
        self.0.as_mut()
    }
}


impl H256 {
    /// Returns a new zero-initialized fixed hash.
    /// Allocates directly in heap.
    /// Minimal resulting wasm code (14 bytes if not inlined).
    pub fn zero() -> H256 {
        use alloc::alloc::{alloc, Layout};
        unsafe {
            let ptr = alloc(Layout::new::<[u8; 32]>()) as *mut [u8; 32];
            H256(Box::from_raw(ptr))
        }
    }

    /// Returns the size of this hash in bytes.
    #[inline]
    pub fn len_bytes() -> usize {
        32
    }

    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    #[inline]
    pub fn copy_to_array(&self, target: &mut [u8; 32]) {
        target.copy_from_slice(&self.0[..]);
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0[..].to_vec()
    }

    /// Returns an unsafe mutable pointer to the data on the heap.
    /// Used by the API to populate data.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

use elrond_codec::*;

impl NestedEncode for H256 {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.write(&self.0[..]);
        Ok(())
    }

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(&self, dest: &mut O, _: ExitCtx, _: fn(ExitCtx, EncodeError) -> !) {
		dest.write(&self.0[..]);
	}
}

impl TopEncode for H256 {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_slice_u8(&self.0[..]);
        Ok(())
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, _: ExitCtx, _: fn(ExitCtx, EncodeError) -> !) {
		output.set_slice_u8(&self.0[..]);
	}
}

impl NestedDecode for H256 {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let mut res = H256::zero();
        input.read_into(res.as_mut())?;
        Ok(res)
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(input: &mut I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
        let mut res = H256::zero();
        input.read_into_or_exit(res.as_mut(), c, exit);
        res
    }
}

impl TopDecode for H256 {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        match <[u8; 32]>::top_decode_boxed(input) {
            Ok(array_box) => Ok(H256(array_box)),
            Err(_) => Err(DecodeError::from(ERR_BAD_H256_LENGTH)),
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(input: I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
        // transmute directly
        let bs = input.into_boxed_slice_u8();
        if bs.len() != 32 {
            exit(c, DecodeError::from(ERR_BAD_H256_LENGTH));
        }
        let raw = Box::into_raw(bs);
        let array_box = unsafe { Box::<[u8; 32]>::from_raw(raw as *mut [u8; 32]) };
        H256(array_box)
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

    #[test]
    fn test_ser_address_ref() {
        let addr = Address::from([4u8; 32]);
        let expected_bytes: &[u8] = &[4u8; 32*3];

        let tuple = (&addr, &&&addr, addr.clone());
        let serialized_bytes = top_encode_to_vec(&tuple).unwrap();
        assert_eq!(serialized_bytes.as_slice(), expected_bytes);
    }
}
