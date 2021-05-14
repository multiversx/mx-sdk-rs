use super::context::*;
use super::functions::*;
use super::value_raw::*;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::identities::Zero;

const STR_PREFIXES: &[&str] = &["str:", "``", "''"];

const ADDR_PREFIX: &str = "address:";
const SC_ADDR_PREFIX: &str = "sc:";
const FILE_PREFIX: &str = "file:";
const KECCAK256_PREFIX: &str = "keccak256:";

const U64_PREFIX: &str = "u64:";
const U32_PREFIX: &str = "u32:";
const U16_PREFIX: &str = "u16:";
const U8_PREFIX: &str = "u8:";
const I64_PREFIX: &str = "i64:";
const I32_PREFIX: &str = "i32:";
const I16_PREFIX: &str = "i16:";
const I8_PREFIX: &str = "i8:";

const BIGUINT_PREFIX: &str = "biguint:";
const NESTED_PREFIX: &str = "nested:";

pub fn interpret_subtree(vst: &ValueSubTree, context: &InterpreterContext) -> Vec<u8> {
	match vst {
		ValueSubTree::Str(s) => interpret_string(s, context),
		ValueSubTree::List(l) => {
			let mut concat = Vec::<u8>::new();
			for item in l.iter() {
				concat.extend_from_slice(interpret_subtree(item, context).as_slice());
			}
			concat
		},
		ValueSubTree::Map(m) => {
			let mut concat = Vec::<u8>::new();
			for (_, value) in m.iter() {
				concat.extend_from_slice(interpret_subtree(value, context).as_slice());
			}
			concat
		},
	}
}

pub fn interpret_string(s: &str, context: &InterpreterContext) -> Vec<u8> {
	if s.is_empty() {
		return Vec::new();
	}

	// concatenate values of different formats
	let split_parts: Vec<_> = s.split('|').collect();
	if split_parts.len() > 1 {
		let mut result = Vec::<u8>::new();
		for part in split_parts.iter() {
			result.extend_from_slice(interpret_string(part, context).as_slice());
		}
		return result;
	}

	if s == "true" {
		return [1u8].to_vec();
	}

	if s == "false" {
		return Vec::new();
	}

	for str_prefix in STR_PREFIXES.iter() {
		if let Some(stripped) = s.strip_prefix(str_prefix) {
			return stripped.as_bytes().to_vec();
		}
	}

	if let Some(stripped) = s.strip_prefix(ADDR_PREFIX) {
		return address_expression(stripped);
	}

	if let Some(stripped) = s.strip_prefix(SC_ADDR_PREFIX) {
		return sc_address_expression(stripped);
	}

	if s.starts_with(FILE_PREFIX) {
		return s.as_bytes().to_vec();
	}

	if let Some(stripped) = s.strip_prefix(KECCAK256_PREFIX) {
		let arg = interpret_string(stripped, context);
		return keccak256(arg.as_slice());
	}

	if let Some(fixed_width) = try_parse_fixed_width(s, context) {
		return fixed_width;
	}

	if let Some(stripped) = s.strip_prefix('+') {
		let bi = BigInt::from_bytes_be(Sign::Plus, parse_unsigned(stripped).as_slice());
		return big_int_to_bytes_be(&bi);
	}

	if let Some(stripped) = s.strip_prefix('-') {
		let bi = BigInt::from_bytes_be(Sign::Minus, parse_unsigned(stripped).as_slice());
		return big_int_to_bytes_be(&bi);
	}

	parse_unsigned(s)
}

