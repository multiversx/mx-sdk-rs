#[cfg(test)]
mod tests {
    use crate::test_util::{check_dep_encode, check_top_encode};

    #[test]
    fn test_dep_encode() {
        let bytes = check_dep_encode(&&[1u8, 2u8, 3u8][..]);
        assert_eq!(bytes.as_slice(), &[0, 0, 0, 3, 1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_top_encode_empty() {
        let empty_byte_slice: &[u8] = &[];
        let bytes = check_top_encode(&empty_byte_slice);
        assert_eq!(bytes.as_slice(), empty_byte_slice);
    }

    #[test]
    fn test_dep_encode_empty() {
        let empty_byte_slice: &[u8] = &[];
        let bytes = check_dep_encode(&empty_byte_slice);
        assert_eq!(bytes.as_slice(), &[0, 0, 0, 0])
    }
}
