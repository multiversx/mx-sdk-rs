use num_bigint::BigUint;
use std::{
    collections::BTreeMap,
    fmt::{self, Write},
};

use super::EsdtInstance;

#[derive(Clone, Debug)]
pub struct EsdtInstances(BTreeMap<u64, EsdtInstance>);

impl Default for EsdtInstances {
    fn default() -> Self {
        EsdtInstances(BTreeMap::new())
    }
}

impl EsdtInstances {
    pub fn new(nonce: u64, value: BigUint) -> Self {
        let mut instances = EsdtInstances(BTreeMap::new());
        instances.add(nonce, value);
        instances
    }

    pub fn new_from_hash(hash: BTreeMap<u64, EsdtInstance>) -> Self {
        EsdtInstances(hash)
    }

    pub fn add(&mut self, nonce: u64, value: BigUint) {
        if self.0.contains_key(&nonce) {
            let esdt_balance = self.0.get_mut(&nonce).unwrap();
            esdt_balance.balance += value;
        } else {
            self.add_new(nonce, value)
        }
    }

    pub fn add_new(&mut self, nonce: u64, value: BigUint) {
        let mut instance = EsdtInstance::default(nonce);
        instance.balance = value;
        self.0.insert(nonce, instance);
    }

    pub fn get_by_nonce(&self, nonce: u64) -> Option<&EsdtInstance> {
        self.0.get(&nonce)
    }

    pub fn get_by_nonce_or_default(&self, nonce: u64) -> EsdtInstance {
        if let Some(instance) = self.0.get(&nonce) {
            instance.clone()
        } else {
            EsdtInstance::default(nonce)
        }
    }

    pub fn get_mut_by_nonce(&mut self, nonce: u64) -> Option<&mut EsdtInstance> {
        self.0.get_mut(&nonce)
    }

    pub fn is_empty_esdt(&self) -> bool {
        self.0.values().all(EsdtInstance::is_empty_esdt)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for EsdtInstances {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut instance_buf = String::new();
        for (_, value) in self.0.iter() {
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
                value.royalties,
                hex::encode(value.hash.as_ref().unwrap().as_slice()),
                hex::encode(value.uri.as_ref().unwrap().as_slice()),
                hex::encode(value.attributes.as_slice())
            )?;
        }
        Ok(())
    }
}
