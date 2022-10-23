use crate::{
    dep_encode_num_mimic, num_conv::universal_decode_number, DecodeError, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

macro_rules! top_encode_num_signed {
    ($num_type:ty, $size_in_bits:expr) => {
        impl TopEncode for $num_type {
            #[inline]
            fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
            where
                O: TopEncodeOutput,
                H: EncodeErrorHandler,
            {
                output.set_i64(*self as i64);
                Ok(())
            }
        }
    };
}

top_encode_num_signed! {i64, 64}
top_encode_num_signed! {i32, 32}
top_encode_num_signed! {isize, 32}
top_encode_num_signed! {i16, 16}
top_encode_num_signed! {i8, 8}

dep_encode_num_mimic! {i64, u64}
dep_encode_num_mimic! {i32, u32}
dep_encode_num_mimic! {isize, u32}
dep_encode_num_mimic! {i16, u16}
dep_encode_num_mimic! {i8, u8}

macro_rules! dep_decode_num_signed {
    ($ty:ty, $num_bytes:expr) => {
        impl NestedDecode for $ty {
            fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
            where
                I: NestedDecodeInput,
                H: DecodeErrorHandler,
            {
                let mut bytes = [0u8; $num_bytes];
                input.read_into(&mut bytes[..], h)?;
                let num = universal_decode_number(&bytes[..], true) as $ty;
                Ok(num)
            }
        }
    };
}

dep_decode_num_signed!(i8, 1);
dep_decode_num_signed!(i16, 2);
dep_decode_num_signed!(i32, 4);
dep_decode_num_signed!(isize, 4);
dep_decode_num_signed!(i64, 8);

macro_rules! top_decode_num_signed {
    ($ty:ty, $bounds_ty:ty) => {
        impl TopDecode for $ty {
            fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
            where
                I: TopDecodeInput,
                H: DecodeErrorHandler,
            {
                let arg_i64 = input.into_i64(h)?;
                let min = <$bounds_ty>::MIN as i64;
                let max = <$bounds_ty>::MAX as i64;
                if arg_i64 < min || arg_i64 > max {
                    Err(h.handle_error(DecodeError::INPUT_OUT_OF_RANGE))
                } else {
                    Ok(arg_i64 as $ty)
                }
            }
        }
    };
}

top_decode_num_signed!(i8, i8);
top_decode_num_signed!(i16, i16);
top_decode_num_signed!(i32, i32);
top_decode_num_signed!(isize, i32); // even if isize can be 64 bits on some platforms, we always deserialize as max 32 bits
top_decode_num_signed!(i64, i64);

#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};

    #[test]
    fn test_top() {
        // signed zero
        check_top_encode_decode(0i8, &[]);
        check_top_encode_decode(0i16, &[]);
        check_top_encode_decode(0i32, &[]);
        check_top_encode_decode(0i64, &[]);
        check_top_encode_decode(0isize, &[]);
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
    }

    #[test]
    fn test_dep() {
        // signed zero
        check_dep_encode_decode(0i8, &[0]);
        check_dep_encode_decode(0i16, &[0, 0]);
        check_dep_encode_decode(0i32, &[0, 0, 0, 0]);
        check_dep_encode_decode(0isize, &[0, 0, 0, 0]);
        check_dep_encode_decode(0i64, &[0, 0, 0, 0, 0, 0, 0, 0]);
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
    }
}
