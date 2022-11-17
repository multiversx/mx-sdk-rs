#[test]
fn allowance_callercaller_go() {
    elrond_wasm_debug::mandos_go("mandos/allowance_CallerCaller.scen.json");
}

#[test]
fn allowance_callerother_go() {
    elrond_wasm_debug::mandos_go("mandos/allowance_CallerOther.scen.json");
}

#[test]
fn allowance_othercaller_go() {
    elrond_wasm_debug::mandos_go("mandos/allowance_OtherCaller.scen.json");
}

#[test]
fn allowance_othereqother_go() {
    elrond_wasm_debug::mandos_go("mandos/allowance_OtherEqOther.scen.json");
}

#[test]
fn allowance_otherneqother_go() {
    elrond_wasm_debug::mandos_go("mandos/allowance_OtherNEqOther.scen.json");
}

#[test]
fn approve_caller_positive_go() {
    elrond_wasm_debug::mandos_go("mandos/approve_Caller-Positive.scen.json");
}

#[test]
fn approve_caller_zero_go() {
    elrond_wasm_debug::mandos_go("mandos/approve_Caller-Zero.scen.json");
}

#[test]
fn approve_other_positive_go() {
    elrond_wasm_debug::mandos_go("mandos/approve_Other-Positive.scen.json");
}

#[test]
fn approve_other_zero_go() {
    elrond_wasm_debug::mandos_go("mandos/approve_Other-Zero.scen.json");
}

#[test]
fn approve_switchcaller_go() {
    elrond_wasm_debug::mandos_go("mandos/approve_SwitchCaller.scen.json");
}

#[test]
fn balanceof_caller_go() {
    elrond_wasm_debug::mandos_go("mandos/balanceOf_Caller.scen.json");
}

#[test]
fn balanceof_noncaller_go() {
    elrond_wasm_debug::mandos_go("mandos/balanceOf_NonCaller.scen.json");
}

#[test]
fn not_payable_go() {
    elrond_wasm_debug::mandos_go("mandos/not_payable.scen.json");
}

#[test]
fn not_payable_esdt_go() {
    elrond_wasm_debug::mandos_go("mandos/not_payable_esdt.scen.json");
}

#[test]
fn totalsupply_positive_go() {
    elrond_wasm_debug::mandos_go("mandos/totalSupply_Positive.scen.json");
}

#[test]
fn totalsupply_zero_go() {
    elrond_wasm_debug::mandos_go("mandos/totalSupply_Zero.scen.json");
}

#[test]
fn transferfrom_alldistinct_balanceeqallowance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_AllDistinct-BalanceEqAllowance.scen.json");
}

#[test]
fn transferfrom_alldistinct_balanceneqallowance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json");
}

#[test]
fn transferfrom_alldistinct_entireallowancemorethanbalance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json",
    );
}

#[test]
fn transferfrom_alldistinct_entirebalanceeqallowance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json",
    );
}

#[test]
fn transferfrom_alldistinct_entirebalancemorethanallowance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json",
    );
}

#[test]
fn transferfrom_alldistinct_morethanallowancelessthanbalance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json",
    );
}

#[test]
fn transferfrom_alldistinct_morethanbalancelessthanallowance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json",
    );
}

#[test]
fn transferfrom_alldistinct_nooverflow_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_AllDistinct-NoOverflow.scen.json");
}

#[test]
fn transferfrom_alldistinct_stillnooverflow_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_AllDistinct-StillNoOverflow.scen.json");
}

#[test]
fn transferfrom_allequal_allowancerelevant_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_AllEqual-AllowanceRelevant.scen.json");
}

#[test]
fn transferfrom_allequal_entirebalance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_AllEqual-EntireBalance.scen.json");
}

#[test]
fn transferfrom_callereqfrom_allowancerelevant_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json");
}

#[test]
fn transferfrom_callereqfrom_entirebalance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_CallerEqFrom-EntireBalance.scen.json");
}

#[test]
fn transferfrom_callereqfrom_morethanbalance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_CallerEqFrom-MoreThanBalance.scen.json");
}

#[test]
fn transferfrom_callereqto_balanceneqallowance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json");
}

#[test]
fn transferfrom_callereqto_morethanallowancelessthanbalance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json",
    );
}

#[test]
fn transferfrom_callereqto_morethanbalancelessthanallowance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json",
    );
}

#[test]
fn transferfrom_exploratory_multipletransferssucceed_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json",
    );
}

#[test]
fn transferfrom_exploratory_multipletransfersthrow_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_Exploratory-MultipleTransfersThrow.scen.json",
    );
}

#[test]
fn transferfrom_fromeqto_balanceeqallowance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_FromEqTo-BalanceEqAllowance.scen.json");
}

#[test]
fn transferfrom_fromeqto_balanceneqallowance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json");
}

#[test]
fn transferfrom_fromeqto_entireallowancemorethanbalance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json",
    );
}

#[test]
fn transferfrom_fromeqto_entirebalanceeqallowance_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json");
}

#[test]
fn transferfrom_fromeqto_entirebalancemorethanallowance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json",
    );
}

#[test]
fn transferfrom_fromeqto_morethanallowancelessthanbalance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json",
    );
}

#[test]
fn transferfrom_fromeqto_morethanbalancelessthanallowance_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json",
    );
}

#[test]
fn transferfrom_fromeqto_nooverflow_go() {
    elrond_wasm_debug::mandos_go("mandos/transferFrom_FromEqTo-NoOverflow.scen.json");
}

#[test]
fn transfer_caller_allowanceirrelevant_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Caller-AllowanceIrrelevant.scen.json");
}

#[test]
fn transfer_caller_entirebalance_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Caller-EntireBalance.scen.json");
}

#[test]
fn transfer_caller_morethanbalance_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Caller-MoreThanBalance.scen.json");
}

#[test]
fn transfer_caller_nooverflow_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Caller-NoOverflow.scen.json");
}

#[test]
fn transfer_caller_positive_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Caller-Positive.scen.json");
}

#[test]
fn transfer_caller_stillnooverflow_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Caller-StillNoOverflow.scen.json");
}

#[test]
fn transfer_caller_zero_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Caller-Zero.scen.json");
}

#[test]
fn transfer_other_allowanceirrelevant_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Other-AllowanceIrrelevant.scen.json");
}

#[test]
fn transfer_other_entirebalance_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Other-EntireBalance.scen.json");
}

#[test]
fn transfer_other_morethanbalance_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Other-MoreThanBalance.scen.json");
}

#[test]
fn transfer_other_nooverflow_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Other-NoOverflow.scen.json");
}

#[test]
fn transfer_other_positive_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Other-Positive.scen.json");
}

#[test]
fn transfer_other_stillnooverflow_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Other-StillNoOverflow.scen.json");
}

#[test]
fn transfer_other_zero_go() {
    elrond_wasm_debug::mandos_go("mandos/transfer_Other-Zero.scen.json");
}
