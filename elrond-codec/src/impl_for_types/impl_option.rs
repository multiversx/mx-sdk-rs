#[cfg(test)]
pub mod tests {
    use alloc::vec::Vec;

    use crate::test_util::check_top_encode_decode;

    #[test]
    fn test_top() {
        let some_v = Some([1i32, 2i32, 3i32].to_vec());
        let expected: &[u8] = &[
            /*opt*/ 1, /*size*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1, 0, 0, 0, 2, 0, 0,
            0, 3,
        ];
        check_top_encode_decode(some_v, expected);

        let none_v: Option<Vec<i32>> = None;
        check_top_encode_decode(none_v, &[]);
    }
}
