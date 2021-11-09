use std::convert::TryInto;

use elrond_wasm::api::{BigIntApi, Handle, ManagedTypeApi};
use num_bigint::{BigInt, Sign};

use crate::DebugApi;

impl ManagedTypeApi for DebugApi {
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        let bytes = managed_types.managed_buffer_map.get(buffer_handle);
        let bi = BigInt::from_bytes_be(Sign::Plus, bytes.as_slice());
        managed_types.big_int_map.insert_new_handle(bi)
    }

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        let bytes = managed_types.managed_buffer_map.get(buffer_handle);
        let bi = BigInt::from_signed_bytes_be(bytes.as_slice());
        managed_types.big_int_map.insert_new_handle(bi)
    }

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle {
        let bi_bytes = self.bi_get_unsigned_bytes(big_int_handle);
        let mut managed_types = self.m_types_borrow_mut();
        managed_types
            .managed_buffer_map
            .insert_new_handle(bi_bytes.into_vec())
    }

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle {
        let bi_bytes = self.bi_get_signed_bytes(big_int_handle);
        let mut managed_types = self.m_types_borrow_mut();
        managed_types
            .managed_buffer_map
            .insert_new_handle(bi_bytes.into_vec())
    }

    fn mb_to_big_float(&self, buffer_handle: Handle) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        let mb_bytes = managed_types.managed_buffer_map.get(buffer_handle);
        let float_bytes: [u8; 8] = mb_bytes
            .as_slice()
            .try_into()
            .expect("slice with incorrect length");
        let new_bf = f64::from_be_bytes(float_bytes);
        managed_types.big_float_map.insert_new_handle(new_bf)
    }

    fn mb_from_big_float(&self, big_float_handle: Handle) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        let bf = managed_types.big_float_map.get(big_float_handle);
        let bf_bytes = bf.to_be_bytes();
        managed_types
            .managed_buffer_map
            .insert_new_handle(Vec::<u8>::from(bf_bytes))
    }
}
