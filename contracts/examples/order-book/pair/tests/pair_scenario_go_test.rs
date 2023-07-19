#[test]
fn cancel_all_orders_go() {
    multiversx_sc_scenario::run_go("scenarios/cancel_all_orders.scen.json");
}

#[test]
fn cancel_orders_go() {
    multiversx_sc_scenario::run_go("scenarios/cancel_orders.scen.json");
}

#[test]
fn create_buy_order_check_go() {
    multiversx_sc_scenario::run_go("scenarios/create_buy_order_check.scen.json");
}

#[test]
fn create_sell_order_check_go() {
    multiversx_sc_scenario::run_go("scenarios/create_sell_order_check.scen.json");
}

#[test]
fn free_orders_go() {
    multiversx_sc_scenario::run_go("scenarios/free_orders.scen.json");
}

#[test]
fn match_orders_go() {
    multiversx_sc_scenario::run_go("scenarios/match_orders.scen.json");
}
