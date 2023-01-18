use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/erc-style-contracts/erc20");

    blockchain.register_contract("file:output/erc20.wasm", erc20::ContractBuilder);
    blockchain
}

#[test]
fn allowance_callercaller_rs() {
    multiversx_sc_scenario::run_rs("scenarios/allowance_CallerCaller.scen.json", world());
}

#[test]
fn allowance_callerother_rs() {
    multiversx_sc_scenario::run_rs("scenarios/allowance_CallerOther.scen.json", world());
}

#[test]
fn allowance_othercaller_rs() {
    multiversx_sc_scenario::run_rs("scenarios/allowance_OtherCaller.scen.json", world());
}

#[test]
fn allowance_othereqother_rs() {
    multiversx_sc_scenario::run_rs("scenarios/allowance_OtherEqOther.scen.json", world());
}

#[test]
fn allowance_otherneqother_rs() {
    multiversx_sc_scenario::run_rs("scenarios/allowance_OtherNEqOther.scen.json", world());
}

#[test]
fn approve_caller_positive_rs() {
    multiversx_sc_scenario::run_rs("scenarios/approve_Caller-Positive.scen.json", world());
}

#[test]
fn approve_caller_zero_rs() {
    multiversx_sc_scenario::run_rs("scenarios/approve_Caller-Zero.scen.json", world());
}

#[test]
fn approve_other_positive_rs() {
    multiversx_sc_scenario::run_rs("scenarios/approve_Other-Positive.scen.json", world());
}

#[test]
fn approve_other_zero_rs() {
    multiversx_sc_scenario::run_rs("scenarios/approve_Other-Zero.scen.json", world());
}

#[test]
fn approve_switchcaller_rs() {
    multiversx_sc_scenario::run_rs("scenarios/approve_SwitchCaller.scen.json", world());
}

#[test]
fn balanceof_caller_rs() {
    multiversx_sc_scenario::run_rs("scenarios/balanceOf_Caller.scen.json", world());
}

#[test]
fn balanceof_noncaller_rs() {
    multiversx_sc_scenario::run_rs("scenarios/balanceOf_NonCaller.scen.json", world());
}

#[test]
fn not_payable_rs() {
    multiversx_sc_scenario::run_rs("scenarios/not_payable.scen.json", world());
}

#[test]
fn totalsupply_positive_rs() {
    multiversx_sc_scenario::run_rs("scenarios/totalSupply_Positive.scen.json", world());
}

#[test]
fn totalsupply_zero_rs() {
    multiversx_sc_scenario::run_rs("scenarios/totalSupply_Zero.scen.json", world());
}

#[test]
fn transferfrom_alldistinct_balanceeqallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-BalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_balanceneqallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_entireallowancemorethanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_entirebalanceeqallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_entirebalancemorethanallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_morethanallowancelessthanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_morethanbalancelessthanallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_nooverflow_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-NoOverflow.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_stillnooverflow_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllDistinct-StillNoOverflow.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_allequal_allowancerelevant_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllEqual-AllowanceRelevant.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_allequal_entirebalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_AllEqual-EntireBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqfrom_allowancerelevant_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqfrom_entirebalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_CallerEqFrom-EntireBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqfrom_morethanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_CallerEqFrom-MoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqto_balanceneqallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqto_morethanallowancelessthanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqto_morethanbalancelessthanallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_exploratory_multipletransferssucceed_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_exploratory_multipletransfersthrow_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_Exploratory-MultipleTransfersThrow.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_balanceeqallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-BalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_balanceneqallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_entireallowancemorethanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_entirebalanceeqallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_entirebalancemorethanallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_morethanallowancelessthanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_morethanbalancelessthanallowance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_nooverflow_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transferFrom_FromEqTo-NoOverflow.scen.json",
        world(),
    );
}

#[test]
fn transfer_caller_allowanceirrelevant_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_Caller-AllowanceIrrelevant.scen.json",
        world(),
    );
}

#[test]
fn transfer_caller_entirebalance_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Caller-EntireBalance.scen.json", world());
}

#[test]
fn transfer_caller_morethanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_Caller-MoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transfer_caller_nooverflow_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Caller-NoOverflow.scen.json", world());
}

#[test]
fn transfer_caller_positive_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Caller-Positive.scen.json", world());
}

#[test]
fn transfer_caller_stillnooverflow_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_Caller-StillNoOverflow.scen.json",
        world(),
    );
}

#[test]
fn transfer_caller_zero_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Caller-Zero.scen.json", world());
}

#[test]
fn transfer_other_allowanceirrelevant_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_Other-AllowanceIrrelevant.scen.json",
        world(),
    );
}

#[test]
fn transfer_other_entirebalance_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Other-EntireBalance.scen.json", world());
}

#[test]
fn transfer_other_morethanbalance_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_Other-MoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transfer_other_nooverflow_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Other-NoOverflow.scen.json", world());
}

#[test]
fn transfer_other_positive_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Other-Positive.scen.json", world());
}

#[test]
fn transfer_other_stillnooverflow_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_Other-StillNoOverflow.scen.json",
        world(),
    );
}

#[test]
fn transfer_other_zero_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_Other-Zero.scen.json", world());
}
