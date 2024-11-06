use crate::types::VMAddress;

pub fn address_hex(address: &VMAddress) -> String {
    alloc::format!("0x{}", hex::encode(address.as_bytes()))
}

pub fn key_hex(key: &[u8]) -> String {
    alloc::format!("0x{}", hex::encode(key))
}

pub fn verbose_hex(value: &[u8]) -> String {
    alloc::format!("0x{}", hex::encode(value))
}

pub fn verbose_hex_list(values: &[Vec<u8>]) -> String {
    let mut s = String::new();
    s.push('[');
    for (i, topic) in values.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(verbose_hex(topic).as_str());
    }
    s.push(']');
    s
}

/// returns it as hex formatted number if it's not valid utf8
pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| verbose_hex(bytes))
}
