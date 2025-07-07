use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/erc-style-contracts/erc1155-marketplace",
    );
    blockchain.register_contract(
        "mxsc:output/erc1155-marketplace.mxsc.json",
        erc1155_marketplace::ContractBuilder,
    );
    blockchain.register_contract(
        "mxsc:../erc1155/output/erc1155.mxsc.json",
        erc1155::ContractBuilder,
    );

    blockchain
}

#[test]
fn auction_batch_rs() {
    world().run("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_egld_rs() {
    world().run("scenarios/auction_single_token_egld.scen.json");
}

#[test]
fn bid_first_egld_rs() {
    world().run("scenarios/bid_first_egld.scen.json");
}

#[test]
fn bid_second_egld_rs() {
    world().run("scenarios/bid_second_egld.scen.json");
}

#[test]
fn bid_third_egld_rs() {
    world().run("scenarios/bid_third_egld.scen.json");
}

#[test]
fn end_auction_rs() {
    world().run("scenarios/end_auction.scen.json");
}
