extern crate rand;

use core::str;

use anyhow::Result;
use multiversx_chain_core::types::BLSKey;

#[derive(Clone, Debug)]
pub struct Validator {
    pub private_key: Vec<u8>,
    pub public_key: BLSKey,
}

impl Validator {
    pub fn from_pem_file(file_path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(file_path)?;
        Self::from_pem_file_contents(contents)
    }

    pub fn from_pem_file_contents(contents: String) -> Result<Self> {
        let pem = pem::parse(contents).expect("Failed to parse PEM file");
        let public_key_str = pem
            .tag()
            .rsplit(' ')
            .next()
            .expect("Failed to extract public key from PEM file");

        let public_key = hex::decode(public_key_str)?;
        let private_key = pem.contents();

        Ok(Validator {
            private_key: private_key.to_vec(),
            public_key: BLSKey::from_vec(public_key).expect("bad public key length"),
        })
    }
}
