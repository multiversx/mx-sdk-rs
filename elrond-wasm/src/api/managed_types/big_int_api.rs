use core::cmp::Ordering;

use crate::types::BoxedBytes;

use super::Handle;

/// Only used for sending sign information from the API.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

/// Definition of the BigInt type required by the API.
pub trait BigIntApi {
    fn new(&self, value: i64) -> Handle;

    fn new_zero(&self) -> Handle {
        self.new(0)
    }

    fn signed_byte_length(&self, x: Handle) -> Handle;
    fn get_signed_bytes(&self, reference: Handle) -> BoxedBytes;
    fn set_signed_bytes(&self, destination: Handle, bytes: &[u8]);
    fn bi_to_i64(&self, reference: Handle) -> Option<i64>;
    fn add(&self, dest: Handle, x: Handle, y: Handle);
    fn sub(&self, dest: Handle, x: Handle, y: Handle);
    fn mul(&self, dest: Handle, x: Handle, y: Handle);
    fn t_div(&self, dest: Handle, x: Handle, y: Handle);
    fn t_mod(&self, dest: Handle, x: Handle, y: Handle);
    fn pow(&self, dest: Handle, x: Handle, y: Handle);
    fn abs(&self, dest: Handle, x: Handle);
    fn neg(&self, dest: Handle, x: Handle);
    fn sign(&self, x: Handle) -> Sign;
    fn cmp(&self, x: Handle, y: Handle) -> Ordering;

    // fn zero(&self, ) -> Self {
    //     0i64.into(&self, )
    // }

    // fn abs_uint(&self, &self) -> Self::BigUint;

    // fn sign(&self, &self) -> Sign;

    // fn to_signed_bytes_be(&self, &self) -> Vec<u8>;

    // fn from_signed_bytes_be(&self, bytes: &[u8]) -> Self;

    // fn pow(&self, &self, exp: u32) -> Self;

    // /// Will return `None` if the number is too big or too small to be converted.
    // fn to_i64(&self, &self) -> Option<i64>;
}
