use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/multi-contract-features");

    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "file:output/multi-contract-features.wasm",
        multi_contract_features::ContractBuilder,
        "multi-contract-features",
    );
    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "file:output/multi-contract-features-view.wasm",
        multi_contract_features::ContractBuilder,
        "multi-contract-features-view",
    );

    blockchain
}

#[test]
#[ignore] // not supported
fn mcf_example_feature_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mcf-example-feature.scen.json", world());
}

#[test]
fn mcf_external_get_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mcf-external-get.scen.json", world());
}

#[test]
fn mcf_external_pure_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mcf-external-pure.scen.json", world());
}
