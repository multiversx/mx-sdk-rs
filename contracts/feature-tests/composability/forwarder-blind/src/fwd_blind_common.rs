multiversx_sc::imports!();

pub const GAS_OVERHEAD: u64 = 7_000_000;

#[multiversx_sc::module]
pub trait ForwarderBlindCommon {
    fn tx_gas(&self) -> u64 {
        require!(
            self.blockchain().get_gas_left() > GAS_OVERHEAD,
            "not enough gas for forwarding"
        );

        self.blockchain().get_gas_left() - GAS_OVERHEAD
    }

    fn send_back_payments(
        &self,
        scenario: &str,
        original_caller: &ManagedAddress,
        payments: &PaymentVec,
    ) {
        if !payments.is_empty() {
            self.send_back_payments_event(
                scenario,
                original_caller,
                payments.clone().into_multi_value(),
            );
            self.tx().to(original_caller).payment(payments).transfer();
        }
    }

    #[event("blindBackPayments")]
    fn send_back_payments_event(
        &self,
        #[indexed] scenario: &str,
        #[indexed] original_caller: &ManagedAddress,
        #[indexed] payments: MultiValueEncoded<PaymentMultiValue>,
    );
}
