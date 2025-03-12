use crate::types::RawHandle;

use super::TxManagedTypes;

impl TxManagedTypes {
    pub fn bf_get_f64(&self, handle: RawHandle) -> f64 {
        *self.big_float_map.get(handle)
    }

    pub fn bf_overwrite(&mut self, handle: RawHandle, value: f64) {
        self.big_float_map.insert(handle, value);
    }
}
