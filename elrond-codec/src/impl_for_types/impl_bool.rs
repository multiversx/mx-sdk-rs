#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};

    #[test]
    fn test_top() {
        check_top_encode_decode(true, &[1]);
        check_top_encode_decode(false, &[]);
    }
    #[test]
    fn test_dep() {
        check_dep_encode_decode(true, &[1]);
        check_dep_encode_decode(false, &[0]);
    }
}
