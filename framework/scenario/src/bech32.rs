use bech32::{Bech32, Hrp};
use multiversx_sc::types::heap::Address;

pub fn decode(bech32_address: &str) -> Address {
    let (_hrp, dest_address_bytes) = bech32::decode(bech32_address)
        .unwrap_or_else(|err| panic!("bech32 decode error for {bech32_address}: {err}"));
    if dest_address_bytes.len() != 32 {
        panic!("Invalid address length after decoding")
    }

    Address::from_slice(&dest_address_bytes)
}

pub fn encode(address: &Address) -> String {
    let hrp = Hrp::parse("erd").expect("invalid hrp");
    bech32::encode::<Bech32>(hrp, address.as_bytes()).expect("bech32 encode error")
}
