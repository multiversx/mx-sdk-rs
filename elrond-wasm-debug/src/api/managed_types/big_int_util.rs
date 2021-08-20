use std::cmp::Ordering;

use num_bigint::{BigInt, Sign};

pub fn big_int_to_i64(bi: &BigInt) -> Option<i64> {
    let (sign, digits) = bi.to_u64_digits();
    match sign {
        Sign::NoSign => Some(0),
        Sign::Plus => {
            if digits.len() == 1 {
                let as_u64 = digits[0];
                if as_u64 <= i64::MAX as u64 {
                    Some(as_u64 as i64)
                } else {
                    None
                }
            } else {
                None
            }
        },
        Sign::Minus => {
            if digits.len() == 1 {
                let as_u64 = digits[0];
                match as_u64.cmp(&0x8000000000000000u64) {
                    Ordering::Less => Some(-(as_u64 as i64)),
                    Ordering::Equal => Some(i64::MIN),
                    Ordering::Greater => None,
                }
            } else {
                None
            }
        },
    }
}
