use crate::{
    dep_encode_from_no_err, dep_encode_num_mimic, num_conv::bytes_to_number,
    top_encode_from_no_err, top_ser::TopEncodeNoErr, DecodeError, EncodeError, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeNoErr, NestedEncodeOutput, TopDecode,
    TopDecodeInput, TopEncode, TopEncodeOutput, TypeInfo,
};

// No reversing needed for u8, because it is a single byte.
impl NestedEncodeNoErr for u8 {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
        dest.push_byte(*self as u8);
    }
}
dep_encode_from_no_err! {u8, TypeInfo::U8}

dep_encode_num_mimic! {usize, u32, TypeInfo::USIZE}

// The main unsigned types need to be reversed before serializing.
macro_rules! dep_encode_num_unsigned {
    ($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
        impl NestedEncodeNoErr for $num_type {
            fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
                dest.write(&self.to_be_bytes()[..]);
            }
        }

        dep_encode_from_no_err! {$num_type, $type_info}
    };
}

dep_encode_num_unsigned! {u64, 64, TypeInfo::U64}
dep_encode_num_unsigned! {u32, 32, TypeInfo::U32}
dep_encode_num_unsigned! {u16, 16, TypeInfo::U16}

macro_rules! top_encode_num_unsigned {
    ($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
        impl TopEncodeNoErr for $num_type {
            #[inline]
            fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
                output.set_u64(*self as u64);
            }
        }
        top_encode_from_no_err! {$num_type, $type_info}
    };
}

top_encode_num_unsigned! {u64, 64, TypeInfo::U64}
top_encode_num_unsigned! {u32, 32, TypeInfo::U32}
top_encode_num_unsigned! {usize, 32, TypeInfo::USIZE}
top_encode_num_unsigned! {u16, 16, TypeInfo::U16}
top_encode_num_unsigned! {u8, 8, TypeInfo::U8}

impl NestedDecode for u8 {
    const TYPE_INFO: TypeInfo = TypeInfo::U8;

    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        input.read_byte()
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        input.read_byte_or_exit(c, exit)
    }
}

macro_rules! dep_decode_num_unsigned {
    ($ty:ty, $num_bytes:expr, $type_info:expr) => {
        impl NestedDecode for $ty {
            const TYPE_INFO: TypeInfo = $type_info;

            fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                let mut bytes = [0u8; $num_bytes];
                input.read_into(&mut bytes[..])?;
                let num = bytes_to_number(&bytes[..], false) as $ty;
                Ok(num)
            }

            fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                input: &mut I,
                c: ExitCtx,
                exit: fn(ExitCtx, DecodeError) -> !,
            ) -> Self {
                let mut bytes = [0u8; $num_bytes];
                input.read_into_or_exit(&mut bytes[..], c, exit);
                let num = bytes_to_number(&bytes[..], false) as $ty;
                num
            }
        }
    };
}

dep_decode_num_unsigned!(u16, 2, TypeInfo::U16);
dep_decode_num_unsigned!(u32, 4, TypeInfo::U32);
dep_decode_num_unsigned!(usize, 4, TypeInfo::USIZE);
dep_decode_num_unsigned!(u64, 8, TypeInfo::U64);

macro_rules! top_decode_num_unsigned {
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

top_decode_num_unsigned!(u8, u8, TypeInfo::U8);
top_decode_num_unsigned!(u16, u16, TypeInfo::U16);
top_decode_num_unsigned!(u32, u32, TypeInfo::U32);
top_decode_num_unsigned!(usize, u32, TypeInfo::USIZE); // even if usize can be 64 bits on some platforms, we always deserialize as max 32 bits
top_decode_num_unsigned!(u64, u64, TypeInfo::U64);

#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};

    #[test]
    fn test_top() {
        // unsigned zero
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
    }

    #[test]
    fn test_dep() {
        // unsigned zero
        check_dep_encode_decode(0u8, &[0]);
        check_dep_encode_decode(0u16, &[0, 0]);
        check_dep_encode_decode(0u32, &[0, 0, 0, 0]);
        check_dep_encode_decode(0usize, &[0, 0, 0, 0]);
        check_dep_encode_decode(0u64, &[0, 0, 0, 0, 0, 0, 0, 0]);
        // unsigned positive
        check_dep_encode_decode(5u8, &[5]);
        check_dep_encode_decode(5u16, &[0, 5]);
        check_dep_encode_decode(5u32, &[0, 0, 0, 5]);
        check_dep_encode_decode(5usize, &[0, 0, 0, 5]);
        check_dep_encode_decode(5u64, &[0, 0, 0, 0, 0, 0, 0, 5]);
    }
}
