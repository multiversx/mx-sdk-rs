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
		buffer[0] > 0x7fu8; // most significant bit is 1

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
    let negative = signed && bytes[0] >> 7 == 1;
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
