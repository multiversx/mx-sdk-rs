use crate::multiversx_sc::types::heap::Address;
use sha2::{Digest, Sha256};

const ADDRESS_LEN: usize = 32;
const SC_ADDR_LEADING_ZEROES: usize = 8;

pub(crate) struct AddressFactory {
    last_generated_address: [u8; ADDRESS_LEN],
}

impl Default for AddressFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressFactory {
    pub fn new() -> Self {
        Self {
            last_generated_address: [0u8; ADDRESS_LEN],
        }
    }

    pub fn new_address(&mut self) -> Address {
        Address::from(self.new_address_raw())
    }

    pub fn new_sc_address(&mut self) -> Address {
        let mut addr = self.new_address_raw();
        for byte in addr.iter_mut().take(SC_ADDR_LEADING_ZEROES) {
            *byte = 0;
        }

        Address::from(addr)
    }

    fn new_address_raw(&mut self) -> [u8; ADDRESS_LEN] {
        let mut hasher = Sha256::new();
        hasher.update(self.last_generated_address);
        let result: [u8; ADDRESS_LEN] = hasher.finalize().into();

        self.last_generated_address = result;

        result
    }
}
