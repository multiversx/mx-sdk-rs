use std::cmp::Ordering;

use elrond_wasm::{
    api::{Handle, ManagedBufferApi},
    types::{heap::Address, ManagedBuffer, ManagedType},
};
use num_traits::Zero;

use crate::{num_bigint, num_bigint::Sign, DebugApi};

impl DebugApi {
    pub fn insert_new_managed_buffer(&self, value: Vec<u8>) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.managed_buffer_map.insert_new_handle(value)
    }

    pub fn insert_new_managed_buffer_old(&self, value: Vec<u8>) -> ManagedBuffer<Self> {
        let mut managed_types = self.m_types_borrow_mut();
        let handle = managed_types.managed_buffer_map.insert_new_handle(value);
        ManagedBuffer::from_raw_handle(handle)
    }

    pub fn address_handle_to_value(&self, address_handle: Handle) -> Address {
        let mut address = Address::zero();
        self.mb_load_slice(address_handle, 0, address.as_mut())
            .unwrap();
        address
    }

    pub fn insert_new_big_uint(&self, value: num_bigint::BigUint) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_int_map.insert_new_handle(value.into())
    }

    pub fn insert_new_big_uint_old(
        &self,
        value: num_bigint::BigUint,
    ) -> elrond_wasm::types::BigUint<Self> {
        let mut managed_types = self.m_types_borrow_mut();
        let handle = managed_types.big_int_map.insert_new_handle(value.into());
        elrond_wasm::types::BigUint::from_raw_handle(handle)
    }

    pub fn insert_new_big_uint_zero(&self) -> Handle {
        self.insert_new_big_uint(num_bigint::BigUint::zero())
    }

    pub fn big_uint_handle_to_value(&self, bu_handle: Handle) -> num_bigint::BigUint {
        let managed_types = self.m_types_borrow();
        managed_types.big_int_map.get(bu_handle).magnitude().clone()
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
