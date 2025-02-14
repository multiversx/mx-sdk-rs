use multiversx_sc::storage::StorageKey;

multiversx_sc::imports!();

/// Exposes ways to retrieve special roles of a specific token on-chain
#[multiversx_sc::module]
pub trait RetrieveSpecialRoles {
    #[endpoint]
    fn token_has_transfer_role(&self, token_id: TokenIdentifier) -> bool {
        let key = ManagedBuffer::new_from_bytes(b"ELRONDtransferesdt");
        let base_key = key.concat(token_id.into_managed_buffer());

        // Decoding the response needs more research
        // Empty response means no address has transferRole for the token
        let remote = SingleValueMapper::<Self::Api, ManagedBuffer, _>::new_from_address(
            SystemSCAddress.to_managed_address(),
            StorageKey::from(base_key),
        );

        !remote.is_empty()
    }
}
