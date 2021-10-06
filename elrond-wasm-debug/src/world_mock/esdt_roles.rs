use std::fmt::{self, Write};

#[derive(Clone, Default, Debug)]
pub struct EsdtRoles(Vec<Vec<u8>>);

impl EsdtRoles {
    pub fn new(roles: Vec<Vec<u8>>) -> Self {
        EsdtRoles(roles)
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
