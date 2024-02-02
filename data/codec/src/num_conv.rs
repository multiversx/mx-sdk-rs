pub type TopEncodeNumberBuffer = [u8; 8];

/// This buffer is needed to provide some underlying structure on stack off which to build a variable-length slice.
///
/// Its length is 9 (one more than necessary, to elegantly deal with the edge case "-1").
pub const fn top_encode_number_buffer() -> TopEncodeNumberBuffer {
    [0u8; 8]
}

/// Encodes number to minimimum number of bytes (top-encoding).
///
/// Smaller types need to be converted to u64 before using this function.
///
/// No generics here, we avoid monomorphization to make the SC binary as small as possible.
pub fn top_encode_number(x: u64, signed: bool, buffer: &mut TopEncodeNumberBuffer) -> &[u8] {
    let offset = fill_buffer_find_offset(x, signed, buffer);

    debug_assert!(offset < 9);

    unsafe { buffer.get_unchecked(offset..) }
}

/// At the same time fills the buffer,
/// and performs the algorithm that tells us how many bytes can be skipped.
///
/// Everything done in one function instead of 2, to avoid any unwanted bounds checks.
///
/// This function is hyper-optimized to not contain any jumps. There are no ifs or loops in this,
/// the entire algorithm is performed via arithmetic, boolean and bitwise operations.
fn fill_buffer_find_offset(x: u64, signed: bool, buffer: &mut TopEncodeNumberBuffer) -> usize {
    let b0 = (x >> 56 & 0xff) as u8;

    let negative = signed && msbit_is_one(b0);
    let skippable_byte = skippable_byte(negative);

    let mut offset = 0usize;
    let mut cursor = 1usize;

    change_one_to_zero_unless(&mut cursor, b0 == skippable_byte);
    offset += cursor;

    let b1 = (x >> 48 & 0xff) as u8;
    change_one_to_zero_unless(&mut cursor, b1 == skippable_byte);
    offset += cursor;

    let b2 = (x >> 40 & 0xff) as u8;
    change_one_to_zero_unless(&mut cursor, b2 == skippable_byte);
    offset += cursor;

    let b3 = (x >> 32 & 0xff) as u8;
    change_one_to_zero_unless(&mut cursor, b3 == skippable_byte);
    offset += cursor;

    let b4 = (x >> 24 & 0xff) as u8;
    change_one_to_zero_unless(&mut cursor, b4 == skippable_byte);
    offset += cursor;

    let b5 = (x >> 16 & 0xff) as u8;
    change_one_to_zero_unless(&mut cursor, b5 == skippable_byte);
    offset += cursor;

    let b6 = (x >> 8 & 0xff) as u8;
    change_one_to_zero_unless(&mut cursor, b6 == skippable_byte);
    offset += cursor;

    // The last byte: it can only get skipped for the number 0.
    // Writing `b7 == skippable_byte` instead would also have caught -1,
    // but that is an edge case where we do not want the last byte skipped.
    let b7 = (x & 0xff) as u8;
    change_one_to_zero_unless(&mut cursor, x == 0);
    offset += cursor;

    buffer[0] = b0;
    buffer[1] = b1;
    buffer[2] = b2;
    buffer[3] = b3;
    buffer[4] = b4;
    buffer[5] = b5;
    buffer[6] = b6;
    buffer[7] = b7;

    // For signed numbers, it can sometimes happen that we are skipping too many bytes,
    // and the most significant bit ends up different than what we started with.
    // In this case we need to backtrack one step.
    // e.g. 255: [255] -> [0, 255]
    // e.g. -129: [0x7f] -> [0xff, 0x7f]
    cursor = 1;
    change_one_to_zero_unless(&mut cursor, signed);
    change_one_to_zero_unless(&mut cursor, offset > 0);

    // The only time when the offset can be 8 (and thus out of bounds)
    // is for the number 0. Conveniently, for 0 all bytes are 0, so applying modulo 8 does not change the outcome.
    let byte_at_offset = buffer[offset % 8];

    // The main condition for stepping back one step: the most significant bit changed in the process.
    let msbit_corrupted = msbit_is_one(byte_at_offset) != msbit_is_one(b0);
    change_one_to_zero_unless(&mut cursor, msbit_corrupted);

    // According to this algorithm, it should be impossible to underflow
    // using wrapping_sub to avoid unnecessary underflow check
    debug_assert!(offset >= cursor);
    offset = offset.wrapping_sub(cursor);

    offset
}

