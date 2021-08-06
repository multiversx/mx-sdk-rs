#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use alloc::vec::Vec;

    #[test]
    fn test_empty_vec_compacted() {
        check_top_encode_decode(Vec::<u8>::new(), &[]);
    }

    #[test]
    fn test_top_unit() {
        check_top_encode_decode((), &[]);
    }

    #[test]
    fn test_dep_unit() {
        check_dep_encode_decode((), &[]);
    }
}
