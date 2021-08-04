#[cfg(test)]
mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use alloc::string::String;

    #[test]
    fn test_top() {
        let s = "abc";
        check_top_encode_decode(String::from(s), &[b'a', b'b', b'c']);
        check_top_encode_decode(String::from(s).into_boxed_str(), &[b'a', b'b', b'c']);
    }

    #[test]
    fn test_dep() {
        let s = "abc";
        check_dep_encode_decode(String::from(s), &[0, 0, 0, 3, b'a', b'b', b'c']);
        check_dep_encode_decode(
            String::from(s).into_boxed_str(),
            &[0, 0, 0, 3, b'a', b'b', b'c'],
        );
    }
}
