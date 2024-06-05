multiversx_sc::imports!();

/// Used for testing the egld_decimal function return type
#[multiversx_sc::module]
pub trait EgldDecimal {
    #[custom_proxy]
    #[payable("EGLD")]
    #[endpoint]
    fn returns_egld_decimal(&self) -> ManagedDecimal<Self::Api, ConstDecimals<18>> {
        self.call_value().egld_decimal()
    }
}
