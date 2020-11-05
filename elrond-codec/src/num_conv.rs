use crate::nested_ser_output::NestedEncodeOutput;

/// Adds number to output buffer.
/// No argument generics here, because we want the executable binary as small as possible.
/// Smaller types need to be converted to u64 before using this function.
/// TODO: there might be a quicker version of this using transmute + reverse bytes.
pub fn using_encoded_number<F: FnOnce(&[u8])>(
	x: u64,
	size_in_bits: usize,
	signed: bool,
	mut compact: bool,
	f: F,
) {
	let mut result = [0u8; 8];
	let mut result_size = 0usize;
	let negative = compact && // only relevant when compact flag
		signed &&  // only possible when signed flag
		x >> (size_in_bits - 1) & 1 == 1; // compute by checking first bit

	let irrelevant_byte = if negative { 0xffu8 } else { 0x00u8 };
	let mut bit_offset = size_in_bits as isize - 8;
	while bit_offset >= 0 {
		// going byte by byte from most to least significant
		let byte = (x >> (bit_offset as usize) & 0xffu64) as u8;

		if compact {
			// compact means ignoring irrelvant leading bytes
			// that is 000... for positives and fff... for negatives
			if byte != irrelevant_byte {
				result[result_size] = byte;
				result_size += 1;
				compact = false;
			}
		} else {
			result[result_size] = byte;
			result_size += 1;
		}

		bit_offset -= 8;
	}

	f(&result[0..result_size])
}

pub fn top_encode_number_to_output<O: NestedEncodeOutput>(output: &mut O, x: u64, signed: bool) {
	let bytes_be: [u8; 8] = x.to_be_bytes();
	if x == 0 {
		// 0 is a special case
		return;
	}

	if signed && x == u64::MAX {
		// -1 is a special case
		output.push_byte(0xffu8);
		return;
	}

	let negative = signed &&  // only possible when signed flag
		bytes_be[0] > 0x7fu8; // most significant bit is 1

	let irrelevant_byte = if negative { 0xffu8 } else { 0x00u8 };

	let mut offset = 0usize;
	while bytes_be[offset] == irrelevant_byte {
		debug_assert!(offset < 7);
		offset += 1;
	}

	if signed && bytes_be[offset] >> 7 != negative as u8 {
		debug_assert!(offset > 0);
		offset -= 1;
	}

	output.write(&bytes_be[offset..]);
}

/// Handles both signed and unsigned of any length.
/// No generics here, because we want the executable binary as small as possible.
#[inline(never)]
pub fn bytes_to_number(bytes: &[u8], signed: bool) -> u64 {
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
