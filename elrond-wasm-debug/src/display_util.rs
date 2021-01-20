use alloc::string::String;
use elrond_wasm::types::Address;

pub fn address_hex(address: &Address) -> String {
	alloc::format!("0x{}", hex::encode(address.as_bytes()))
}

pub fn key_hex(key: &[u8]) -> String {
	alloc::format!("0x{}", hex::encode(key))
}

pub fn verbose_hex(value: &[u8]) -> String {
	alloc::format!("0x{}", hex::encode(value))
}