fn try_parse_fixed_width(s: &str, context: &InterpreterContext) -> Option<Vec<u8>> {
	if let Some(stripped) = s.strip_prefix(U64_PREFIX) {
		return Some(parse_fixed_width_unsigned(stripped, 8));
	}

	if let Some(stripped) = s.strip_prefix(U32_PREFIX) {
		return Some(parse_fixed_width_unsigned(stripped, 4));
	}

	if let Some(stripped) = s.strip_prefix(U16_PREFIX) {
		return Some(parse_fixed_width_unsigned(stripped, 2));
	}

	if let Some(stripped) = s.strip_prefix(U8_PREFIX) {
		return Some(parse_fixed_width_unsigned(stripped, 1));
	}

	if let Some(stripped) = s.strip_prefix(I64_PREFIX) {
		return Some(parse_fixed_width_signed(stripped, 8));
	}

	if let Some(stripped) = s.strip_prefix(I32_PREFIX) {
		return Some(parse_fixed_width_signed(stripped, 4));
	}

	if let Some(stripped) = s.strip_prefix(I16_PREFIX) {
		return Some(parse_fixed_width_signed(stripped, 2));
	}

	if let Some(stripped) = s.strip_prefix(I8_PREFIX) {
		return Some(parse_fixed_width_signed(stripped, 1));
	}

	if let Some(stripped) = s.strip_prefix(BIGUINT_PREFIX) {
		return Some(parse_biguint(stripped));
	}

	if let Some(stripped) = s.strip_prefix(NESTED_PREFIX) {
		return Some(parse_nested(stripped, context));
	}

	None
}

fn parse_fixed_width_signed(s: &str, length: usize) -> Vec<u8> {
	if let Some(stripped) = s.strip_prefix('-') {
		let mut result = vec![0xffu8; length];
		let bi = BigInt::from_bytes_be(Sign::Minus, parse_unsigned(stripped).as_slice());
		let bytes = bi.to_signed_bytes_be();
		assert!(
			bytes.len() <= length,
			"representation of {} does not fit in {} bytes",
			s,
			length
		);
		let offset = length - bytes.len();
		if !bytes.is_empty() {
			result[offset..].clone_from_slice(&bytes[..]);
		}
		result
	} else {
		let s = if let Some(stripped) = s.strip_prefix('+') {
			stripped
		} else {
			s
		};
		let result = parse_fixed_width_unsigned(s, length);
		if !result.is_empty() && result[0] >> 7 == 1 {
			panic!("representation of {} does not fit in {} bytes", s, length);
		}
		result
	}
}

fn parse_fixed_width_unsigned(s: &str, length: usize) -> Vec<u8> {
	let parsed = parse_unsigned(s);
	assert!(
		parsed.len() <= length,
		"representation of {} does not fit in {} bytes",
		s,
		length
	);

	let mut result = vec![0u8; length];
	let offset = length - parsed.len();
	if !parsed.is_empty() {
		result[offset..].clone_from_slice(&parsed[..]);
	}
	result
}

fn parse_biguint(s: &str) -> Vec<u8> {
	let parsed = parse_unsigned(s);
	let encoded_length = (parsed.len() as u32).to_be_bytes();
	[&encoded_length[..], &parsed[..]].concat()
}

fn parse_nested(s: &str, context: &InterpreterContext) -> Vec<u8> {
	let parsed = interpret_string(s, context);
	let encoded_length = (parsed.len() as u32).to_be_bytes();
	[&encoded_length[..], &parsed[..]].concat()
}

fn parse_unsigned(s: &str) -> Vec<u8> {
	let clean = s.replace(&['_', ','][..], "");
	if clean.starts_with("0x") || clean.starts_with("0X") {
		let clean = &clean[2..];
		return if clean.len() % 2 == 0 {
			hex::decode(clean).unwrap()
		} else {
			let even_bytes = format!("0{}", clean);
			hex::decode(&even_bytes[..]).unwrap()
		};
	}

	if clean.starts_with("0b") || clean.starts_with("0B") {
		let clean = &clean[2..];
		if clean.is_empty() {
			return Vec::new();
		}
		let bu = BigUint::parse_bytes(clean.as_bytes(), 2).unwrap();
		return big_uint_to_bytes_be(&bu);
	}

	if let Some(bu) = BigUint::parse_bytes(clean.as_bytes(), 10) {
		big_uint_to_bytes_be(&bu)
	} else {
		panic!("Could not parse base 10 number: {}", clean)
	}
}

fn big_uint_to_bytes_be(bu: &BigUint) -> Vec<u8> {
	if bu.is_zero() {
		Vec::new()
	} else {
		bu.to_bytes_be()
	}
}

fn big_int_to_bytes_be(bi: &BigInt) -> Vec<u8> {
	if bi.is_zero() {
		Vec::new()
	} else {
		bi.to_signed_bytes_be()
	}
}
