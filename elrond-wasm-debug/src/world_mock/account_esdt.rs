use num_bigint::BigUint;
use num_traits::Zero;

use crate::key_hex;
use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::{self, Write},
};

use super::{EsdtInstances, EsdtRoles};

#[derive(Clone, Debug)]
pub enum EsdtData {
    Short(Vec<u8>),
    Full(EsdtFullData),
}

#[derive(Clone, Default, Debug)]
pub struct EsdtFullData {
    pub token_identifier: Vec<u8>,
    pub instances: EsdtInstances,
    pub last_nonce: u64,
    pub roles: EsdtRoles,
    pub frozen: u64,
}

impl Default for EsdtData {
    fn default() -> Self {
        EsdtData::Short("".as_bytes().to_vec())
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
            EsdtData::Full(EsdtFullData {
                token_identifier,
                instances: EsdtInstances::new(nonce, value),
                last_nonce: nonce,
                roles: EsdtRoles::default(),
                frozen: 0u64,
            }),
        );
    }

    pub fn get_esdt_balance(&self, token_identifier: &[u8], nonce: u64) -> BigUint {
        if let Some(esdt_data) = self.get_by_identifier(token_identifier) {
            match esdt_data {
                EsdtData::Short(short_esdt) => todo!(),
                EsdtData::Full(full_esdt) => {
                    if let Some(instance) = full_esdt.instances.get_by_nonce(nonce) {
                        instance.balance.clone()
                    } else {
                        BigUint::zero()
                    }
                },
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
        match self {
            EsdtData::Short(short_esdt) => write!(&mut esdt_buf, "{}", key_hex(short_esdt),)?,
            EsdtData::Full(full_esdt) => write!(
                &mut esdt_buf,
                "{{
                    instances: [{}],
                    last_nonce: {},
                    roles: [{}],
                    frozen: {},
                }}",
                full_esdt.instances, full_esdt.last_nonce, full_esdt.roles, full_esdt.frozen
            )?,
        };
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
