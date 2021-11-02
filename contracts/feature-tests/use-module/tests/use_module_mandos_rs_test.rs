mod user_builtin {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait UserBuiltin {
        #[endpoint(SetUserName)]
        fn set_user_name(&self, name: &BoxedBytes) -> BigUint;
    }
}

mod dns_mock {
    elrond_wasm::imports!();

    #[elrond_wasm::contract]
    pub trait DnsMock {
        #[proxy]
        fn user_builtin_proxy(&self, to: ManagedAddress) -> super::user_builtin::Proxy<Self::Api>;

        #[payable("EGLD")]
        #[endpoint]
        fn register(&self, name: BoxedBytes, #[payment] _payment: BigUint) -> AsyncCall {
            let address = self.blockchain().get_caller();
            self.user_builtin_proxy(address)
                .set_user_name(&name)
                .async_call()
        }
    }
}

use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/use-module.wasm",
        Box::new(|context| Box::new(use_module::contract_obj(context))),
    );

    blockchain.register_contract(
        "file:test-wasm/dns.wasm",
        Box::new(|context| Box::new(dns_mock::contract_obj(context))),
    );

    blockchain
}

#[test]
fn use_module_dns_register_rs() {
    elrond_wasm_debug::mandos_rs("mandos/use_module_dns_register.scen.json", world());
}

#[test]
fn use_module_features_rs() {
    elrond_wasm_debug::mandos_rs("mandos/use_module_features.scen.json", world());
}

#[test]
fn use_module_internal_rs() {
    elrond_wasm_debug::mandos_rs("mandos/use_module_internal.scen.json", world());
}

#[test]
fn use_module_no_endpoint_rs() {
    elrond_wasm_debug::mandos_rs("mandos/use_module_no_endpoint.scen.json", world());
}

#[test]
fn use_module_pause_rs() {
    elrond_wasm_debug::mandos_rs("mandos/use_module_pause.scen.json", world());
}

// Governance module tests

#[test]
fn cancel_defeated_proposal_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/use_module_governance/cancel_defeated_proposal.scen.json",
        world(),
    );
}

#[test]
fn change_configuration_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/use_module_governance/change_configuration.scen.json",
        world(),
    );
}

#[test]
fn init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/use_module_governance/init.scen.json", world());
}

#[test]
fn invalid_proposals_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/use_module_governance/invalid_proposals.scen.json",
        world(),
    );
}

#[test]
fn withdraw_governance_tokens_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/use_module_governance/withdraw_governance_tokens.scen.json",
        world(),
    );
}
