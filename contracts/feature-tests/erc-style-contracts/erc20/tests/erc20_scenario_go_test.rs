use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn allowance_caller_caller_go() {
    world().run("scenarios/allowance_CallerCaller.scen.json");
}

#[test]
fn allowance_caller_other_go() {
    world().run("scenarios/allowance_CallerOther.scen.json");
}

#[test]
fn allowance_other_caller_go() {
    world().run("scenarios/allowance_OtherCaller.scen.json");
}

#[test]
fn allowance_other_eq_other_go() {
    world().run("scenarios/allowance_OtherEqOther.scen.json");
}

#[test]
fn allowance_other_n_eq_other_go() {
    world().run("scenarios/allowance_OtherNEqOther.scen.json");
}

#[test]
fn approve_caller_positive_go() {
    world().run("scenarios/approve_Caller-Positive.scen.json");
}

#[test]
fn approve_caller_zero_go() {
    world().run("scenarios/approve_Caller-Zero.scen.json");
}

#[test]
fn approve_other_positive_go() {
    world().run("scenarios/approve_Other-Positive.scen.json");
}

#[test]
fn approve_other_zero_go() {
    world().run("scenarios/approve_Other-Zero.scen.json");
}

#[test]
fn approve_switch_caller_go() {
    world().run("scenarios/approve_SwitchCaller.scen.json");
}

#[test]
fn balance_of_caller_go() {
    world().run("scenarios/balanceOf_Caller.scen.json");
}

#[test]
fn balance_of_non_caller_go() {
    world().run("scenarios/balanceOf_NonCaller.scen.json");
}

#[test]
fn not_payable_go() {
    world().run("scenarios/not_payable.scen.json");
}

#[test]
fn not_payable_esdt_go() {
    world().run("scenarios/not_payable_esdt.scen.json");
}

#[test]
fn total_supply_positive_go() {
    world().run("scenarios/totalSupply_Positive.scen.json");
}

#[test]
fn total_supply_zero_go() {
    world().run("scenarios/totalSupply_Zero.scen.json");
}

#[test]
fn transfer_from_all_distinct_balance_eq_allowance_go() {
    world().run("scenarios/transferFrom_AllDistinct-BalanceEqAllowance.scen.json");
}

#[test]
fn transfer_from_all_distinct_balance_n_eq_allowance_go() {
    world().run("scenarios/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json");
}

#[test]
fn transfer_from_all_distinct_entire_allowance_more_than_balance_go() {
    world().run("scenarios/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json");
}

#[test]
fn transfer_from_all_distinct_entire_balance_eq_allowance_go() {
    world().run("scenarios/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json");
}

#[test]
fn transfer_from_all_distinct_entire_balance_more_than_allowance_go() {
    world().run("scenarios/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json");
}

#[test]
fn transfer_from_all_distinct_more_than_allowance_less_than_balance_go() {
    world().run("scenarios/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json");
}

#[test]
fn transfer_from_all_distinct_more_than_balance_less_than_allowance_go() {
    world().run("scenarios/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json");
}

#[test]
fn transfer_from_all_distinct_no_overflow_go() {
    world().run("scenarios/transferFrom_AllDistinct-NoOverflow.scen.json");
}

#[test]
fn transfer_from_all_distinct_still_no_overflow_go() {
    world().run("scenarios/transferFrom_AllDistinct-StillNoOverflow.scen.json");
}

#[test]
fn transfer_from_all_equal_allowance_relevant_go() {
    world().run("scenarios/transferFrom_AllEqual-AllowanceRelevant.scen.json");
}

#[test]
fn transfer_from_all_equal_entire_balance_go() {
    world().run("scenarios/transferFrom_AllEqual-EntireBalance.scen.json");
}

#[test]
fn transfer_from_caller_eq_from_allowance_relevant_go() {
    world().run("scenarios/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json");
}

#[test]
fn transfer_from_caller_eq_from_entire_balance_go() {
    world().run("scenarios/transferFrom_CallerEqFrom-EntireBalance.scen.json");
}

