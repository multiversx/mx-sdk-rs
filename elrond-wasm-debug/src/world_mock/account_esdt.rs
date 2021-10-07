use num_bigint::BigUint;
use num_traits::Zero;

use crate::key_hex;
use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::{self, Write},
};

use super::{EsdtInstances, EsdtRoles};

#[derive(Clone, Default, Debug)]
pub struct EsdtData {
    pub token_identifier: Vec<u8>,
    pub instances: EsdtInstances,
    pub last_nonce: u64,
    pub roles: EsdtRoles,
    pub frozen: bool,
}

impl EsdtData {
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty_esdt() && self.last_nonce == 0 && self.roles.is_empty() && !self.frozen
    }
}

#[derive(Clone, Default, Debug)]
pub struct AccountEsdt(HashMap<Vec<u8>, EsdtData>);

impl AccountEsdt {
    pub fn get_by_identifier(&self, identifier: &[u8]) -> Option<&EsdtData> {
        self.0.get(identifier)
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

    pub fn new(token_identifier: Vec<u8>, nonce: u64, value: BigUint) -> Self {
        let mut esdt = AccountEsdt::default();

        esdt.push_esdt(token_identifier, nonce, value);
        esdt
    }

    pub fn new_from_hash(hash: HashMap<Vec<u8>, EsdtData>) -> Self {
        AccountEsdt(hash)
    }

    pub fn push_esdt(&mut self, token_identifier: Vec<u8>, nonce: u64, value: BigUint) {
        self.0.insert(
            token_identifier.clone(),
            EsdtData {
                token_identifier,
                instances: EsdtInstances::new(nonce, value),
                last_nonce: nonce,
                roles: EsdtRoles::default(),
                frozen: false,
            },
        );
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

    pub fn iter(&self) -> Iter<Vec<u8>, EsdtData> {
        self.0.iter()
    }
}

impl fmt::Display for EsdtData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        write!(
            &mut esdt_buf,
            "{{
                token_identifier: {},
                instances: [{}],
                last_nonce: {},
                roles: [{}],
                frozen: {},
            }}",
            key_hex(self.token_identifier.as_slice()),
            self.instances,
            self.last_nonce,
            self.roles,
            self.frozen
        )?;
        Ok(())
    }
}

impl fmt::Display for AccountEsdt {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let esdt_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &esdt_keys {
            let value = self.0.get(key).unwrap();
            write!(
                &mut esdt_buf,
                "\n\t\t\t{} -> {}",
                key_hex(key.as_slice()),
                value
            )?;
        }
        Ok(())
    }
}
