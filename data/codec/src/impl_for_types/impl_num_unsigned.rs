use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    dep_encode_num_mimic,
};

// No reversing needed for u8, because it is a single byte.
impl NestedEncode for u8 {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        dest.push_byte(*self);
        Ok(())
    }

    fn if_u8<Output, If, Else, R>(input: Output, if_branch: If, _else_branch: Else) -> R
    where
        If: FnOnce(Output) -> R,
        Else: FnOnce(Output) -> R,
    {
        if_branch(input)
    }
}

dep_encode_num_mimic! {usize, u32}

// The main unsigned types need to be reversed before serializing.
macro_rules! dep_encode_num_unsigned {
    ($num_type:ty, $size_in_bits:expr) => {
        impl NestedEncode for $num_type {
            #[inline]
            fn dep_encode_or_handle_err<O, H>(
                &self,
                dest: &mut O,
                _h: H,
            ) -> Result<(), H::HandledErr>
            where
                O: NestedEncodeOutput,
                H: EncodeErrorHandler,
            {
                dest.write(&self.to_be_bytes()[..]);
                Ok(())
            }
        }
    };
}

dep_encode_num_unsigned! {u128, 128}
dep_encode_num_unsigned! {u64, 64}
dep_encode_num_unsigned! {u32, 32}
dep_encode_num_unsigned! {u16, 16}

macro_rules! top_encode_num_unsigned {
    ($num_type:ty, $size_in_bits:expr) => {
        impl TopEncode for $num_type {
            #[inline]
            fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
            where
                O: TopEncodeOutput,
                H: EncodeErrorHandler,
            {
                output.set_u64(*self as u64);
                Ok(())
            }
        }
    };
}

impl TopEncode for u128 {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if *self <= u64::MAX as u128 {
            output.set_u64(*self as u64);
        } else {
            // minimal big-endian representation, between 9 and 16 bytes here;
            // the high half is non-zero, so a single 64-bit `clz` is enough
            let hi = (*self >> 64) as u64;
            let byte_len = 8 + (64 - hi.leading_zeros() as usize).div_ceil(8);
            let bytes = self.to_be_bytes();
            output.set_slice_u8(&bytes[16 - byte_len..]);
        }
        Ok(())
    }
}
top_encode_num_unsigned! {u64, 64}
top_encode_num_unsigned! {u32, 32}
top_encode_num_unsigned! {usize, 32}
top_encode_num_unsigned! {u16, 16}
top_encode_num_unsigned! {u8, 8}

impl NestedDecode for u8 {
    #[inline]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        input.read_byte(h)
    }

    fn if_u8<Input, If, Else, R>(input: Input, if_branch: If, _else_branch: Else) -> R
    where
        If: FnOnce(Input) -> R,
        Else: FnOnce(Input) -> R,
    {
        if_branch(input)
    }
}

macro_rules! dep_decode_num_unsigned {
    ($ty:ty, $num_bytes:expr) => {
        impl NestedDecode for $ty {
            #[inline]
            fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
            where
                I: NestedDecodeInput,
                H: DecodeErrorHandler,
            {
                let mut bytes = [0u8; $num_bytes];
                input.read_into(&mut bytes[..], h)?;
                let num = <$ty>::from_be_bytes(bytes);
                Ok(num)
            }
        }
    };
}

dep_decode_num_unsigned!(u16, 2);
dep_decode_num_unsigned!(u32, 4);
dep_decode_num_unsigned!(u64, 8);
dep_decode_num_unsigned!(u128, 16);

impl NestedDecode for usize {
    #[inline]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        u32::dep_decode_or_handle_err(input, h).map(|num| num as usize)
    }
}

macro_rules! top_decode_num_unsigned {
    ($ty:ty, $bounds_ty:ty) => {
        impl TopDecode for $ty {
            #[inline]
            fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
            where
                I: TopDecodeInput,
                H: DecodeErrorHandler,
            {
                let arg_u64 = input.into_u64(h)?;
                let max = <$bounds_ty>::MAX as u64;
                if arg_u64 > max {
                    Err(h.handle_error(DecodeError::INPUT_TOO_LONG))
                } else {
                    Ok(arg_u64 as $ty)
                }
            }
        }
    };
}

top_decode_num_unsigned!(u8, u8);
top_decode_num_unsigned!(u16, u16);
top_decode_num_unsigned!(u32, u32);
top_decode_num_unsigned!(usize, u32); // even if usize can be 64 bits on some platforms, we always deserialize as max 32 bits
top_decode_num_unsigned!(u64, u64);

