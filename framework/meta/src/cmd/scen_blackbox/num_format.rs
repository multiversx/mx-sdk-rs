use multiversx_sc_scenario::num_bigint::{BigInt, BigUint};

/// Formats an unsigned numeric value from big-endian bytes.
/// Uses BigUint for parsing; emits `u64` suffix if it fits, `u128` otherwise.
pub fn format_unsigned(bytes: &[u8], abi_type: &str) -> String {
    let val = BigUint::from_bytes_be(bytes);
    let suffix = match abi_type {
        "BigUint" => {
            if val <= BigUint::from(u64::MAX) {
                "u64"
            } else {
                "u128"
            }
        }
        other => other,
    };
    let digits = val.to_string();
    let formatted = insert_digit_separators(&digits);
    format!("{formatted}{suffix}")
}

/// Formats a signed numeric value from big-endian two's complement bytes.
/// Uses BigInt for parsing; emits `i64` suffix if it fits, `i128` otherwise.
pub fn format_signed(bytes: &[u8], abi_type: &str) -> String {
    let val = BigInt::from_signed_bytes_be(bytes);
    let suffix = match abi_type {
        "BigInt" => {
            if val >= BigInt::from(i64::MIN) && val <= BigInt::from(i64::MAX) {
                "i64"
            } else {
                "i128"
            }
        }
        other => other,
    };
    let digits = val.to_string();
    let formatted = insert_digit_separators(&digits);
    format!("{formatted}{suffix}")
}

/// Inserts `_` separators every 3 digits in a numeric string.
/// Handles an optional leading `-` for negative numbers.
pub fn insert_digit_separators(s: &str) -> String {
    let (sign, digits) = if let Some(rest) = s.strip_prefix('-') {
        ("-", rest)
    } else {
        ("", s)
    };

    if digits.len() <= 3 {
        return s.to_string();
    }

    let mut result = String::with_capacity(sign.len() + digits.len() + digits.len() / 3);
    result.push_str(sign);

    let first_group = digits.len() % 3;
    if first_group > 0 {
        result.push_str(&digits[..first_group]);
    }
    for chunk in digits.as_bytes()[first_group..].chunks(3) {
        if !result.is_empty() && !result.ends_with('-') {
            result.push('_');
        }
        result.push_str(std::str::from_utf8(chunk).unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── insert_digit_separators ──

    #[test]
    fn separators_short_numbers_unchanged() {
        assert_eq!(insert_digit_separators("0"), "0");
        assert_eq!(insert_digit_separators("1"), "1");
        assert_eq!(insert_digit_separators("42"), "42");
        assert_eq!(insert_digit_separators("999"), "999");
    }

    #[test]
    fn separators_thousands() {
        assert_eq!(insert_digit_separators("1000"), "1_000");
        assert_eq!(insert_digit_separators("12345"), "12_345");
        assert_eq!(insert_digit_separators("123456"), "123_456");
        assert_eq!(insert_digit_separators("1234567"), "1_234_567");
        assert_eq!(insert_digit_separators("1000000000"), "1_000_000_000");
    }

    #[test]
    fn separators_negative() {
        assert_eq!(insert_digit_separators("-1"), "-1");
        assert_eq!(insert_digit_separators("-999"), "-999");
        assert_eq!(insert_digit_separators("-1000"), "-1_000");
        assert_eq!(insert_digit_separators("-1234567"), "-1_234_567");
    }

    // ── format_unsigned ──

    #[test]
    fn unsigned_zero() {
        assert_eq!(format_unsigned(&[], "u64"), "0u64");
        assert_eq!(format_unsigned(&[0], "u32"), "0u32");
    }

    #[test]
    fn unsigned_small_values() {
        assert_eq!(format_unsigned(&[1], "u8"), "1u8");
        assert_eq!(format_unsigned(&[0xFF], "u8"), "255u8");
        assert_eq!(format_unsigned(&[0x01, 0x00], "u16"), "256u16");
    }

    #[test]
    fn unsigned_large_u64() {
        // u64::MAX = 18_446_744_073_709_551_615
        let bytes = u64::MAX.to_be_bytes();
        assert_eq!(
            format_unsigned(&bytes, "u64"),
            "18_446_744_073_709_551_615u64"
        );
    }

    #[test]
    fn unsigned_biguint_fits_u64() {
        assert_eq!(format_unsigned(&[42], "BigUint"), "42u64");
        assert_eq!(format_unsigned(&[0x01, 0x00], "BigUint"), "256u64");
    }

    #[test]
    fn unsigned_biguint_needs_u128() {
        // u64::MAX + 1 = 18_446_744_073_709_551_616
        let val = (u64::MAX as u128) + 1;
        let bytes = val.to_be_bytes();
        // strip leading zeros
        let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
        assert_eq!(
            format_unsigned(&bytes[start..], "BigUint"),
            "18_446_744_073_709_551_616u128"
        );
    }

    #[test]
    fn unsigned_with_separators() {
        // 1_000_000
        let bytes = 1_000_000u32.to_be_bytes();
        let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
        assert_eq!(format_unsigned(&bytes[start..], "u32"), "1_000_000u32");
    }

    // ── format_signed ──

    #[test]
    fn signed_zero() {
        assert_eq!(format_signed(&[], "i64"), "0i64");
        assert_eq!(format_signed(&[0], "i32"), "0i32");
    }

    #[test]
    fn signed_positive() {
        assert_eq!(format_signed(&[1], "i8"), "1i8");
        assert_eq!(format_signed(&[0x7F], "i8"), "127i8");
        assert_eq!(format_signed(&[0x01, 0x00], "i16"), "256i16");
    }

    #[test]
    fn signed_negative() {
        // -1 in two's complement = 0xFF
        assert_eq!(format_signed(&[0xFF], "i8"), "-1i8");
        // -128 in two's complement = 0x80
        assert_eq!(format_signed(&[0x80], "i8"), "-128i8");
        // -256 in two's complement = 0xFF, 0x00
        assert_eq!(format_signed(&[0xFF, 0x00], "i16"), "-256i16");
    }

    #[test]
    fn signed_bigint_fits_i64() {
        assert_eq!(format_signed(&[42], "BigInt"), "42i64");
        assert_eq!(format_signed(&[0xFF], "BigInt"), "-1i64");
    }

    #[test]
    fn signed_bigint_needs_i128() {
        // i64::MAX + 1 = 9_223_372_036_854_775_808
        // Must keep leading 0x00 so BigInt treats it as positive
        let val = (i64::MAX as i128) + 1;
        let bytes = val.to_be_bytes();
        // Find the first non-zero byte, but keep one byte before it if high bit is set
        let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
        let start = if start < bytes.len() && bytes[start] & 0x80 != 0 {
            start.saturating_sub(1)
        } else {
            start
        };
        assert_eq!(
            format_signed(&bytes[start..], "BigInt"),
            "9_223_372_036_854_775_808i128"
        );
    }

    #[test]
    fn signed_with_separators() {
        // -1_000_000 in two's complement big-endian
        let val = -1_000_000i32;
        let bytes = val.to_be_bytes();
        assert_eq!(format_signed(&bytes, "i32"), "-1_000_000i32");
    }
}
