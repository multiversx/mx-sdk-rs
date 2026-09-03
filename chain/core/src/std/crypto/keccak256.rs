pub const KECCAK256_RESULT_LEN: usize = 32;

/// Computes the Keccak-256 hash of the given data.
///
/// Returns a 32-byte array containing the hash digest.
pub fn keccak256(data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
    use sha3::Digest;
    sha3::Keccak256::digest(data).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn keccak256_empty() {
        // Keccak-256("") =
        // c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
        let expected = hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
        assert_eq!(keccak256(b""), expected);
    }

    #[test]
    fn keccak256_abc() {
        // Keccak-256("abc") =
        // 4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45
        let expected = hex!("4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45");
        assert_eq!(keccak256(b"abc"), expected);
    }

    #[test]
    fn keccak256_different_inputs_differ() {
        assert_ne!(keccak256(b"foo"), keccak256(b"bar"));
    }
}
