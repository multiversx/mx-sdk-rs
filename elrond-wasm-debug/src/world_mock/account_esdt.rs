use num_bigint::BigUint;

use crate::{esdt_instance::EsdtInstances, key_hex};
use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::Deref,
};

#[derive(Clone, Default, Debug)]
pub struct EsdtRoles(Vec<Vec<u8>>);

#[derive(Clone, Default, Debug)]
pub struct EsdtData {
    pub token_identifier: Vec<u8>,
    pub instances: EsdtInstances,
    pub last_nonce: u64,
    pub roles: EsdtRoles,
    pub frozen: u64,
}

#[derive(Clone, Default, Debug)]
pub struct AccountEsdt(HashMap<Vec<u8>, EsdtData>);

impl AccountEsdt {
    pub fn get_by_identifier(&self, identifier: Vec<u8>) -> Option<EsdtData> {
        self.iter().find_map(|(_, x)| {
            if x.token_identifier == identifier {
                Some(x.clone())
            } else {
                None
            }
        })
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
                frozen: 0u64,
            },
        );
    }
}

impl Deref for AccountEsdt {
    type Target = HashMap<Vec<u8>, EsdtData>;

    fn deref(&self) -> &Self::Target {
        &self.0
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

impl EsdtRoles {
    pub fn new(roles: Vec<Vec<u8>>) -> Self {
        EsdtRoles(roles)
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

impl fmt::Display for EsdtRoles {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let esdt_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|k| k.clone()).collect();

        for value in &esdt_keys {
            write!(&mut esdt_buf, "{}", hex::encode(value.as_slice()))?;
        }
        Ok(())
    }
}
