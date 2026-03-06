// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

use order_book_pair::*;

const ORDER_BOOK_PAIR_CODE_PATH: MxscPath = MxscPath::new("output/order-book-pair.mxsc.json");
const MATCH_PROVIDER_ADDRESS: TestAddress = TestAddress::new("match_provider");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER1_ADDRESS: TestAddress = TestAddress::new("user1");
const USER2_ADDRESS: TestAddress = TestAddress::new("user2");
const PAIR_ADDRESS: TestSCAddress = TestSCAddress::new("pair");
const BUSD_ABCDEF: TestTokenId = TestTokenId::new("BUSD-abcdef");
const WEGLD_ABCDEF: TestTokenId = TestTokenId::new("WEGLD-abcdef");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/examples/order-book/pair");
    blockchain.register_contract(ORDER_BOOK_PAIR_CODE_PATH, order_book_pair::ContractBuilder);
    blockchain
}

#[test]
fn cancel_all_orders_scen() {
    let mut world = world();
    cancel_all_orders_scen_steps(&mut world);
}

pub fn cancel_all_orders_scen_steps(world: &mut ScenarioWorld) {
    complete_setup_steps(world);

    create_buy_order_steps(world);

    world
        .tx()
        .id("cancelAllOrders")
        .from(USER1_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .cancel_all_orders_endpoint()
        .run();

    world
        .tx()
        .id("getAddressOrderIds")
        .from(USER1_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .get_address_order_ids(ScenarioValueRaw::new("address:user1"))
        .run();
}

#[test]
fn cancel_orders_scen() {
    let mut world = world();
    cancel_orders_scen_steps(&mut world);
}

pub fn cancel_orders_scen_steps(world: &mut ScenarioWorld) {
    complete_setup_steps(world);

    create_buy_order_steps(world);

    world
        .tx()
        .id("cancelOrders")
        .from(USER1_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .cancel_orders_endpoint(MultiValueVec::from(vec![0u64]))
        .run();

    world
        .tx()
        .id("getAddressOrderIds")
        .from(USER1_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .get_address_order_ids(ScenarioValueRaw::new("address:user1"))
        .run();
}

pub fn complete_setup_steps(world: &mut ScenarioWorld) {
    init_accounts_steps(world);

    deploy_steps(world);
}

pub fn create_buy_order_steps(world: &mut ScenarioWorld) {
    world
        .tx()
        .id("createBuyOrder")
        .from(USER1_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .create_buy_order_endpoint(ScenarioValueRaw::new(
            "biguint:200000|address:match_provider|0x00|biguint:1000|u64:0|u64:1000",
        ))
        .payment(Payment::try_new(BUSD_ABCDEF, 0, 2_000_000u64).unwrap())
        .run();
}

#[test]
fn create_buy_order_check_scen() {
    let mut world = world();
    create_buy_order_check_scen_steps(&mut world);
}

pub fn create_buy_order_check_scen_steps(world: &mut ScenarioWorld) {
    complete_setup_steps(world);

    create_buy_order_steps(world);

    world
        .tx()
        .id("getOrderById")
        .from(USER1_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .orders(0u64)
        .returns(ExpectValue(ScenarioValueRaw::new("u64:0|address:user1|address:match_provider|biguint:2000000|biguint:200000|0x00|biguint:1000|u64:0|u64:1000|u64:0|0x00")))
        .run();

    world
        .tx()
        .id("getAddressOrderIds")
        .from(USER1_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .get_address_order_ids(ScenarioValueRaw::new("address:user1"))
        .returns(ExpectValue(MultiValueVec::from(vec![0u64])))
        .run();
}

pub fn create_sell_order_steps(world: &mut ScenarioWorld) {
    world
        .tx()
        .id("createSellOrder")
        .from(USER2_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .create_sell_order_endpoint(ScenarioValueRaw::new(
            "biguint:2000000|address:match_provider|0x00|biguint:10000|u64:0|u64:1000",
        ))
        .payment(Payment::try_new(WEGLD_ABCDEF, 0, 200_000u64).unwrap())
        .run();
}

#[test]
fn create_sell_order_check_scen() {
    let mut world = world();
    create_sell_order_check_scen_steps(&mut world);
}

pub fn create_sell_order_check_scen_steps(world: &mut ScenarioWorld) {
    complete_setup_steps(world);

    create_sell_order_steps(world);

    world
        .tx()
        .id("getOrderById")
        .from(USER2_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .orders(0u64)
        .returns(ExpectValue(ScenarioValueRaw::new("u64:0|address:user2|address:match_provider|biguint:200000|biguint:2000000|0x00|biguint:10000|u64:0|u64:1000|u64:0|0x01")))
        .run();

    world
        .tx()
        .id("getAddressOrderIds")
        .from(USER2_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .get_address_order_ids(ScenarioValueRaw::new("address:user2"))
        .returns(ExpectValue(MultiValueVec::from(vec![0u64])))
        .run();
}

pub fn deploy_steps(world: &mut ScenarioWorld) {
    world
        .tx()
        .id("deploy-router")
        .from(OWNER_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .init(WEGLD_ABCDEF, BUSD_ABCDEF)
        .code(ORDER_BOOK_PAIR_CODE_PATH)
        .new_address(PAIR_ADDRESS)
        .run();
}

#[test]
fn free_orders_scen() {
    let mut world = world();
    free_orders_scen_steps(&mut world);
}

pub fn free_orders_scen_steps(world: &mut ScenarioWorld) {
    complete_setup_steps(world);

    create_buy_order_steps(world);

    create_sell_order_steps(world);

    world.current_block().block_epoch(50u64);

    world
        .tx()
        .id("freeOrders")
        .from(MATCH_PROVIDER_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .free_orders_endpoint(MultiValueVec::from(vec![0u64, 1u64]))
        .run();
}

pub fn init_accounts_steps(world: &mut ScenarioWorld) {
    world
        .account(MATCH_PROVIDER_ADDRESS)
        .nonce(0u64)
        .balance(5_000_000_000u64)
        .esdt_balance(BUSD_ABCDEF, 5_000_000_000u64)
        .esdt_balance(WEGLD_ABCDEF, 5_000_000_000u64);
    world
        .account(OWNER_ADDRESS)
        .nonce(0u64)
        .balance(5_000_000_000u64)
        .esdt_balance(BUSD_ABCDEF, 5_000_000_000u64)
        .esdt_balance(WEGLD_ABCDEF, 5_000_000_000u64);
    world
        .account(USER1_ADDRESS)
        .nonce(0u64)
        .balance(5_000_000_000u64)
        .esdt_balance(BUSD_ABCDEF, 5_000_000_000u64)
        .esdt_balance(WEGLD_ABCDEF, 5_000_000_000u64);
    world
        .account(USER2_ADDRESS)
        .nonce(0u64)
        .balance(5_000_000_000u64)
        .esdt_balance(BUSD_ABCDEF, 5_000_000_000u64)
        .esdt_balance(WEGLD_ABCDEF, 5_000_000_000u64);
}

#[test]
fn match_orders_scen() {
    let mut world = world();
    match_orders_scen_steps(&mut world);
}

pub fn match_orders_scen_steps(world: &mut ScenarioWorld) {
    complete_setup_steps(world);

    create_buy_order_steps(world);

    create_sell_order_steps(world);

    world
        .tx()
        .id("matchOrders")
        .from(MATCH_PROVIDER_ADDRESS)
        .to(PAIR_ADDRESS)
        .typed(order_book_pair_proxy::OrderBookPairProxy)
        .match_orders_endpoint(ScenarioValueRaw::new("u64:0|u64:1"))
        .run();
}
