#[cfg(test)]
pub mod tests {

    use crate::test_util::check_top_encode_decode;

    #[test]
    fn test_top_vec_i32_compacted() {
        let v = [1i32, 2i32, 3i32].to_vec();
        let expected: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        check_top_encode_decode(v, expected);
    }

    #[test]
    fn test_top_vec_u8_compacted() {
        check_top_encode_decode([1u8, 2u8, 3u8].to_vec(), &[1u8, 2u8, 3u8]);
    }
}
