use core::convert::TryFrom;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ManagedAddressFeatures {
    #[endpoint]
    fn maddress_from_array(&self, array: &[u8; 32]) -> ManagedAddress {
        array.into()
    }

    #[endpoint]
    fn maddress_from_managed_buffer(&self, managed_buffer: ManagedBuffer) -> ManagedAddress {
        ManagedAddress::try_from(managed_buffer).unwrap_or_else(|err| sc_panic!(err))
    }
}
