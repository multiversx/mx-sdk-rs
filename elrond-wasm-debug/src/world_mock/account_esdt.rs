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
    pub instances: EsdtInstances,
    pub last_nonce: Option<u64>,
    pub roles: Option<EsdtRoles>,
}

pub struct AccountEsdt(HashMap<Vec<u8>, EsdtData>);

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
                instances: [{}],
                last_nonce: {},
                roles: [{}],
            }}",
            self.instances,
            self.last_nonce.unwrap(),
            self.roles.unwrap()
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
