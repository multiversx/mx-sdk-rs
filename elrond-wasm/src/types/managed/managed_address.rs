use crate::{api::ManagedTypeApi, types::Address};

use super::ManagedByteArray;

pub type ManagedAddress<M> = ManagedByteArray<M, 32>;

impl<M> ManagedByteArray<M, 32>
where
    M: ManagedTypeApi,
{
    pub fn from_address(api: M, address: Address) -> Self {
        Self::new_from_bytes(api, address.as_array())
    }

    pub fn to_address(&self) -> Address {
        let mut result = Address::zero();
        let _ = self.buffer.load_slice(0, result.as_mut());
        result
    }
}
