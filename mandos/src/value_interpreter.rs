use num_bigint::{BigInt, BigUint, Sign};
use num_traits::identities::Zero;
use super::context::*;
use super::value_raw::*;

const STR_PREFIXES: [&str; 3] = ["str:", "``", "''"];

const ADDR_PREFIX: &str = "address:";
const FILE_PREFIX: &str = "file:";
// const keccak256Prefix = "keccak256:"

const U64_PREFIX: &str = "u64:";
const U32_PREFIX: &str = "u32:";
const U16_PREFIX: &str = "u16:";
const U8_PREFIX : &str = "u8:";
const I64_PREFIX: &str = "i64:";
const I32_PREFIX: &str = "i32:";
const I16_PREFIX: &str = "i16:";
const I8_PREFIX : &str = "i8:";

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
        }
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
        return Vec::new()
    }

    for str_prefix in STR_PREFIXES.iter() {
        if s.starts_with(str_prefix) {
            return s[str_prefix.len() .. ].as_bytes().to_vec()
        }
    }
    
    if s.starts_with(ADDR_PREFIX) {
        return address(&s[ADDR_PREFIX.len() .. ]);
    }

    if s.starts_with(FILE_PREFIX) {
        return s.as_bytes().to_vec();
    }

    if let Some(fixed_width) = try_parse_fixed_width(s) {
        return fixed_width;
    }

    if s.starts_with('+') {
        let bi = BigInt::from_bytes_be(Sign::Plus, parse_unsigned(&s[1..]).as_slice());
        return big_int_to_bytes_be(&bi);
    }

    if s.starts_with('-') {
        let bi = BigInt::from_bytes_be(Sign::Minus, parse_unsigned(&s[1..]).as_slice());
        return big_int_to_bytes_be(&bi);
    }

    parse_unsigned(s)
}

fn try_parse_fixed_width(s: &str) -> Option<Vec<u8>> {
    if s.starts_with(U64_PREFIX) {
        return Some(parse_fixed_width_unsigned(&s[U64_PREFIX.len()..], 8));
    }

    if s.starts_with(U32_PREFIX) {
        return Some(parse_fixed_width_unsigned(&s[U32_PREFIX.len()..], 4));
    }

    if s.starts_with(U16_PREFIX) {
        return Some(parse_fixed_width_unsigned(&s[U16_PREFIX.len()..], 2));
    }

    if s.starts_with(U8_PREFIX) {
        return Some(parse_fixed_width_unsigned(&s[U8_PREFIX.len()..], 1));
    }

    if s.starts_with(I64_PREFIX) {
        return Some(parse_fixed_width_signed(&s[I64_PREFIX.len()..], 8));
    }

    if s.starts_with(I32_PREFIX) {
        return Some(parse_fixed_width_signed(&s[I32_PREFIX.len()..], 4));
    }

    if s.starts_with(I16_PREFIX) {
        return Some(parse_fixed_width_signed(&s[I16_PREFIX.len()..], 2));
    }

    if s.starts_with(I8_PREFIX) {
        return Some(parse_fixed_width_signed(&s[I8_PREFIX.len()..], 1));
    }

    None
}

fn parse_fixed_width_signed(s: &str, length: usize) -> Vec<u8> {
    if s.starts_with('-') {
        let mut result = vec![0xffu8; length];
        let bi = BigInt::from_bytes_be(Sign::Minus, parse_unsigned(&s[1..]).as_slice());
        let bytes = bi.to_signed_bytes_be();
        assert!(
            bytes.len() <= length,
            "representation of {} does not fit in {} bytes",
            s, length);
        let offset = length - bytes.len();
        if !bytes.is_empty() {
            result[offset..].clone_from_slice(&bytes[..]);
        }
        result
    } else {
        let s = if s.starts_with('+') { &s[1..] } else { s };
        let result = parse_fixed_width_unsigned(s, length);
        if !result.is_empty() && result[0] >> 7 == 1 {
            panic!("representation of {} does not fit in {} bytes",
                s, length);
        }
        result
    }
}

fn parse_fixed_width_unsigned(s: &str, length: usize) -> Vec<u8> {
    let parsed = parse_unsigned(s);
    assert!(
        parsed.len() <= length,
        "representation of {} does not fit in {} bytes",
        s, length);

    let mut result = vec![0u8; length];
    let offset = length - parsed.len();
    if !parsed.is_empty() {
        result[offset..].clone_from_slice(&parsed[..]);
    }
    result
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
        }
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

fn address(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    if bytes.len() > 32 {
        return bytes[.. 32].to_vec();
    }
    let mut result = vec![b'_'; 32];
    result[.. bytes.len()].copy_from_slice(bytes);
    result
}
