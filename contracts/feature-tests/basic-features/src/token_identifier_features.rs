multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_egld(&self) -> EgldOrEsdtTokenIdentifier {
        EgldOrEsdtTokenIdentifier::egld()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, token_id: EgldOrEsdtTokenIdentifier) -> bool {
        token_id.is_valid()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_esdt_identifier()
    }
}
