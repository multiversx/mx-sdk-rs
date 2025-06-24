use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/multi-contract-features");

    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "mxsc:output/multi-contract-features.mxsc.json",
        multi_contract_features::ContractBuilder,
        "multi-contract-features",
    );
    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "mxsc:output/multi-contract-features-view.mxsc.json",
        multi_contract_features::ContractBuilder,
        "multi-contract-features-view",
    );
    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "mxsc:output/multi-contract-alt-impl.mxsc.json",
        multi_contract_features::ContractBuilder,
        "multi-contract-alt-impl",
    );

    blockchain
}

#[test]
#[ignore = "not yet supported"]
fn mcf_alt_init_rs() {
    world().run("scenarios/mcf-alt-init.scen.json");
}

#[test]
#[ignore = "not supported in principle"]
fn mcf_example_feature_rs() {
    world().run("scenarios/mcf-example-feature.scen.json");
}

#[test]
fn mcf_external_get_rs() {
    world().run("scenarios/mcf-external-get.scen.json");
}

#[test]
fn mcf_external_pure_rs() {
    world().run("scenarios/mcf-external-pure.scen.json");
}
