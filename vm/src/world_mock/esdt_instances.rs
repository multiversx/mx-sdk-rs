use super::{EsdtInstance, EsdtInstanceMetadata};
use crate::display_util::verbose_hex_list;
use num_bigint::BigUint;
use num_traits::Zero;
use std::{
    collections::BTreeMap,
    fmt::{self, Write},
};

#[derive(Clone, Debug, Default)]
pub struct EsdtInstances(BTreeMap<u64, EsdtInstance>);

impl EsdtInstances {
    pub fn new() -> Self {
        EsdtInstances(BTreeMap::new())
    }

    pub fn new_from_hash(hash: BTreeMap<u64, EsdtInstance>) -> Self {
        EsdtInstances(hash)
    }

    pub fn add(&mut self, nonce: u64, value: BigUint) {
        if self.0.contains_key(&nonce) {
            let esdt_balance = self.0.get_mut(&nonce).unwrap();
            esdt_balance.balance += value;
        } else {
            let mut instance = EsdtInstance::default(nonce);
            instance.balance = value;
            self.push_instance(instance)
        }
    }

    pub fn push_instance(&mut self, instance: EsdtInstance) {
        self.0.insert(instance.nonce, instance);
    }

    pub fn increase_balance(
        &mut self,
        nonce: u64,
        value: &BigUint,
        metadata: EsdtInstanceMetadata,
    ) {
        let instance = self.0.entry(nonce).or_insert_with(|| EsdtInstance {
            nonce,
            balance: BigUint::zero(),
            metadata: metadata.clone(),
        });
        if instance.balance.is_zero() {
            instance.metadata = metadata;
        }

        instance.balance += value;
    }

    pub fn set_balance(&mut self, nonce: u64, value: &BigUint, metadata: EsdtInstanceMetadata) {
        let _ = self
            .0
            .entry(nonce)
            .and_modify(|instance| {
                instance.balance = value.clone();
                instance.nonce = nonce;
                instance.metadata = metadata.clone();
            })
            .or_insert_with(|| EsdtInstance {
                nonce,
                balance: value.clone(),
                metadata,
            });
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

    pub fn get_instances(&self) -> &BTreeMap<u64, EsdtInstance> {
        &self.0
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
            let creator_encoded = if let Some(creator) = &value.metadata.creator {
                hex::encode(creator)
            } else {
                "".to_string()
            };
            write!(
                instance_buf,
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
                creator_encoded,
                value.metadata.royalties,
                hex::encode(
                    value
                        .metadata
                        .hash
                        .as_ref()
                        .unwrap_or(&Vec::new())
                        .as_slice()
                ),
                verbose_hex_list(value.metadata.uri.as_slice()),
                hex::encode(value.metadata.attributes.as_slice())
            )?;
        }
        Ok(())
    }
}
