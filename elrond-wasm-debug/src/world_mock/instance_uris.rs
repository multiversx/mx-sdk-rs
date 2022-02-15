use std::fmt::{self, Write};

#[derive(Clone, Default, Debug)]
pub struct InstanceUris(Vec<Vec<u8>>);

impl InstanceUris {
    pub fn new(roles: Vec<Vec<u8>>) -> Self {
        InstanceUris(roles)
    }

    pub fn new_from_slice(roles: &[u8]) -> Self {
        let uris: Vec<Vec<u8>> = roles
            .split(|ch| *ch == b'\n')
            .map(|slice| slice.to_vec())
            .collect();
        InstanceUris(uris)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self) -> Vec<Vec<u8>> {
        self.0.clone()
    }

    pub fn get_as_string_vec(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|elem| String::from_utf8(elem.clone()).unwrap())
            .collect()
    }

    pub fn get_as_slice_vec(&self) -> Vec<&[u8]> {
        self.0.iter().map(|elem| elem.as_slice()).collect()
    }
}

impl fmt::Display for InstanceUris {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let esdt_keys: Vec<Vec<u8>> = self.clone().0.to_vec();

        for value in &esdt_keys {
            write!(&mut esdt_buf, "{}", hex::encode(value.as_slice()))?;
        }
        Ok(())
    }
}
