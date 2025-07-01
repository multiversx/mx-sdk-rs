mod handle_map;
mod tx_big_float;
mod tx_big_int;
mod tx_managed_buffer;
mod tx_managed_map;

pub use handle_map::HandleMap;
use num_bigint::BigInt;
pub use tx_big_int::{
    big_int_signed_bytes, big_int_to_i64, big_uint_to_u64, big_uint_unsigned_bytes,
};
pub use tx_managed_buffer::InvalidSliceError;

use std::collections::HashMap;

pub(crate) type ManagedBufferImpl = Vec<u8>;
pub(crate) type ManagedMapImpl = HashMap<Vec<u8>, Vec<u8>>;

#[derive(Debug)]
pub struct ManagedTypeContainer {
    pub(crate) big_int_map: HandleMap<BigInt>,
    pub(crate) big_float_map: HandleMap<f64>,
    pub(crate) managed_buffer_map: HandleMap<ManagedBufferImpl>,
    pub(crate) managed_map_map: HandleMap<ManagedMapImpl>,
}

impl ManagedTypeContainer {
    pub fn new() -> Self {
        ManagedTypeContainer {
            big_int_map: HandleMap::new(),
            big_float_map: HandleMap::new(),
            managed_buffer_map: HandleMap::new(),
            managed_map_map: HandleMap::new(),
        }
    }
}

impl Default for ManagedTypeContainer {
    fn default() -> Self {
        ManagedTypeContainer::new()
    }
}