/// Handles both top-encoding and nested-encoding, signed and unsigned, of any length.
///
/// The result needs to be validated to not exceed limits and then cast to the desired type.
///
/// No generics here, we avoid monomorphization to make the SC binary as small as possible.
pub fn universal_decode_number(bytes: &[u8], signed: bool) -> u64 {
    if bytes.is_empty() {
        return 0;
    }
    let negative = signed && msbit_is_one(bytes[0]);
    let mut result = if negative {
        // start with all bits set to 1,
        // to ensure that if there are fewer bytes than the result type width,
        // the leading bits will be 1 instead of 0
        u64::MAX
    } else {
        0u64
    };
    for byte in bytes.iter() {
        result <<= 8;
        result |= *byte as u64;
    }
    result
}

/// Most significant bit is 1.
#[inline]
fn msbit_is_one(byte: u8) -> bool {
    byte >= 0b1000_0000u8
}

#[inline]
fn change_one_to_zero_unless(x: &mut usize, condition: bool) {
    debug_assert!(*x <= 1);
    *x &= condition as usize;
}

/// For negative = true, yields 0xff.
///
/// For negative = false, yields 0x00.
///
/// Has no if, doesn't branch.
#[inline]
fn skippable_byte(negative: bool) -> u8 {
    0u8.wrapping_sub(negative as u8)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_to_zero_unless() {
        let mut x = 1;
        change_one_to_zero_unless(&mut x, true);
        assert_eq!(x, 1);
        change_one_to_zero_unless(&mut x, false);
        assert_eq!(x, 0);
    }

    #[test]
    fn test_skippable_byte() {
        assert_eq!(skippable_byte(true), 0xffu8);
        assert_eq!(skippable_byte(false), 0x00u8);
    }

    /// Only checks the filling out of the buffer.
    #[test]
    fn test_populate_buffer() {
        let mut buffer = top_encode_number_buffer();
        let _ = fill_buffer_find_offset(0x12345678abcdef12, false, &mut buffer);
        assert_eq!(buffer, [0x12, 0x34, 0x56, 0x78, 0xab, 0xcd, 0xef, 0x12]);
    }

    fn test_encode_decode(x: u64, signed: bool, bytes: &[u8]) {
        let mut buffer = top_encode_number_buffer();
        assert_eq!(
            top_encode_number(x, signed, &mut buffer),
            bytes,
            "encode failed for {x}"
        );
        assert_eq!(
            universal_decode_number(bytes, signed,),
            x,
            "decode failed for {x}"
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_top_encode_number() {
        // unsigned
        test_encode_decode(0x00, false, &[]);
        test_encode_decode(0x01, false, &[1]);
        test_encode_decode(0x7f, false, &[0x7f]);
        test_encode_decode(0x80, false, &[0x80]);
        test_encode_decode(0xff, false, &[0xff]);
        test_encode_decode(0x0100, false, &[1, 0]);
        test_encode_decode(0xff00, false, &[0xff, 0]);
        test_encode_decode(0xffff, false, &[0xff, 0xff]);
        test_encode_decode(0xffffffffffffffff, false, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

        // signed, positive
        test_encode_decode(0x00, true, &[]);
        test_encode_decode(0x01, true, &[1]);
        test_encode_decode(0x7f, true, &[0x7f]);
        test_encode_decode(0x80, true, &[0x00, 0x80]);
        test_encode_decode(0x0100, true, &[1, 0]);
        test_encode_decode(0xff00, true, &[0x00, 0xff, 0]);
        test_encode_decode(0xffff, true, &[0x00, 0xff, 0xff]);
        test_encode_decode(0x7fffffffffffffff, true, &[0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        test_encode_decode(0x8000000000000000, true, &[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // signed, negative
        test_encode_decode(-1i64 as u64, true, &[0xff]);
        test_encode_decode(-2i64 as u64, true, &[0xfe]);
        test_encode_decode(-126i64 as u64, true, &[0x82]);
        test_encode_decode(-127i64 as u64, true, &[0x81]);
        test_encode_decode(-128i64 as u64, true, &[0x80]);
        test_encode_decode(-129i64 as u64, true, &[0xff, 0x7f]);
        test_encode_decode(-255i64 as u64, true, &[0xff, 0x01]);
        test_encode_decode(-256i64 as u64, true, &[0xff, 0x00]);
        test_encode_decode(-257i64 as u64, true, &[0xfe, 0xff]);
    }
}
