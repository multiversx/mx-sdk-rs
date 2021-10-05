use crate::{esdt_instance::EsdtInstances, key_hex};
use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::Deref,
};

#[derive(Clone)]
pub struct EsdtRoles(HashMap<Vec<u8>, Vec<u8>>);

#[derive(Clone, Default)]
pub struct EsdtData {
    pub token_identifier: Vec<u8>,
    pub instances: EsdtInstances,
    pub last_nonce: u64,
    pub roles: Option<EsdtRoles>,
    pub frozen: u64,
}

#[derive(Clone, Default)]
pub struct AccountEsdt(HashMap<Vec<u8>, EsdtData>);

impl AccountEsdt {
    pub fn get_by_identifier(&self, identifier: Vec<u8>) -> Option<&EsdtData> {
        self.iter().find_map(|(_, x)| {
            if x.token_identifier == identifier {
                Some(x)
            } else {
                None
            }
        })
    }
}

impl Deref for AccountEsdt {
    type Target = HashMap<Vec<u8>, EsdtData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for EsdtData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            self.roles.unwrap(),
            self.frozen
        )?;
        Ok(())
    }
}

impl fmt::Display for AccountEsdt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let mut esdt_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let mut esdt_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &esdt_keys {
            let value = self.0.get(key).unwrap();
            write!(
                &mut esdt_buf,
                "\n\t\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.as_slice())
            )?;
        }
        Ok(())
    }
}
