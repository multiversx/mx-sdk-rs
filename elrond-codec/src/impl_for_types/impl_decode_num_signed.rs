use crate::codec_err::DecodeError;
use crate::nested_de::NestedDecode;
use crate::nested_de_input::NestedDecodeInput;
use crate::num_conv::bytes_to_number;
use crate::top_de::TopDecode;
use crate::top_de_input::TopDecodeInput;
use crate::TypeInfo;

macro_rules! decode_num_signed {
    ($ty:ty, $num_bytes:expr, $type_info:expr) => {
        impl NestedDecode for $ty {
            const TYPE_INFO: TypeInfo = $type_info;

            fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                let bytes = input.read_slice($num_bytes)?;
                let num = bytes_to_number(bytes, true) as $ty;
                Ok(num)
            }

            fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                input: &mut I,
                c: ExitCtx,
                exit: fn(ExitCtx, DecodeError) -> !,
            ) -> Self {
                let bytes = input.read_slice_or_exit($num_bytes, c, exit);
                let num = bytes_to_number(bytes, true) as $ty;
                num
            }
        }
    };
}

decode_num_signed!(i8, 1, TypeInfo::I8);
decode_num_signed!(i16, 2, TypeInfo::I16);
decode_num_signed!(i32, 4, TypeInfo::I32);
decode_num_signed!(isize, 4, TypeInfo::ISIZE);
decode_num_signed!(i64, 8, TypeInfo::I64);

macro_rules! decode_num_signed {
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

decode_num_signed!(i8, i8, TypeInfo::I8);
decode_num_signed!(i16, i16, TypeInfo::I16);
decode_num_signed!(i32, i32, TypeInfo::I32);
decode_num_signed!(isize, i32, TypeInfo::ISIZE); // even if isize can be 64 bits on some platforms, we always deserialize as max 32 bits
decode_num_signed!(i64, i64, TypeInfo::I64);