#[test]
fn transfer_from_caller_eq_from_more_than_balance_go() {
    world().run("scenarios/transferFrom_CallerEqFrom-MoreThanBalance.scen.json");
}

#[test]
fn transfer_from_caller_eq_to_balance_n_eq_allowance_go() {
    world().run("scenarios/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json");
}

#[test]
fn transfer_from_caller_eq_to_more_than_allowance_less_than_balance_go() {
    world().run("scenarios/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json");
}

#[test]
fn transfer_from_caller_eq_to_more_than_balance_less_than_allowance_go() {
    world().run("scenarios/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json");
}

#[test]
fn transfer_from_exploratory_multiple_transfers_succeed_go() {
    world().run("scenarios/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json");
}

#[test]
fn transfer_from_exploratory_multiple_transfers_throw_go() {
    world().run("scenarios/transferFrom_Exploratory-MultipleTransfersThrow.scen.json");
}

#[test]
fn transfer_from_from_eq_to_balance_eq_allowance_go() {
    world().run("scenarios/transferFrom_FromEqTo-BalanceEqAllowance.scen.json");
}

#[test]
fn transfer_from_from_eq_to_balance_n_eq_allowance_go() {
    world().run("scenarios/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json");
}

#[test]
fn transfer_from_from_eq_to_entire_allowance_more_than_balance_go() {
    world().run("scenarios/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json");
}

#[test]
fn transfer_from_from_eq_to_entire_balance_eq_allowance_go() {
    world().run("scenarios/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json");
}

#[test]
fn transfer_from_from_eq_to_entire_balance_more_than_allowance_go() {
    world().run("scenarios/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json");
}

#[test]
fn transfer_from_from_eq_to_more_than_allowance_less_than_balance_go() {
    world().run("scenarios/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json");
}

#[test]
fn transfer_from_from_eq_to_more_than_balance_less_than_allowance_go() {
    world().run("scenarios/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json");
}

#[test]
fn transfer_from_from_eq_to_no_overflow_go() {
    world().run("scenarios/transferFrom_FromEqTo-NoOverflow.scen.json");
}

#[test]
fn transfer_caller_allowance_irrelevant_go() {
    world().run("scenarios/transfer_Caller-AllowanceIrrelevant.scen.json");
}

#[test]
fn transfer_caller_entire_balance_go() {
    world().run("scenarios/transfer_Caller-EntireBalance.scen.json");
}

#[test]
fn transfer_caller_more_than_balance_go() {
    world().run("scenarios/transfer_Caller-MoreThanBalance.scen.json");
}

#[test]
fn transfer_caller_no_overflow_go() {
    world().run("scenarios/transfer_Caller-NoOverflow.scen.json");
}

#[test]
fn transfer_caller_positive_go() {
    world().run("scenarios/transfer_Caller-Positive.scen.json");
}

#[test]
fn transfer_caller_still_no_overflow_go() {
    world().run("scenarios/transfer_Caller-StillNoOverflow.scen.json");
}

#[test]
fn transfer_caller_zero_go() {
    world().run("scenarios/transfer_Caller-Zero.scen.json");
}

#[test]
fn transfer_other_allowance_irrelevant_go() {
    world().run("scenarios/transfer_Other-AllowanceIrrelevant.scen.json");
}

#[test]
fn transfer_other_entire_balance_go() {
    world().run("scenarios/transfer_Other-EntireBalance.scen.json");
}

#[test]
fn transfer_other_more_than_balance_go() {
    world().run("scenarios/transfer_Other-MoreThanBalance.scen.json");
}

#[test]
fn transfer_other_no_overflow_go() {
    world().run("scenarios/transfer_Other-NoOverflow.scen.json");
}

#[test]
fn transfer_other_positive_go() {
    world().run("scenarios/transfer_Other-Positive.scen.json");
}

#[test]
fn transfer_other_still_no_overflow_go() {
    world().run("scenarios/transfer_Other-StillNoOverflow.scen.json");
}

#[test]
fn transfer_other_zero_go() {
    world().run("scenarios/transfer_Other-Zero.scen.json");
}
