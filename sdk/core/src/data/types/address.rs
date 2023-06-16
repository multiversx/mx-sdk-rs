use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::types::{ManagedAddress};
use crate::data::address::Address;
use crate::data::types::native::NativeConvertible;

impl<M: ManagedTypeApi> NativeConvertible for ManagedAddress<M> {
    type Native = Address;

    fn to_native(&self) -> Self::Native {
        Address::from_bytes(self.to_byte_array())
    }
}