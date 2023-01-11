use std::cmp::Ordering;

use multiversx_sc::{
    api::{HandleTypeInfo, ManagedBufferApi},
    types::{heap::Address, ManagedBuffer, ManagedType},
};
use num_traits::Zero;

use crate::{num_bigint, num_bigint::Sign, DebugApi};

impl DebugApi {
    pub fn insert_new_managed_buffer(
        &self,
        value: Vec<u8>,
    ) -> <Self as HandleTypeInfo>::ManagedBufferHandle {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.managed_buffer_map.insert_new_handle(value)
    }

    pub fn insert_new_managed_buffer_old(&self, value: Vec<u8>) -> ManagedBuffer<Self> {
        let handle = self.insert_new_managed_buffer(value);
        ManagedBuffer::from_handle(handle)
    }

    pub fn mb_handle_to_value(
        &self,
        mb_handle: <Self as HandleTypeInfo>::ManagedBufferHandle,
    ) -> Vec<u8> {
        ManagedBuffer::<Self>::from_handle(mb_handle)
            .to_boxed_bytes()
            .into_vec()
    }

    pub fn address_handle_to_value(
        &self,
        address_handle: <Self as HandleTypeInfo>::ManagedBufferHandle,
    ) -> Address {
        let mut address = Address::zero();
        self.mb_load_slice(address_handle, 0, address.as_mut())
            .unwrap();
        address
    }

    pub fn insert_new_big_uint(
        &self,
        value: num_bigint::BigUint,
    ) -> <Self as HandleTypeInfo>::BigIntHandle {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_int_map.insert_new_handle(value.into())
    }

    pub fn set_big_uint(
        &self,
        handle: <Self as HandleTypeInfo>::BigIntHandle,
        value: num_bigint::BigUint,
    ) {
        let mut managed_types = handle.context.m_types_borrow_mut();
        managed_types
            .big_int_map
            .insert(handle.get_raw_handle_unchecked(), value.into())
    }

    pub fn insert_new_big_uint_old(
        &self,
        value: num_bigint::BigUint,
    ) -> multiversx_sc::types::BigUint<Self> {
        let mut managed_types = self.m_types_borrow_mut();
        let handle = managed_types.big_int_map.insert_new_handle(value.into());
        multiversx_sc::types::BigUint::from_handle(handle)
    }

    pub fn insert_new_big_uint_zero(&self) -> <Self as HandleTypeInfo>::BigIntHandle {
        self.insert_new_big_uint(num_bigint::BigUint::zero())
    }

    pub fn big_uint_handle_to_value(
        &self,
        bu_handle: <Self as HandleTypeInfo>::BigIntHandle,
    ) -> num_bigint::BigUint {
        let managed_types = bu_handle.context.m_types_borrow();
        managed_types
            .big_int_map
            .get(bu_handle.get_raw_handle_unchecked())
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
