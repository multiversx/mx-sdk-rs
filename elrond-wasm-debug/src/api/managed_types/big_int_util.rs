use std::cmp::Ordering;

use elrond_wasm::types::{ManagedBuffer, ManagedType};
use num_bigint::Sign;
use num_traits::Zero;

use crate::DebugApi;

impl DebugApi {
    pub fn insert_new_managed_buffer(&self, value: Vec<u8>) -> ManagedBuffer<Self> {
        let mut managed_types = self.m_types_borrow_mut();
        let handle = managed_types.managed_buffer_map.insert_new_handle(value);
        ManagedBuffer::from_raw_handle(handle)
    }

    pub fn insert_new_big_uint(
        &self,
        value: num_bigint::BigUint,
    ) -> elrond_wasm::types::BigUint<Self> {
        let mut managed_types = self.m_types_borrow_mut();
        let handle = managed_types.big_int_map.insert_new_handle(value.into());
        elrond_wasm::types::BigUint::from_raw_handle(handle)
    }

    pub fn insert_new_big_uint_zero(&self) -> elrond_wasm::types::BigUint<Self> {
        self.insert_new_big_uint(num_bigint::BigUint::zero())
    }

    pub fn big_uint_value(&self, bu: &elrond_wasm::types::BigUint<Self>) -> num_bigint::BigUint {
        let managed_types = self.m_types_borrow();
        managed_types
            .big_int_map
            .get(bu.get_raw_handle())
            .magnitude()
            .clone()
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
