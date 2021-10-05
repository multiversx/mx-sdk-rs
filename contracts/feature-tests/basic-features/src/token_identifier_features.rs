elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_egld(&self) -> TokenIdentifier {
        TokenIdentifier::egld()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, bytes: &[u8]) -> bool {
        TokenIdentifier::from(bytes).is_valid_esdt_identifier()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_esdt_identifier()
    }
}
