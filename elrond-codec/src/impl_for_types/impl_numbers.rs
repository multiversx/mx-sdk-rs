#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use core::num::NonZeroUsize;

    #[test]
    fn test_top() {
        // zero
        check_top_encode_decode(0u8, &[]);
        check_top_encode_decode(0u16, &[]);
        check_top_encode_decode(0u32, &[]);
        check_top_encode_decode(0u64, &[]);
        check_top_encode_decode(0usize, &[]);
        // unsigned positive
        check_top_encode_decode(5u8, &[5]);
        check_top_encode_decode(5u16, &[5]);
        check_top_encode_decode(5u32, &[5]);
        check_top_encode_decode(5u64, &[5]);
        check_top_encode_decode(5usize, &[5]);
        // signed positive
        check_top_encode_decode(5i8, &[5]);
        check_top_encode_decode(5i16, &[5]);
        check_top_encode_decode(5i32, &[5]);
        check_top_encode_decode(5i64, &[5]);
        check_top_encode_decode(5isize, &[5]);
        // signed negative
        check_top_encode_decode(-5i8, &[251]);
        check_top_encode_decode(-5i16, &[251]);
        check_top_encode_decode(-5i32, &[251]);
        check_top_encode_decode(-5i64, &[251]);
        check_top_encode_decode(-5isize, &[251]);
        // non zero usize
        check_top_encode_decode(NonZeroUsize::new(5).unwrap(), &[5]);
    }

    #[test]
    fn test_dep() {
        // unsigned positive
        check_dep_encode_decode(5u8, &[5]);
        check_dep_encode_decode(5u16, &[0, 5]);
        check_dep_encode_decode(5u32, &[0, 0, 0, 5]);
        check_dep_encode_decode(5usize, &[0, 0, 0, 5]);
        check_dep_encode_decode(5u64, &[0, 0, 0, 0, 0, 0, 0, 5]);
        // signed positive
        check_dep_encode_decode(5i8, &[5]);
        check_dep_encode_decode(5i16, &[0, 5]);
        check_dep_encode_decode(5i32, &[0, 0, 0, 5]);
        check_dep_encode_decode(5isize, &[0, 0, 0, 5]);
        check_dep_encode_decode(5i64, &[0, 0, 0, 0, 0, 0, 0, 5]);
        // signed negative
        check_dep_encode_decode(-5i8, &[251]);
        check_dep_encode_decode(-5i16, &[255, 251]);
        check_dep_encode_decode(-5i32, &[255, 255, 255, 251]);
        check_dep_encode_decode(-5isize, &[255, 255, 255, 251]);
        check_dep_encode_decode(-5i64, &[255, 255, 255, 255, 255, 255, 255, 251]);
        // non zero usize
        check_dep_encode_decode(NonZeroUsize::new(5).unwrap(), &[0, 0, 0, 5]);
    }
}
