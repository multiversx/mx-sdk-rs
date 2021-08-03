use crate::api::{BigIntApi, Handle};
use crate::types::BoxedBytes;

impl BigIntApi for super::UncallableApi {
    fn new(&self, _value: i64) -> Handle {
        unreachable!()
    }

    fn signed_byte_length(&self, _x: Handle) -> Handle {
        unreachable!()
    }

    fn get_signed_bytes(&self, _reference: Handle) -> BoxedBytes {
        unreachable!()
    }

    fn set_signed_bytes(&self, _destination: Handle, _bytes: &[u8]) {
        unreachable!()
    }

    fn is_int64(&self, _reference: Handle) -> Handle {
        unreachable!()
    }

    fn get_int64(&self, _reference: Handle) -> i64 {
        unreachable!()
    }

    fn add(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn sub(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn mul(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn t_div(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn t_mod(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn pow(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn abs(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }

    fn neg(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }

    fn sign(&self, _x: Handle) -> i32 {
        unreachable!()
    }

    fn cmp(&self, _x: Handle, _y: Handle) -> i32 {
        unreachable!()
    }
}
