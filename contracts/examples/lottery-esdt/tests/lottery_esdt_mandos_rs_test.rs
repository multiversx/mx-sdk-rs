use elrond_wasm::*;
use elrond_wasm_debug::*;

#[allow(dead_code)]
fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lottery-esdt");

    blockchain.register_contract(
        "file:output/lottery-esdt.wasm",
        Box::new(|context| Box::new(lottery_esdt::contract_obj(context))),
    );
    blockchain
}

#[test]
fn buy_all_tickets_different_accounts_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/buy-all-tickets-different-accounts.scen.json",
        world(),
    );
}

#[test]
fn buy_more_tickets_than_allowed_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-more-tickets-than-allowed.scen.json", world());
}

#[test]
fn buy_ticket_after_deadline_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-after-deadline.scen.json", world());
}

#[test]
fn buy_ticket_after_determined_winner_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/buy-ticket-after-determined-winner.scen.json",
        world(),
    );
}

#[test]
fn buy_ticket_after_sold_out_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-after-sold-out.scen.json", world());
}

#[test]
fn buy_ticket_all_options_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-all-options.scen.json", world());
}

#[test]
fn buy_ticket_another_account_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-another-account.scen.json", world());
}

#[test]
fn buy_ticket_not_on_whitelist_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-not-on-whitelist.scen.json", world());
}

#[test]
fn buy_ticket_same_account_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-same-account.scen.json", world());
}

#[test]
fn buy_ticket_second_lottery_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-second-lottery.scen.json", world());
}

#[test]
fn buy_ticket_wrong_fee_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket-wrong-fee.scen.json", world());
}

#[test]
fn buy_ticket_simple_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy-ticket.scen.json", world());
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc1_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/determine-winner-different-ticket-holders-winner-acc1.scen.json",
        world(),
    );
}

#[test]
fn determine_winner_early_rs() {
    elrond_wasm_debug::mandos_rs("mandos/determine-winner-early.scen.json", world());
}

#[test]
fn determine_winner_same_ticket_holder_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/determine-winner-same-ticket-holder.scen.json",
        world(),
    );
}

/* NOT SUPPORTED YET
#[test]
fn determine_winner_split_prize_pool_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/determine-winner-split-prize-pool.scen.json",
        world(),
    );
}
*/

#[test]
fn lottery_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-init.scen.json", world());
}

#[test]
fn start_after_announced_winner_rs() {
    elrond_wasm_debug::mandos_rs("mandos/start-after-announced-winner.scen.json", world());
}

#[test]
fn start_all_options_bigger_whitelist_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/start-all-options-bigger-whitelist.scen.json",
        world(),
    );
}

#[test]
fn start_alternative_function_name_rs() {
    elrond_wasm_debug::mandos_rs("mandos/start-alternative-function-name.scen.json", world());
}

#[test]
fn start_fixed_deadline_rs() {
    elrond_wasm_debug::mandos_rs("mandos/start-fixed-deadline.scen.json", world());
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json",
        world(),
    );
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
        world(),
    );
}

#[test]
fn start_limited_tickets_and_fixed_deadline_valid_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/start-limited-tickets-and-fixed-deadline.scen.json",
        world(),
    );
}

#[test]
fn start_limited_tickets_simple_rs() {
    elrond_wasm_debug::mandos_rs("mandos/start-limited-tickets.scen.json", world());
}

#[test]
fn start_second_lottery_rs() {
    elrond_wasm_debug::mandos_rs("mandos/start-second-lottery.scen.json", world());
}

#[test]
fn start_with_all_options_rs() {
    elrond_wasm_debug::mandos_rs("mandos/start-with-all-options.scen.json", world());
}

#[test]
fn start_with_no_options_rs() {
    elrond_wasm_debug::mandos_rs("mandos/start-with-no-options.scen.json", world());
}
