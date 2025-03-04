multiversx_sc::imports!();

/// Used for testing the egld_decimal function return type
#[multiversx_sc::module]
pub trait EgldDecimal {
    #[payable("EGLD")]
    #[endpoint]
    fn returns_egld_decimal(&self) -> ManagedDecimal<Self::Api, EgldDecimals> {
        self.call_value().egld_decimal()
    }
}
