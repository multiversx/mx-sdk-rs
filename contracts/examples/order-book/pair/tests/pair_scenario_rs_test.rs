use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/order-book/pair");

    blockchain.register_contract(
        "file:output/order-book-pair.wasm",
        order_book_pair::ContractBuilder,
    );
    blockchain
}

#[test]
fn cancel_all_orders_rs() {
    world().run("scenarios/cancel_all_orders.scen.json");
}

#[test]
fn cancel_orders_rs() {
    world().run("scenarios/cancel_orders.scen.json");
}

#[test]
fn create_buy_order_check_rs() {
    world().run("scenarios/create_buy_order_check.scen.json");
}

#[test]
fn create_sell_order_check_rs() {
    world().run("scenarios/create_sell_order_check.scen.json");
}

#[test]
fn free_orders_rs() {
    world().run("scenarios/free_orders.scen.json");
}

#[test]
fn match_orders_rs() {
    world().run("scenarios/match_orders.scen.json");
}
