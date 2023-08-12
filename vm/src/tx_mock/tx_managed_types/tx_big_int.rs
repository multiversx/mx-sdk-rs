use std::cmp::Ordering;

use crate::types::RawHandle;
use num_bigint::Sign;
use num_traits::Zero;

use super::TxManagedTypes;

impl TxManagedTypes {
    pub fn bi_new_from_big_int(&mut self, value: num_bigint::BigInt) -> RawHandle {
        self.big_int_map.insert_new_handle_raw(value)
    }

    pub fn bi_overwrite(&mut self, destination: RawHandle, value: num_bigint::BigInt) {
        self.big_int_map.insert(destination, value);
    }

    pub fn bi_get(&self, handle: RawHandle) -> num_bigint::BigInt {
        self.big_int_map.get(handle).clone()
    }

    pub fn bu_get(&self, handle: RawHandle) -> num_bigint::BigUint {
        self.bi_get(handle)
            .try_into()
            .expect("number cannot be negative")
    }

    pub fn bi_to_i64(&self, handle: RawHandle) -> Option<i64> {
        let bi = self.bi_get(handle);
        big_int_to_i64(&bi)
    }

    pub fn bi_get_unsigned_bytes(&self, handle: RawHandle) -> Vec<u8> {
        let bi = self.bi_get(handle);
        if bi.is_zero() {
            Vec::new()
        } else {
            let (_, bytes) = bi.to_bytes_be();
            bytes
        }
    }

    pub fn bi_set_unsigned_bytes(&mut self, destination: RawHandle, bytes: &[u8]) {
        let bi = num_bigint::BigInt::from_bytes_be(Sign::Plus, bytes);
        self.bi_overwrite(destination, bi);
    }

    pub fn bi_get_signed_bytes(&self, handle: RawHandle) -> Vec<u8> {
        let bi = self.bi_get(handle);
        if bi.is_zero() {
            Vec::new()
        } else {
            bi.to_signed_bytes_be()
        }
    }

    pub fn bi_set_signed_bytes(&mut self, destination: RawHandle, bytes: &[u8]) {
        let bi = num_bigint::BigInt::from_signed_bytes_be(bytes);
        self.bi_overwrite(destination, bi);
    }
}

pub fn big_int_to_i64(bi: &num_bigint::BigInt) -> Option<i64> {
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
