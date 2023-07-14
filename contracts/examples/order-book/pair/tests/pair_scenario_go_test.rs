use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn cancel_all_orders_go() {
    world().run("scenarios/cancel_all_orders.scen.json");
}

#[test]
fn cancel_orders_go() {
    world().run("scenarios/cancel_orders.scen.json");
}

#[test]
fn create_buy_order_check_go() {
    world().run("scenarios/create_buy_order_check.scen.json");
}

#[test]
fn create_sell_order_check_go() {
    world().run("scenarios/create_sell_order_check.scen.json");
}

#[test]
fn free_orders_go() {
    world().run("scenarios/free_orders.scen.json");
}

#[test]
fn match_orders_go() {
    world().run("scenarios/match_orders.scen.json");
}
