use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/erc-style-contracts/lottery-erc20",
    );

    blockchain.register_contract(
        "file:output/lottery-erc20.wasm",
        lottery_erc20::ContractBuilder,
    );

    blockchain.register_contract("file:../erc20/output/erc20.wasm", erc20::ContractBuilder);

    blockchain
}

#[test]
fn buy_all_tickets_different_accounts_rs() {
    world().run("scenarios/buy-all-tickets-different-accounts.scen.json");
}

#[test]
fn buy_more_tickets_than_allowed_rs() {
    world().run("scenarios/buy-more-tickets-than-allowed.scen.json");
}

#[test]
fn buy_ticket_rs() {
    world().run("scenarios/buy-ticket.scen.json");
}

#[test]
fn buy_ticket_after_deadline_rs() {
    world().run("scenarios/buy-ticket-after-deadline.scen.json");
}

#[test]
fn buy_ticket_after_determined_winner_rs() {
    world().run("scenarios/buy-ticket-after-determined-winner.scen.json");
}

#[test]
fn buy_ticket_after_sold_out_rs() {
    world().run("scenarios/buy-ticket-after-sold-out.scen.json");
}

#[test]
fn buy_ticket_all_options_rs() {
    world().run("scenarios/buy-ticket-all-options.scen.json");
}

#[test]
fn buy_ticket_another_account_rs() {
    world().run("scenarios/buy-ticket-another-account.scen.json");
}

#[test]
fn buy_ticket_not_on_whitelist_rs() {
    world().run("scenarios/buy-ticket-not-on-whitelist.scen.json");
}

#[test]
fn buy_ticket_same_account_rs() {
    world().run("scenarios/buy-ticket-same-account.scen.json");
}

#[test]
fn buy_ticket_second_lottery_rs() {
    world().run("scenarios/buy-ticket-second-lottery.scen.json");
}

#[test]
fn buy_ticket_wrong_fee_rs() {
    world().run("scenarios/buy-ticket-wrong-fee.scen.json");
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc_1_rs() {
    world().run("scenarios/determine-winner-different-ticket-holders-winner-acc1.scen.json");
}

#[test]
fn determine_winner_early_rs() {
    world().run("scenarios/determine-winner-early.scen.json");
}

#[test]
fn determine_winner_same_ticket_holder_rs() {
    world().run("scenarios/determine-winner-same-ticket-holder.scen.json");
}

// TODO: un-ignore after the scenario runner supports chaining async calls
#[test]
#[ignore]
fn determine_winner_split_prize_pool_rs() {
    world().run("scenarios/determine-winner-split-prize-pool.scen.json");
}

#[test]
fn lottery_init_rs() {
    world().run("scenarios/lottery-init.scen.json");
}

#[test]
fn start_after_announced_winner_rs() {
    world().run("scenarios/start-after-announced-winner.scen.json");
}

#[test]
fn start_all_options_bigger_whitelist_rs() {
    world().run("scenarios/start-all-options-bigger-whitelist.scen.json");
}

#[test]
fn start_alternative_function_name_rs() {
    world().run("scenarios/start-alternative-function-name.scen.json");
}

#[test]
fn start_fixed_deadline_rs() {
    world().run("scenarios/start-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_rs() {
    world().run("scenarios/start-limited-tickets.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_rs() {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_rs() {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_rs() {
    world().run(
        "scenarios/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
    );
}

#[test]
fn start_second_lottery_rs() {
    world().run("scenarios/start-second-lottery.scen.json");
}

#[test]
fn start_with_all_options_rs() {
    world().run("scenarios/start-with-all-options.scen.json");
}

#[test]
fn start_with_no_options_rs() {
    world().run("scenarios/start-with-no-options.scen.json");
}
