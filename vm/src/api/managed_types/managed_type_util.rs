use multiversx_sc::{
    api::{HandleConstraints, HandleTypeInfo, ManagedBufferApiImpl},
    types::{heap::Address, ManagedBuffer, ManagedType},
};
use num_traits::Zero;

use crate::{num_bigint, DebugApi};

impl DebugApi {
    pub fn insert_new_managed_buffer(
        &self,
        value: Vec<u8>,
    ) -> <Self as HandleTypeInfo>::ManagedBufferHandle {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.managed_buffer_map.insert_new_handle(value)
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
