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

    fn handle_back_transfers_if_any(
        &self,
        opt_back_transfers: Option<BackTransfers<Self::Api>>,
    ) -> Option<BackTransfers<Self::Api>> {
        opt_back_transfers.map(|back_transfers| back_transfers.clone())
    }
}
