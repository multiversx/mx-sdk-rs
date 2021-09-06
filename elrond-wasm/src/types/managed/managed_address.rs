use crate::{api::ManagedTypeApi, types::Address};

use super::{ManagedByteArray, ManagedFrom};

pub type ManagedAddress<M> = ManagedByteArray<M, 32>;

impl<M> ManagedByteArray<M, 32>
where
    M: ManagedTypeApi,
{
    pub fn from_address(api: M, address: Address) -> Self {
        Self::new_from_bytes(api, address.as_array())
    }

    pub fn zero_address(api: M) -> Self {
        Self::new_from_bytes(api, &[0u8; 32])
    }

    pub fn to_address(&self) -> Address {
        let mut result = Address::zero();
        let _ = self.buffer.load_slice(0, result.as_mut());
        result
    }
}

impl<M> ManagedFrom<M, &Address> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, address: &Address) -> Self {
        Self::new_from_bytes(api, address.as_array())
    }
}

impl<M> ManagedFrom<M, Address> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, address: Address) -> Self {
        Self::new_from_bytes(api, address.as_array())
    }
}
