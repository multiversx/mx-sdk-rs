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
pub fn bytes_to_string(bytes: &[u8]) -> String {
	String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| verbose_hex(bytes))
}
