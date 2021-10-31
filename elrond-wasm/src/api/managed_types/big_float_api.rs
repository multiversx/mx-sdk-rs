use core::cmp::Ordering;

use crate::types::BoxedBytes;

use super::Handle;

pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

/// Definition of the BigFloat type required by the API.
pub trait BigFloatApi {
    fn bf_from_parts(&self, integral_part: i32, fractional_part: i32, exponent: i32) -> Handle;
    fn bf_from_frac(&self, numerator: i64, denominator: i64) -> Handle;    
    fn bf_from_sci(&self, significand: i64, exponent: i64) -> Handle;
    
    fn bf_add(&self, dest: Handle, x: Handle, y: Handle);
    fn bf_sub(&self, dest: Handle, x: Handle, y: Handle);
    fn bf_mul(&self, dest: Handle, x: Handle, y: Handle);
    fn bf_div(&self, dest: Handle, x: Handle, y: Handle);
    
    fn bf_abs(&self, dest: Handle, x: Handle);
    fn bf_neg(&self, dest: Handle, x: Handle);
    fn bf_cmp(&self, x: Handle, y: Handle) -> Ordering;
    fn bf_sign(&self, x: Handle) -> Sign;
    fn bf_clone(&self, dest: Handle, x: Handle);
    fn bf_sqrt(&self, dest: Handle,x: Handle);
    fn bf_pow(&self, dest: Handle,x: Handle,y: Handle);
    
    fn bf_floor(&self, dest: Handle, x: Handle);
    fn bf_ceil(&self, dest: Handle,x: Handle);
    fn bf_trunc(&self, dest: Handle,x: Handle);
    
    fn bf_is_bi(&self, x: Handle) -> bool;
    fn bf_set_i64(&self, dest: Handle, value: i64);
    fn bf_set_bi(&self, dest: Handle, bi: Handle);
    
    fn bf_get_const_pi(&self, dest: Handle);
    fn bf__get_const_e(&self, dest: Handle);
}