impl TopDecode for u128 {
    #[inline]
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut buffer = [0u8; 16];
        input.into_max_size_buffer_align_right(&mut buffer, h)?;
        Ok(u128::from_be_bytes(buffer))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        DecodeError, TopDecode,
        test_util::{check_dep_encode_decode, check_top_encode_decode},
    };

    #[test]
    fn test_top() {
        // unsigned zero
        check_top_encode_decode(0u8, &[]);
        check_top_encode_decode(0u16, &[]);
        check_top_encode_decode(0u32, &[]);
        check_top_encode_decode(0u64, &[]);
        check_top_encode_decode(0usize, &[]);
        check_top_encode_decode(0u128, &[]);
        // unsigned positive
        check_top_encode_decode(5u8, &[5]);
        check_top_encode_decode(5u16, &[5]);
        check_top_encode_decode(5u32, &[5]);
        check_top_encode_decode(5u64, &[5]);
        check_top_encode_decode(5usize, &[5]);
        check_top_encode_decode(5u128, &[5]);
    }

    #[test]
    fn test_dep() {
        // unsigned zero
        check_dep_encode_decode(0u8, &[0]);
        check_dep_encode_decode(0u16, &[0, 0]);
        check_dep_encode_decode(0u32, &[0, 0, 0, 0]);
        check_dep_encode_decode(0usize, &[0, 0, 0, 0]);
        check_dep_encode_decode(0u64, &[0, 0, 0, 0, 0, 0, 0, 0]);
        check_dep_encode_decode(0u128, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        // unsigned positive
        check_dep_encode_decode(5u8, &[5]);
        check_dep_encode_decode(5u16, &[0, 5]);
        check_dep_encode_decode(5u32, &[0, 0, 0, 5]);
        check_dep_encode_decode(5usize, &[0, 0, 0, 5]);
        check_dep_encode_decode(5u64, &[0, 0, 0, 0, 0, 0, 0, 5]);
        check_dep_encode_decode(5u128, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5]);
    }

    #[test]
    fn test_top_max() {
        check_top_encode_decode(u8::MAX, &[255]);
        check_top_encode_decode(u16::MAX, &[0xFF, 0xFF]);
        check_top_encode_decode(u32::MAX, &[0xFF, 0xFF, 0xFF, 0xFF]);
        check_top_encode_decode(u64::MAX, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        check_top_encode_decode(u128::MAX, &[0xFF; 16]);
        check_top_encode_decode(1u128 << 64, &[1, 0, 0, 0, 0, 0, 0, 0, 0]);
        check_top_encode_decode(u64::MAX as u128, &[0xFF; 8]);
    }

    #[test]
    fn test_top_u128_large() {
        // boundary around u64::MAX
        check_top_encode_decode(u64::MAX as u128 + 1, &[1, 0, 0, 0, 0, 0, 0, 0, 0]);
        check_top_encode_decode(u64::MAX as u128 + 2, &[1, 0, 0, 0, 0, 0, 0, 0, 1]);
        // every byte length from 9 to 16: smallest and largest value
        for len in 9usize..=16 {
            let min = 1u128 << (8 * (len - 1));
            let mut expected_min = [0u8; 16];
            expected_min[0] = 1;
            check_top_encode_decode(min, &expected_min[..len]);

            let max = if len == 16 {
                u128::MAX
            } else {
                (1u128 << (8 * len)) - 1
            };
            check_top_encode_decode(max, &[0xFFu8; 16][..len]);
        }
        // mixed bytes, 1 EGLD * 10^6 (10^24)
        check_top_encode_decode(
            1_000_000_000_000_000_000_000_000u128,
            &[0xD3, 0xC2, 0x1B, 0xCE, 0xCC, 0xED, 0xA1, 0x00, 0x00, 0x00],
        );
        // non-zero low half, zero in the middle
        check_top_encode_decode(
            0x0100_0000_0000_0000_0000_0000_0000_0001u128,
            &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        );
    }

    #[test]
    fn test_dep_max() {
        check_dep_encode_decode(u8::MAX, &[0xFF]);
        check_dep_encode_decode(u16::MAX, &[0xFF, 0xFF]);
        check_dep_encode_decode(u32::MAX, &[0xFF, 0xFF, 0xFF, 0xFF]);
        check_dep_encode_decode(u64::MAX, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        check_dep_encode_decode(
            u128::MAX,
            &[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ],
        );
    }

    #[test]
    fn test_top_decode_out_of_range() {
        // 256 as u8 should fail
        assert_eq!(
            u8::top_decode(&[1u8, 0u8][..]),
            Err(DecodeError::INPUT_TOO_LONG),
        );
        // u16::MAX + 1 as u16 should fail
        assert_eq!(
            u16::top_decode(&[1u8, 0, 0][..]),
            Err(DecodeError::INPUT_TOO_LONG),
        );
        // 17 bytes as u128 should fail
        assert_eq!(
            u128::top_decode(&[1u8; 17][..]),
            Err(DecodeError::INPUT_TOO_LONG),
        );
    }
}
