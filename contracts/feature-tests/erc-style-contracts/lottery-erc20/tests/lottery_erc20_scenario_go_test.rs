use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn buy_all_tickets_different_accounts_go() {
    world().run("scenarios/buy-all-tickets-different-accounts.scen.json");
}

#[test]
fn buy_more_tickets_than_allowed_go() {
    world().run("scenarios/buy-more-tickets-than-allowed.scen.json");
}

#[test]
fn buy_ticket_go() {
    world().run("scenarios/buy-ticket.scen.json");
}

#[test]
fn buy_ticket_after_deadline_go() {
    world().run("scenarios/buy-ticket-after-deadline.scen.json");
}

#[test]
fn buy_ticket_after_determined_winner_go() {
    world().run("scenarios/buy-ticket-after-determined-winner.scen.json");
}

#[test]
fn buy_ticket_after_sold_out_go() {
    world().run("scenarios/buy-ticket-after-sold-out.scen.json");
}

#[test]
fn buy_ticket_all_options_go() {
    world().run("scenarios/buy-ticket-all-options.scen.json");
}

#[test]
fn buy_ticket_another_account_go() {
    world().run("scenarios/buy-ticket-another-account.scen.json");
}

#[test]
fn buy_ticket_not_on_whitelist_go() {
    world().run("scenarios/buy-ticket-not-on-whitelist.scen.json");
}

#[test]
fn buy_ticket_same_account_go() {
    world().run("scenarios/buy-ticket-same-account.scen.json");
}

#[test]
fn buy_ticket_second_lottery_go() {
    world().run("scenarios/buy-ticket-second-lottery.scen.json");
}

#[test]
fn buy_ticket_wrong_fee_go() {
    world().run("scenarios/buy-ticket-wrong-fee.scen.json");
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc_1_go() {
    world().run("scenarios/determine-winner-different-ticket-holders-winner-acc1.scen.json");
}

#[test]
fn determine_winner_early_go() {
    world().run("scenarios/determine-winner-early.scen.json");
}

#[test]
fn determine_winner_same_ticket_holder_go() {
    world().run("scenarios/determine-winner-same-ticket-holder.scen.json");
}

#[test]
#[ignore]
fn determine_winner_split_prize_pool_go() {
    world().run("scenarios/determine-winner-split-prize-pool.scen.json");
}

#[test]
fn lottery_init_go() {
    world().run("scenarios/lottery-init.scen.json");
}

#[test]
fn start_after_announced_winner_go() {
    world().run("scenarios/start-after-announced-winner.scen.json");
}

#[test]
fn start_all_options_bigger_whitelist_go() {
    world().run("scenarios/start-all-options-bigger-whitelist.scen.json");
}

#[test]
fn start_alternative_function_name_go() {
    world().run("scenarios/start-alternative-function-name.scen.json");
}

#[test]
fn start_fixed_deadline_go() {
    world().run("scenarios/start-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_go() {
    world().run("scenarios/start-limited-tickets.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_go() {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_go() {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_go() {
    world().run(
        "scenarios/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
    );
}

#[test]
fn start_second_lottery_go() {
    world().run("scenarios/start-second-lottery.scen.json");
}

#[test]
fn start_with_all_options_go() {
    world().run("scenarios/start-with-all-options.scen.json");
}

#[test]
fn start_with_no_options_go() {
    world().run("scenarios/start-with-no-options.scen.json");
}
