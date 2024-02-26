mod vh_big_float;
mod vh_big_int;
mod vh_managed_buffer;
mod vh_managed_map;

pub use vh_big_float::VMHooksBigFloat;
pub use vh_big_int::VMHooksBigInt;
pub use vh_managed_buffer::VMHooksManagedBuffer;
pub use vh_managed_map::VMHooksManagedMap;

use std::fmt::Debug;

use crate::types::RawHandle;

use super::VMHooksError;

const FLOAT_GOB_VERSION: u8 = 1;
const _W: usize = 64;
// Number of bits in a mantissa word
// 64-bit precision for IEEE-754

/// Provides VM hook implementations for methods that deal with more than one type of managed type.
///
/// It is also the trait that unifies all managed type functionality.
pub trait VMHooksManagedTypes:
    VMHooksBigInt + VMHooksManagedBuffer + VMHooksManagedMap + VMHooksBigFloat + VMHooksError + Debug
{
    fn mb_to_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_unsigned_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_to_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_signed_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_from_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_lock().bi_get_unsigned_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);
    }

    fn mb_from_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_lock().bi_get_signed_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);
    }

    fn bi_to_string(&self, bi_handle: RawHandle, str_handle: RawHandle) {
        let bi = self.m_types_lock().bi_get(bi_handle);
        let s = bi.to_string();
        self.m_types_lock().mb_set(str_handle, s.into_bytes());
    }

    fn mb_set_random(&self, dest_handle: RawHandle, length: usize) {
        let bytes = self.random_next_bytes(length);
        self.mb_set(dest_handle, bytes.as_slice());
    }

    fn mb_from_big_float(&self, m_buffer_handle: RawHandle, big_float_handle: RawHandle) {
        // transform big float into bytes with respect to IEEE-754
        let big_float = self.m_types_lock().bf_get_f64(big_float_handle);
        let bytes = gob_encode(big_float);
        // println!("managed buffer {:#?}", )
        self.m_types_lock().mb_set(m_buffer_handle, bytes); // encoded bytes
    }
}

pub fn gob_encode(floating_number: f64) -> Vec<u8> {
    let mut buf = Vec::new();

    // Encode the version
    buf.push(FLOAT_GOB_VERSION);

    // Calculate the mode byte
    let mut mode_byte = 0u8;
    // Extract rounding mode bits (lowest 3 bits)
    let rounding_mode_bits = ((floating_number.to_bits() >> 61) & 0b111) as u8; // Extract 3 bits
    mode_byte |= rounding_mode_bits << 5;

    // Extract accuracy bits (next 2 bits)
    let accuracy_bits = ((floating_number.to_bits() >> 59) & 0b11) as u8; // Extract 2 bits
    mode_byte |= accuracy_bits << 3;

    // Extract form bits (next 2 bits)
    let form_bits = ((floating_number.to_bits() >> 57) & 0b11) as u8; // Extract 2 bits
    mode_byte |= form_bits << 1;

    // Extract negation flag (highest bit)
    let negation_flag_bit = ((floating_number.to_bits() >> 63) & 0b1) as u8; // Extract 1 bit
    mode_byte |= negation_flag_bit;

    // Push the mode byte to the buffer
    buf.push(mode_byte);

    // Encode precision
    let precision_bytes = floating_number.to_bits().to_be_bytes();
    buf.extend_from_slice(&precision_bytes);

    // Encode exponent
    let exponent_bytes = floating_number.to_bits().to_be_bytes();
    buf.extend_from_slice(&exponent_bytes[7..]);

    // Calculate the number of mantissa words
    let n = (((floating_number.to_bits() >> 52) & 0x7FF) as usize) + (_W - 1) / _W;

    // Encode mantissa
    if form_bits == 0b11 {
        let mantissa_bytes = floating_number.to_bits().to_be_bytes();
        buf.extend_from_slice(&mantissa_bytes[8..8 + n]);
    }

    buf
}
