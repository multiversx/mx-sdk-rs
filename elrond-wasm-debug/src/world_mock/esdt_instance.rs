use crate::key_hex;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::fmt::{self, Write};

#[derive(Clone)]
pub struct EsdtUri(HashMap<Vec<u8>, Vec<u8>>);

// EsdtInstance holds the data for a Elrond standard digital token transaction

#[derive(Clone)]
pub struct EsdtInstance {
    pub nonce: u64,
    pub value: BigInt,
    pub esdt_type: u32,
    pub name: Option<Vec<u8>>,
    pub creator: Option<Vec<u8>>,
    pub reserved: Option<Vec<u8>>,
    pub royalties: Option<u32>,
    pub hash: Option<Vec<u8>>,
    pub uri: Option<EsdtUri>,
    pub properties: Option<Vec<u8>>,
    pub attributes: Option<Vec<u8>>,
}

#[derive(Clone)]
pub struct EsdtInstances(HashMap<Vec<u8>, EsdtInstance>);

impl fmt::Display for EsdtInstances {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut instance_buf = String::new();
        let mut instance_keys: Vec<Vec<u8>> =
            self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &instance_keys {
            let value = self.0.get(key).unwrap();
            write!(
                &mut instance_buf,
                "\n\t\t\t\t{} -> {{
                    nonce: {},
                    value: {},
                    esdt_type: {},
                    name: {},
                    creator: {},
                    reserved: {},
                    royalties: {},
                    hash: {},
                    uri: {},
                    properties: {},
                    attributes: {}
                }}",
                key_hex(key.as_slice()),
                value.nonce,
                value.value,
                value.esdt_type,
                hex::encode(value.name.unwrap().as_slice()),
                hex::encode(value.creator.unwrap().as_slice()),
                hex::encode(value.reserved.unwrap().as_slice()),
                value.royalties.unwrap(),
                hex::encode(value.hash.unwrap().as_slice()),
                value.uri.unwrap(),
                hex::encode(value.properties.unwrap().as_slice()),
                hex::encode(value.attributes.unwrap().as_slice())
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for EsdtUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let mut esdt_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &esdt_keys {
            let value = self.0.get(key).unwrap();
            write!(
                &mut esdt_buf,
                "\n\t\t\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.as_slice())
            )?;
        }
        Ok(())
    }
}
