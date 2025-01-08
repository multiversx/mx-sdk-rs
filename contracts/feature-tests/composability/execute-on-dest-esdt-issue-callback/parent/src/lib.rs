#![no_std]

multiversx_sc::imports!();

pub mod child_proxy;

// Base cost for standalone + estimate cost of actual sc call
const ISSUE_EXPECTED_GAS_COST: u64 = 90_000_000 + 25_000_000;

#[multiversx_sc::contract]
pub trait Parent {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint]
    fn deposit(&self) {}

    #[endpoint(deployChildContract)]
    fn deploy_child_contract(&self, code: ManagedBuffer) {
        let gas_left = self.blockchain().get_gas_left();
        let child_contract_address = self
            .tx()
            .raw_deploy()
            .code(code)
            .gas(gas_left)
            .returns(ReturnsNewManagedAddress)
            .sync_call();

        self.child_contract_address().set(&child_contract_address);
    }

    #[payable("EGLD")]
    #[endpoint(executeOnDestIssueToken)]
    fn execute_on_dest_issue_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let issue_cost = self.call_value().single_egld_value();
        let child_contract_adress = self.child_contract_address().get();

        self.tx()
            .to(&child_contract_adress)
            .typed(child_proxy::ChildProxy)
            .issue_wrapped_egld(token_display_name, token_ticker, initial_supply)
            .egld(issue_cost)
            .gas(ISSUE_EXPECTED_GAS_COST)
            .sync_call();
    }

    // storage

    #[view(getChildContractAddress)]
    #[storage_mapper("childContractAddress")]
    fn child_contract_address(&self) -> SingleValueMapper<ManagedAddress>;
}
