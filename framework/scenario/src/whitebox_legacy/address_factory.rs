use crate::multiversx_sc::types::heap::Address;
use multiversx_chain_vm::chain_core::std::crypto;

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
        let result = crypto::sha256(&self.last_generated_address);
        self.last_generated_address = result;
        result
    }
}
