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
        fn register(&self, name: BoxedBytes) {
            let _payment = self.call_value().egld_value();
            let address = self.blockchain().get_caller();
            self.user_builtin_proxy(address)
                .set_user_name(&name)
                .async_call()
                .call_and_exit()
        }
    }
}

use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain
        .register_contract_builder("file:output/use-module.wasm", use_module::ContractBuilder);

    blockchain.register_contract_builder(
        "file:test-wasm/elrond-wasm-sc-dns.wasm",
        dns_mock::ContractBuilder,
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
