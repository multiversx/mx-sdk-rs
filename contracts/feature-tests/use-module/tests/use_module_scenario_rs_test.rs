mod dns_mock {
    multiversx_sc::imports!();

    #[multiversx_sc::contract]
    pub trait DnsMock {
        #[payable("EGLD")]
        #[endpoint]
        fn register(&self, name: BoxedBytes) {
            let _payment = self.call_value().egld();
            let address = self.blockchain().get_caller();
            self.tx()
                .to(&address)
                .typed(system_proxy::UserBuiltinProxy)
                .set_user_name(&name)
                .async_call_and_exit();
        }
    }
}

use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/use-module");
    blockchain.register_contract(
        "mxsc:output/use-module.mxsc.json",
        use_module::ContractBuilder,
    );

    blockchain.register_contract(
        "mxsc:test-wasm/elrond-wasm-sc-dns.mxsc.json",
        dns_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn use_module_claim_developer_rewards_rs() {
    world().run("scenarios/use_module_claim_developer_rewards.scen.json");
}

#[test]
fn use_module_dns_register_rs() {
    world().run("scenarios/use_module_dns_register.scen.json");
}

#[test]
fn use_module_features_rs() {
    world().run("scenarios/use_module_features.scen.json");
}

#[test]
fn use_module_internal_rs() {
    world().run("scenarios/use_module_internal.scen.json");
}

#[test]
fn use_module_no_endpoint_rs() {
    world().run("scenarios/use_module_no_endpoint.scen.json");
}

/// Will not work in scenarios-rs, since there is no gas usage
#[test]
#[ignore]
fn use_module_ongoing_operation_example_rs() {
    world().run("scenarios/use_module_ongoing_operation_example.scen.json");
}

#[test]
fn use_module_only_admin_rs() {
    world().run("scenarios/use_module_only_admin.scen.json");
}

#[test]
fn use_module_only_owner_rs() {
    world().run("scenarios/use_module_only_owner.scen.json");
}

#[test]
fn use_module_pause_rs() {
    world().run("scenarios/use_module_pause.scen.json");
}
