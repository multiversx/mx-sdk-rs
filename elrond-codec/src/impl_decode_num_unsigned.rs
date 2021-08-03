use crate::codec_err::DecodeError;
use crate::nested_de::NestedDecode;
use crate::nested_de_input::NestedDecodeInput;
use crate::num_conv::bytes_to_number;
use crate::top_de::TopDecode;
use crate::top_de_input::TopDecodeInput;
use crate::TypeInfo;

macro_rules! decode_num_unsigned {
    ($ty:ty, $num_bytes:expr, $type_info:expr) => {
        impl NestedDecode for $ty {
            const TYPE_INFO: TypeInfo = $type_info;

            fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                let bytes = input.read_slice($num_bytes)?;
                let num = bytes_to_number(bytes, false) as $ty;
                Ok(num)
            }

            fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                input: &mut I,
                c: ExitCtx,
                exit: fn(ExitCtx, DecodeError) -> !,
            ) -> Self {
                let bytes = input.read_slice_or_exit($num_bytes, c, exit);
                let num = bytes_to_number(bytes, false) as $ty;
                num
            }
        }
    };
}

decode_num_unsigned!(u16, 2, TypeInfo::U16);
decode_num_unsigned!(u32, 4, TypeInfo::U32);
decode_num_unsigned!(usize, 4, TypeInfo::USIZE);
decode_num_unsigned!(u64, 8, TypeInfo::U64);

macro_rules! decode_num_unsigned {
    ($ty:ty, $bounds_ty:ty, $type_info:expr) => {
        impl TopDecode for $ty {
            const TYPE_INFO: TypeInfo = $type_info;

            fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                let arg_u64 = input.into_u64();
                let max = <$bounds_ty>::MAX as u64;
                if arg_u64 > max {
                    Err(DecodeError::INPUT_TOO_LONG)
                } else {
                    Ok(arg_u64 as $ty)
                }
            }

            fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
                input: I,
                c: ExitCtx,
                exit: fn(ExitCtx, DecodeError) -> !,
            ) -> Self {
                let arg_u64 = input.into_u64();
                let max = <$bounds_ty>::MAX as u64;
                if arg_u64 > max {
                    exit(c, DecodeError::INPUT_TOO_LONG)
                } else {
                    arg_u64 as $ty
                }
            }
        }
    };
}

decode_num_unsigned!(u8, u8, TypeInfo::U8);
decode_num_unsigned!(u16, u16, TypeInfo::U16);
decode_num_unsigned!(u32, u32, TypeInfo::U32);
decode_num_unsigned!(usize, u32, TypeInfo::USIZE); // even if usize can be 64 bits on some platforms, we always deserialize as max 32 bits
decode_num_unsigned!(u64, u64, TypeInfo::U64);
