use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/examples/nft-minter");
    blockchain.register_contract(
        "mxsc:output/nft-minter.mxsc.json",
        nft_minter::ContractBuilder,
    );
    blockchain
}

#[test]
fn buy_nft_rs() {
    world().run("scenarios/buy_nft.scen.json");
}

#[test]
fn create_nft_rs() {
    world().run("scenarios/create_nft.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}
