pub const SHA256_RESULT_LEN: usize = 32;

/// Computes the SHA-256 hash of the given data.
///
/// Returns a 32-byte array containing the hash digest.
pub fn sha256(data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
    use sha2::Digest;
    sha2::Sha256::digest(data).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn sha256_empty() {
        // NIST FIPS 180-4: SHA-256("") =
        // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let expected = hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(sha256(b""), expected);
    }

    #[test]
    fn sha256_abc() {
        // NIST FIPS 180-4: SHA-256("abc") =
        // ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
        let expected = hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(sha256(b"abc"), expected);
    }

    #[test]
    fn sha256_different_inputs_differ() {
        assert_ne!(sha256(b"foo"), sha256(b"bar"));
    }
}
