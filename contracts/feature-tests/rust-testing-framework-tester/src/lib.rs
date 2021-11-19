#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait RustTestingFrameworkTester {
    #[init]
    fn init(&self) {
        self.total_value().set(&BigUint::from(1u32));
    }

    #[endpoint]
    fn sum(&self, first: BigUint, second: BigUint) -> BigUint {
        first + second
    }

    #[endpoint]
    fn sum_sc_result(&self, first: BigUint, second: BigUint) -> SCResult<BigUint> {
        require!(first > 0 && second > 0, "Non-zero required");
        Ok(first + second)
    }

    #[endpoint]
    fn get_caller_legacy(&self) -> Address {
        self.blockchain().get_caller_legacy()
    }

    #[endpoint]
    fn get_egld_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::egld(), 0)
    }

    #[payable("EGLD")]
    #[endpoint]
    fn receive_egld(&self) -> BigUint {
        self.call_value().egld_value()
    }

    #[payable("EGLD")]
    #[endpoint]
    fn recieve_egld_half(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value() / 2u32;
        self.send()
            .direct(&caller, &TokenIdentifier::egld(), 0, &payment_amount, &[]);
    }

    #[endpoint]
    fn add(&self, value: BigUint) {
        let caller = self.blockchain().get_caller();

        self.total_value().update(|val| *val += &value);
        self.value_per_caller(&caller).update(|val| *val += value);
    }

    #[storage_mapper("totalValue")]
    fn total_value(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("valuePerCaller")]
    fn value_per_caller(&self, caller: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
