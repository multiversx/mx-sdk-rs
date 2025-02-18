multiversx_sc::imports!();

/// Exposes ways to retrieve special roles of a specific token on-chain
#[multiversx_sc::module]
pub trait RetrieveSpecialRoles {
    #[endpoint]
    fn token_has_transfer_role(&self, token_identifier: TokenIdentifier) -> bool {
        self.blockchain().token_has_transfer_role(token_identifier)
    }
}
