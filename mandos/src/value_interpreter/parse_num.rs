use super::prefixes::*;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::identities::Zero;

pub fn try_parse_fixed_width(s: &str) -> Option<Vec<u8>> {
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

    None
}

pub fn parse_num(s: &str) -> Vec<u8> {
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

fn parse_fixed_width_signed(s: &str, length: usize) -> Vec<u8> {
    if let Some(stripped) = s.strip_prefix('-') {
        let mut result = vec![0xffu8; length];
        let bi = BigInt::from_bytes_be(Sign::Minus, parse_unsigned(stripped).as_slice());
        let bytes = bi.to_signed_bytes_be();
        assert!(
            bytes.len() <= length,
            "representation of {s} does not fit in {length} bytes"
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
        assert!(
            result.is_empty() || result[0] >> 7 != 1,
            "representation of {s} does not fit in {length} bytes"
        );
        result
    }
}

fn parse_fixed_width_unsigned(s: &str, length: usize) -> Vec<u8> {
    let parsed = parse_unsigned(s);
    assert!(
        parsed.len() <= length,
        "representation of {s} does not fit in {length} bytes"
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

fn parse_unsigned(s: &str) -> Vec<u8> {
    let clean = s.replace(&['_', ','][..], "");
    if clean.starts_with("0x") || clean.starts_with("0X") {
        let clean = &clean[2..];
        return if clean.len() % 2 == 0 {
            hex::decode(clean).unwrap()
        } else {
            let even_bytes = format!("0{clean}");
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
        panic!("Could not parse base 10 number: {clean}")
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
