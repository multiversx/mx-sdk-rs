use num_bigint::BigUint;
use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::Deref,
};

// EsdtInstance holds the data for a Elrond standard digital token transaction

#[derive(Clone, Default, Debug)]
pub struct EsdtInstance {
    pub value: BigUint,
    pub esdt_type: u64,
    pub name: Option<Vec<u8>>,
    pub creator: Option<Vec<u8>>,
    pub reserved: Option<Vec<u8>>,
    pub royalties: Option<u64>,
    pub hash: Option<Vec<u8>>,
    pub uri: Option<Vec<u8>>,
    pub properties: Option<Vec<u8>>,
    pub attributes: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct EsdtInstances(HashMap<u64, EsdtInstance>);

impl Deref for EsdtInstances {
    type Target = HashMap<u64, EsdtInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for EsdtInstances {
    fn default() -> Self {
        EsdtInstances(HashMap::new())
    }
}

impl EsdtInstances {
    pub fn new(nonce: u64, value: BigUint) -> Self {
        let instances = EsdtInstances(HashMap::new());
        instances.add(nonce, value);
        instances
    }
    pub fn add(&self, nonce: u64, value: BigUint) {
        if self.contains_key(&nonce) {
            let esdt_balance = &self.get_mut(&nonce).unwrap();
            esdt_balance.value += value;
        } else {
            self.add_new(nonce, value)
        }
    }

    pub fn add_new(&self, nonce: u64, value: BigUint) {
        self.insert(
            nonce,
            EsdtInstance {
                value: value.clone(),
                esdt_type: 0u64,
                name: None,
                creator: None,
                reserved: None,
                royalties: None,
                hash: None,
                uri: None,
                properties: None,
                attributes: None,
            },
        );
    }
    pub fn get_by_nonce(&self, nonce: u64) -> Option<EsdtInstance> {
        self.iter()
            .find_map(|(key, &val)| if key == &nonce { Some(val) } else { None })
    }
}

impl fmt::Display for EsdtInstances {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut instance_buf = String::new();
        let mut instance_keys: Vec<u64> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &instance_keys {
            let value = self.0.get(key).unwrap();
            write!(
                &mut instance_buf,
                "\n\t\t\t\t{} -> {{
                    value: {},
                    esdt_type: {},
                    name: {},
                    creator: {},
                    reserved: {},
                    royalties: {},
                    hash: {},
                    uri: [{} ],
                    properties: {},
                    attributes: {}
                }}",
                key.to_string(),
                value.value,
                value.esdt_type,
                hex::encode(value.name.unwrap().as_slice()),
                hex::encode(value.creator.unwrap().as_slice()),
                hex::encode(value.reserved.unwrap().as_slice()),
                value.royalties.unwrap(),
                hex::encode(value.hash.unwrap().as_slice()),
                hex::encode(value.uri.unwrap().as_slice()),
                hex::encode(value.properties.unwrap().as_slice()),
                hex::encode(value.attributes.unwrap().as_slice())
            )?;
        }
        Ok(())
    }
}
