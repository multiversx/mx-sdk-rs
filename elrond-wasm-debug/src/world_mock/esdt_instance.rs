use num_bigint::BigUint;
use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::Deref,
};

// EsdtInstance holds the data for a Elrond standard digital token transaction

#[derive(Clone, Default, Debug)]
pub struct EsdtInstance {
    pub nonce: u64,
    pub balance: BigUint,
    pub creator: Option<Vec<u8>>,
    pub royalties: Option<u64>,
    pub hash: Option<Vec<u8>>,
    pub uri: Option<Vec<u8>>,
    pub attributes: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct EsdtInstances(HashMap<Vec<u8>, EsdtInstance>);

impl Deref for EsdtInstances {
    type Target = HashMap<Vec<u8>, EsdtInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Default for EsdtInstances {
    fn default() -> Self {
        EsdtInstances(HashMap::new())
    }
}

impl EsdtInstances {
    pub fn new(nonce: u64, value: BigUint) -> Self {
        let mut instances = EsdtInstances(HashMap::new());
        instances.add(nonce, value);
        instances
    }
    pub fn new_from_hash(hash: HashMap<Vec<u8>, EsdtInstance>) -> Self {
        EsdtInstances(hash)
    }
    pub fn add(&mut self, nonce: u64, value: BigUint) {
        if self.contains_key(&nonce.to_be_bytes().to_vec()) {
            let esdt_balance = self.0.get_mut(&nonce.to_be_bytes().to_vec()).unwrap();
            esdt_balance.balance += value;
        } else {
            self.add_new(nonce, value)
        }
    }

    pub fn add_new(&mut self, nonce: u64, value: BigUint) {
        self.0.insert(
            nonce.to_be_bytes().to_vec(),
            EsdtInstance {
                nonce: nonce.clone(),
                balance: value,
                creator: None,
                royalties: None,
                hash: None,
                uri: None,
                attributes: None,
            },
        );
    }
    pub fn get_by_nonce(&self, nonce: u64) -> Option<EsdtInstance> {
        self.iter().find_map(|(key, val)| {
            if key == &nonce.to_be_bytes().to_vec() {
                Some(val.clone())
            } else {
                None
            }
        })
    }
}

impl fmt::Display for EsdtInstances {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut instance_buf = String::new();
        let instance_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &instance_keys {
            let value = self.0.get(key).unwrap();
            write!(
                &mut instance_buf,
                "{{
                    nonce: {},
                    balance: {},
                    creator: {},
                    royalties: {},
                    hash: {},
                    uri: [{} ],
                    attributes: {}
                }}",
                value.nonce,
                value.balance,
                hex::encode(value.creator.as_ref().unwrap().as_slice()),
                value.royalties.unwrap(),
                hex::encode(value.hash.as_ref().unwrap().as_slice()),
                hex::encode(value.uri.as_ref().unwrap().as_slice()),
                hex::encode(value.attributes.as_ref().unwrap().as_slice())
            )?;
        }
        Ok(())
    }
}
