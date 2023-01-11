use crate::{num_bigint, num_bigint::BigInt};
use alloc::string::String;
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        heap::{Address, BoxedBytes},
        BigUint, ManagedType,
    },
};
use std::fmt;

pub struct BigUintPrinter<M: ManagedTypeApi> {
    pub value: BigUint<M>,
}

pub fn address_hex(address: &Address) -> String {
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

impl<M: ManagedTypeApi> fmt::Debug for BigUintPrinter<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let handle = self.value.get_handle();
        let mut bytes = self.value.to_bytes_be();
        if bytes.is_empty() {
            bytes = BoxedBytes::from(vec![0u8]);
        }

        let hex = hex::encode(bytes.as_slice());
        let dec = BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes.as_slice());

        f.debug_struct("BigUint")
            .field("handle", &handle)
            .field("hex", &hex)
            .field("dec", &dec.to_string())
            .finish()
    }
}
