use std::{error::Error, fmt::Display};

use bech32::{Bech32, Hrp};
use multiversx_sc::types::heap::Address;

#[derive(Debug)]
pub struct InvalidAddressLengthError;

impl Display for InvalidAddressLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Invalid address length after decoding".fmt(f)
    }
}

impl Error for InvalidAddressLengthError {}

pub fn try_decode(bech32_address: &str) -> Result<Address, Box<dyn Error>> {
    let (_hrp, dest_address_bytes) = bech32::decode(bech32_address)?;
    if dest_address_bytes.len() != 32 {
        return Err(Box::new(InvalidAddressLengthError));
    }

    Ok(Address::from_slice(&dest_address_bytes))
}

pub fn decode(bech32_address: &str) -> Address {
    try_decode(bech32_address).expect("bech32 Address decode failed")
}

pub fn encode(address: &Address) -> String {
    let hrp = Hrp::parse("erd").expect("invalid hrp");
    bech32::encode::<Bech32>(hrp, address.as_bytes()).expect("bech32 encode error")
}
