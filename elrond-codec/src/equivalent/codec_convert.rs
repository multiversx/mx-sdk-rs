use crate::{
    CodecFrom, PanicErrorHandler, TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
};

pub fn codec_convert_or_panic<From, To, Medium>(from: From) -> To
where
    From: TopEncodeMulti,
    To: CodecFrom<From>,
    Medium: Default + TopDecodeMultiInput + TopEncodeMultiOutput,
{
    let mut medium: Medium = Default::default();
    let Ok(()) = from.multi_encode_or_handle_err(&mut medium, PanicErrorHandler);
    let Ok(result) = To::multi_decode_or_handle_err(&mut medium, PanicErrorHandler);
    result
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn test_codec_convert_or_panic() {
        assert_eq!(5i64, codec_convert_or_panic::<_, _, Vec<Vec<u8>>>(5i64));
        assert_eq!(5i64, codec_convert_or_panic::<_, _, Vec<Vec<u8>>>(5i32));
        assert_eq!(5i64, codec_convert_or_panic::<_, _, Vec<Vec<u8>>>(5i32));
    }

    fn convert_add<T1, T2, R>(x: T1, y: T2) -> R
    where
        T1: TopEncodeMulti,
        T2: TopEncodeMulti,
        u32: CodecFrom<T1>,
        u32: CodecFrom<T2>,
        R: CodecFrom<u32>,
    {
        let conv_x = codec_convert_or_panic::<T1, u32, Vec<Vec<u8>>>(x);
        let conv_y = codec_convert_or_panic::<T2, u32, Vec<Vec<u8>>>(y);
        codec_convert_or_panic::<u32, R, Vec<Vec<u8>>>(conv_x + conv_y)
    }

    #[test]
    fn test_convert_add() {
        assert_eq!(3u32, convert_add(1u32, 2u32));
        assert_eq!(8u64, convert_add(3u16, 5u8));
        assert_eq!(17usize, convert_add(8usize, 9usize));
    }
}
