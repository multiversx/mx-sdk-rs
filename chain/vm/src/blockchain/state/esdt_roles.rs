use std::fmt::{self, Write};

#[derive(Clone, Default, Debug)]
pub struct EsdtRoles(Vec<Vec<u8>>);

impl EsdtRoles {
    pub fn new(roles: Vec<Vec<u8>>) -> Self {
        EsdtRoles(roles)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self) -> Vec<Vec<u8>> {
        self.0.clone()
    }
}

impl fmt::Display for EsdtRoles {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let esdt_keys: Vec<Vec<u8>> = self.clone().0.to_vec();

        for value in &esdt_keys {
            write!(esdt_buf, "{}", hex::encode(value.as_slice()))?;
        }
        Ok(())
    }
}
