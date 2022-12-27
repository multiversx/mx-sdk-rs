#[test]
fn buy_all_tickets_different_accounts_go() {
    mx_sc_debug::mandos_go("scenarios/buy-all-tickets-different-accounts.scen.json");
}

#[test]
fn buy_more_tickets_than_allowed_go() {
    mx_sc_debug::mandos_go("scenarios/buy-more-tickets-than-allowed.scen.json");
}

#[test]
fn buy_ticket_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket.scen.json");
}

#[test]
fn buy_ticket_after_deadline_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-after-deadline.scen.json");
}

#[test]
fn buy_ticket_after_determined_winner_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-after-determined-winner.scen.json");
}

#[test]
fn buy_ticket_after_sold_out_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-after-sold-out.scen.json");
}

#[test]
fn buy_ticket_all_options_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-all-options.scen.json");
}

#[test]
fn buy_ticket_another_account_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-another-account.scen.json");
}

#[test]
fn buy_ticket_not_on_whitelist_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-not-on-whitelist.scen.json");
}

#[test]
fn buy_ticket_same_account_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-same-account.scen.json");
}

#[test]
fn buy_ticket_second_lottery_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-second-lottery.scen.json");
}

#[test]
fn buy_ticket_wrong_fee_go() {
    mx_sc_debug::mandos_go("scenarios/buy-ticket-wrong-fee.scen.json");
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc1_go() {
    mx_sc_debug::mandos_go(
        "scenarios/determine-winner-different-ticket-holders-winner-acc1.scen.json",
    );
}

#[test]
fn determine_winner_early_go() {
    mx_sc_debug::mandos_go("scenarios/determine-winner-early.scen.json");
}

#[test]
fn determine_winner_same_ticket_holder_go() {
    mx_sc_debug::mandos_go("scenarios/determine-winner-same-ticket-holder.scen.json");
}

// #[test]
// fn determine_winner_split_prize_pool_go() {
// 	mx_sc_debug::mandos_go("scenarios/determine-winner-split-prize-pool.scen.json");
// }

#[test]
fn lottery_init_go() {
    mx_sc_debug::mandos_go("scenarios/lottery-init.scen.json");
}

#[test]
fn start_after_announced_winner_go() {
    mx_sc_debug::mandos_go("scenarios/start-after-announced-winner.scen.json");
}

#[test]
fn start_all_options_bigger_whitelist_go() {
    mx_sc_debug::mandos_go("scenarios/start-all-options-bigger-whitelist.scen.json");
}

#[test]
fn start_alternative_function_name_go() {
    mx_sc_debug::mandos_go("scenarios/start-alternative-function-name.scen.json");
}

#[test]
fn start_fixed_deadline_go() {
    mx_sc_debug::mandos_go("scenarios/start-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_go() {
    mx_sc_debug::mandos_go("scenarios/start-limited-tickets.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_go() {
    mx_sc_debug::mandos_go("scenarios/start-limited-tickets-and-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_go() {
    mx_sc_debug::mandos_go(
        "scenarios/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json",
    );
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_go() {
    mx_sc_debug::mandos_go(
        "scenarios/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
    );
}

#[test]
fn start_second_lottery_go() {
    mx_sc_debug::mandos_go("scenarios/start-second-lottery.scen.json");
}

#[test]
fn start_with_all_options_go() {
    mx_sc_debug::mandos_go("scenarios/start-with-all-options.scen.json");
}

#[test]
fn start_with_no_options_go() {
    mx_sc_debug::mandos_go("scenarios/start-with-no-options.scen.json");
}
