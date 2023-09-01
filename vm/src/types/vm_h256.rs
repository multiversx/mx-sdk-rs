use core::fmt::Debug;

// const ERR_BAD_H256_LENGTH: &str = "bad H256 length";
const ZERO_32: &[u8] = &[0u8; 32];

/// Type that holds 32 bytes of data.
/// Data is kept on the heap to keep wasm size low and avoid copies.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct H256([u8; 32]);

impl H256 {
    pub const fn new(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }
}

impl From<[u8; 32]> for H256 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    fn from(arr: [u8; 32]) -> Self {
        H256(arr)
    }
}

impl<'a> From<&'a [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
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
    fn from(bytes: &'a mut [u8; 32]) -> Self {
        H256(*bytes)
    }
}

impl From<Box<[u8; 32]>> for H256 {
    fn from(bytes: Box<[u8; 32]>) -> Self {
        H256(*bytes)
    }
}

impl H256 {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut arr = [0u8; 32];
        let len = core::cmp::min(slice.len(), 32);
        arr[..len].copy_from_slice(&slice[..len]);
        H256(arr)
    }
}

impl From<H256> for [u8; 32] {
    fn from(s: H256) -> Self {
        s.0
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for H256 {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl H256 {
    /// Returns a new zero-initialized fixed hash.
    pub fn zero() -> Self {
        H256([0u8; 32])
    }

    /// Extracts a byte slice containing the entire fixed hash.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn as_array(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0[..].to_vec()
    }

    /// True if all 32 bytes of the hash are zero.
    pub fn is_zero(&self) -> bool {
        self.as_bytes() == ZERO_32
    }
}
