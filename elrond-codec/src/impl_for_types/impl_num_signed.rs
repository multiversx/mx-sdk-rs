use crate::{
    dep_encode_from_no_err, dep_encode_num_mimic, num_conv::bytes_to_number,
    top_encode_from_no_err, top_ser::TopEncodeNoErr, DecodeError, EncodeError, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeNoErr, NestedEncodeOutput, TopDecode,
    TopDecodeInput, TopEncode, TopEncodeOutput, TypeInfo,
};

macro_rules! top_encode_num_signed {
    ($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
        impl TopEncodeNoErr for $num_type {
            #[inline]
            fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
                output.set_i64(*self as i64);
            }
        }

        top_encode_from_no_err! {$num_type, $type_info}
    };
}

top_encode_num_signed! {i64, 64, TypeInfo::I64}
top_encode_num_signed! {i32, 32, TypeInfo::I32}
top_encode_num_signed! {isize, 32, TypeInfo::ISIZE}
top_encode_num_signed! {i16, 16, TypeInfo::I16}
top_encode_num_signed! {i8, 8, TypeInfo::I8}

dep_encode_num_mimic! {i64, u64, TypeInfo::I64}
dep_encode_num_mimic! {i32, u32, TypeInfo::I32}
dep_encode_num_mimic! {isize, u32, TypeInfo::ISIZE}
dep_encode_num_mimic! {i16, u16, TypeInfo::I16}
dep_encode_num_mimic! {i8, u8, TypeInfo::I8}

macro_rules! dep_decode_num_signed {
    ($ty:ty, $num_bytes:expr, $type_info:expr) => {
        impl NestedDecode for $ty {
            const TYPE_INFO: TypeInfo = $type_info;

            fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                let mut bytes = [0u8; $num_bytes];
                input.read_into(&mut bytes[..])?;
                let num = bytes_to_number(&bytes[..], true) as $ty;
                Ok(num)
            }

            fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                input: &mut I,
                c: ExitCtx,
                exit: fn(ExitCtx, DecodeError) -> !,
            ) -> Self {
                let mut bytes = [0u8; $num_bytes];
                input.read_into_or_exit(&mut bytes[..], c, exit);
                let num = bytes_to_number(&bytes[..], true) as $ty;
                num
            }
        }
    };
}

dep_decode_num_signed!(i8, 1, TypeInfo::I8);
dep_decode_num_signed!(i16, 2, TypeInfo::I16);
dep_decode_num_signed!(i32, 4, TypeInfo::I32);
dep_decode_num_signed!(isize, 4, TypeInfo::ISIZE);
dep_decode_num_signed!(i64, 8, TypeInfo::I64);

macro_rules! top_decode_num_signed {
    ($ty:ty, $bounds_ty:ty, $type_info:expr) => {
        impl TopDecode for $ty {
            const TYPE_INFO: TypeInfo = $type_info;

            fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                let arg_i64 = input.into_i64();
                let min = <$bounds_ty>::MIN as i64;
                let max = <$bounds_ty>::MAX as i64;
                if arg_i64 < min || arg_i64 > max {
                    Err(DecodeError::INPUT_OUT_OF_RANGE)
                } else {
                    Ok(arg_i64 as $ty)
                }
            }

            fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
                input: I,
                c: ExitCtx,
                exit: fn(ExitCtx, DecodeError) -> !,
            ) -> Self {
                let arg_i64 = input.into_i64();
                let min = <$bounds_ty>::MIN as i64;
                let max = <$bounds_ty>::MAX as i64;
                if arg_i64 < min || arg_i64 > max {
                    exit(c, DecodeError::INPUT_OUT_OF_RANGE)
                } else {
                    arg_i64 as $ty
                }
            }
        }
    };
}

top_decode_num_signed!(i8, i8, TypeInfo::I8);
top_decode_num_signed!(i16, i16, TypeInfo::I16);
top_decode_num_signed!(i32, i32, TypeInfo::I32);
top_decode_num_signed!(isize, i32, TypeInfo::ISIZE); // even if isize can be 64 bits on some platforms, we always deserialize as max 32 bits
top_decode_num_signed!(i64, i64, TypeInfo::I64);

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
