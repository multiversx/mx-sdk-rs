use crate::num_bigint::BigUint;
use num_traits::Zero;

use crate::key_hex;
use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::{self, Write},
};

use super::{EsdtInstanceMetadata, EsdtInstances, EsdtRoles};

#[derive(Clone, Default, Debug)]
pub struct EsdtData {
    pub instances: EsdtInstances,
    pub last_nonce: u64,
    pub roles: EsdtRoles,
    pub frozen: bool,
}

impl EsdtData {
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty_esdt()
            && self.last_nonce == 0
            && self.roles.is_empty()
            && !self.frozen
    }

    pub fn get_roles(&self) -> Vec<Vec<u8>> {
        self.roles.get()
    }
}

#[derive(Clone, Default, Debug)]
pub struct AccountEsdt(HashMap<Vec<u8>, EsdtData>);

impl AccountEsdt {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_identifier(&self, identifier: &[u8]) -> Option<&EsdtData> {
        self.0.get(identifier)
    }

    /// Will provide a clone.
    pub fn get_roles(&self, identifier: &[u8]) -> Vec<Vec<u8>> {
        self.get_by_identifier_or_default(identifier).get_roles()
    }

    pub fn set_roles(&mut self, token_identifier: Vec<u8>, roles: Vec<Vec<u8>>) {
        let esdt_data = self.0.entry(token_identifier).or_insert_with(|| EsdtData {
            instances: EsdtInstances::new(),
            last_nonce: 0,
            roles: EsdtRoles::default(),
            frozen: false,
        });
        esdt_data.roles = EsdtRoles::new(roles);
    }

    /// Will provide a clone.
    pub fn get_by_identifier_or_default(&self, identifier: &[u8]) -> EsdtData {
        if let Some(value) = self.0.get(identifier) {
            value.clone()
        } else {
            EsdtData::default()
        }
    }

    pub fn get_mut_by_identifier(&mut self, identifier: &[u8]) -> Option<&mut EsdtData> {
        self.0.get_mut(identifier)
    }

    pub fn new_from_raw_map(hash: HashMap<Vec<u8>, EsdtData>) -> Self {
        AccountEsdt(hash)
    }

    pub fn increase_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: EsdtInstanceMetadata,
    ) {
        let esdt_data = self.0.entry(token_identifier).or_insert_with(|| EsdtData {
            instances: EsdtInstances::new(),
            last_nonce: nonce,
            roles: EsdtRoles::default(),
            frozen: false,
        });
        esdt_data.instances.increase_balance(nonce, value, metadata);
    }

    pub fn set_esdt_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: EsdtInstanceMetadata,
    ) {
        let esdt_data = self.0.entry(token_identifier).or_insert_with(|| EsdtData {
            instances: EsdtInstances::new(),
            last_nonce: nonce,
            roles: EsdtRoles::default(),
            frozen: false,
        });
        esdt_data.instances.set_balance(nonce, value, metadata);
    }

    pub fn get_esdt_balance(&self, token_identifier: &[u8], nonce: u64) -> BigUint {
        if let Some(esdt_data) = self.get_by_identifier(token_identifier) {
            if let Some(instance) = esdt_data.instances.get_by_nonce(nonce) {
                instance.balance.clone()
            } else {
                BigUint::zero()
            }
        } else {
            BigUint::zero()
        }
    }

    pub fn add_uris(&mut self, token_identifier: &[u8], nonce: u64, mut new_uris: Vec<Vec<u8>>) {
        self.0
            .get_mut(token_identifier)
            .unwrap_or_else(|| panic!("invalid token"))
            .instances
            .get_mut_by_nonce(nonce)
            .unwrap_or_else(|| panic!("invalid token nonce"))
            .metadata
            .uri
            .append(&mut new_uris);
    }

    pub fn update_attributes(
        &mut self,
        token_identifier: &[u8],
        nonce: u64,
        new_attribute_bytes: Vec<u8>,
    ) {
        self.0
            .get_mut(token_identifier)
            .unwrap_or_else(|| panic!("invalid token"))
            .instances
            .get_mut_by_nonce(nonce)
            .unwrap_or_else(|| panic!("invalid token nonce"))
            .metadata
            .attributes = new_attribute_bytes;
    }

    pub fn iter(&self) -> Iter<Vec<u8>, EsdtData> {
        self.0.iter()
    }
}

impl fmt::Display for EsdtData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        write!(
            esdt_buf,
            "{{
                instances: [{}],
                last_nonce: {},
                roles: [{}],
                frozen: {},
            }}",
            self.instances, self.last_nonce, self.roles, self.frozen
        )?;
        Ok(())
    }
}

impl fmt::Display for AccountEsdt {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let esdt_keys: Vec<Vec<u8>> = self.0.keys().cloned().collect();

        for key in &esdt_keys {
            let value = self.0.get(key).unwrap();
            write!(esdt_buf, "\n\t\t\t{} -> {value}", key_hex(key.as_slice()))?;
        }
        Ok(())
    }
}
