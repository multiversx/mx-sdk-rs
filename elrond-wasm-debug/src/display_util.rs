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

/// returns it as hex formatted number if it's not valid utf8
pub fn vec_u8_to_string(vec_u8: &Vec<u8>) -> String {
	String::from_utf8(vec_u8.clone()).unwrap_or_else(|_| verbose_hex(vec_u8))
}
