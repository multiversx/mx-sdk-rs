/// Encodes number to minimimum number of bytes (top-encoding).
///
/// Smaller types need to be converted to u64 before using this function.
///
/// No generics here, we avoid monomorphization to make the SC binary as small as possible.
pub fn top_encode_number(x: u64, signed: bool, buffer: &mut [u8; 8]) -> &[u8] {
    *buffer = x.to_be_bytes();
    if x == 0 {
        // 0 is a special case
        return &[];
    }

    if signed && x == u64::MAX {
        // -1 is a special case
        // will return a single 0xFF byte
        return &buffer[7..];
    }

    let negative = signed &&  // only possible when signed flag
        msb_is_one(buffer[0]); // most significant bit is 1

    let irrelevant_byte = if negative { 0xffu8 } else { 0x00u8 };

    let mut offset = 0usize;
    while buffer[offset] == irrelevant_byte {
        debug_assert!(offset < 7);
        offset += 1;
    }

    if signed && buffer[offset] >> 7 != negative as u8 {
        debug_assert!(offset > 0);
        offset -= 1;
    }

    &buffer[offset..]
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
    let negative = signed && msb_is_one(bytes[0]);
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
fn msb_is_one(byte: u8) -> bool {
    byte >= 0b1000_0000u8
}

#[cfg(test)]
#[rustfmt::skip]
mod test {
    use super::*;

    fn test_encode_decode(x: u64, signed: bool, bytes: &[u8]) {
        let mut buffer = [0u8; 8];
        assert_eq!(top_encode_number(x, signed, &mut buffer), bytes);
        assert_eq!(universal_decode_number(bytes, signed,), x);
    }

    #[test]
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
