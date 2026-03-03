#[cfg(test)]
mod tests {
    use crate::test_util::{
        check_dep_encode, check_dep_encode_decode, check_top_encode, check_top_encode_decode,
    };

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

    #[test]
    fn test_top_encode_non_empty() {
        let bytes = check_top_encode(&&[1u8, 2u8, 3u8][..]);
        assert_eq!(bytes.as_slice(), &[1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_top_vec_u8_decode() {
        // Vec<u8> round-trips as byte buffer
        check_top_encode_decode(alloc::vec![1u8, 2, 3], &[1, 2, 3]);
    }

    #[test]
    fn test_dep_vec_u8_decode() {
        check_dep_encode_decode(alloc::vec![1u8, 2, 3], &[0, 0, 0, 3, 1, 2, 3]);
    }
}
