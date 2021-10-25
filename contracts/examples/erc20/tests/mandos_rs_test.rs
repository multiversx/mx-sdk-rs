use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/erc20.wasm",
        Box::new(|context| Box::new(erc20::contract_obj(context))),
    );
    blockchain
}

#[test]
fn allowance_callercaller_rs() {
    elrond_wasm_debug::mandos_rs("mandos/allowance_CallerCaller.scen.json", contract_map());
}

#[test]
fn allowance_callerother_rs() {
    elrond_wasm_debug::mandos_rs("mandos/allowance_CallerOther.scen.json", contract_map());
}

#[test]
fn allowance_othercaller_rs() {
    elrond_wasm_debug::mandos_rs("mandos/allowance_OtherCaller.scen.json", contract_map());
}

#[test]
fn allowance_othereqother_rs() {
    elrond_wasm_debug::mandos_rs("mandos/allowance_OtherEqOther.scen.json", contract_map());
}

#[test]
fn allowance_otherneqother_rs() {
    elrond_wasm_debug::mandos_rs("mandos/allowance_OtherNEqOther.scen.json", contract_map());
}

#[test]
fn approve_caller_positive_rs() {
    elrond_wasm_debug::mandos_rs("mandos/approve_Caller-Positive.scen.json", contract_map());
}

#[test]
fn approve_caller_zero_rs() {
    elrond_wasm_debug::mandos_rs("mandos/approve_Caller-Zero.scen.json", contract_map());
}

#[test]
fn approve_other_positive_rs() {
    elrond_wasm_debug::mandos_rs("mandos/approve_Other-Positive.scen.json", contract_map());
}

#[test]
fn approve_other_zero_rs() {
    elrond_wasm_debug::mandos_rs("mandos/approve_Other-Zero.scen.json", contract_map());
}

#[test]
fn approve_switchcaller_rs() {
    elrond_wasm_debug::mandos_rs("mandos/approve_SwitchCaller.scen.json", contract_map());
}

#[test]
fn balanceof_caller_rs() {
    elrond_wasm_debug::mandos_rs("mandos/balanceOf_Caller.scen.json", contract_map());
}

#[test]
fn balanceof_noncaller_rs() {
    elrond_wasm_debug::mandos_rs("mandos/balanceOf_NonCaller.scen.json", contract_map());
}

#[test]
fn not_payable_rs() {
    elrond_wasm_debug::mandos_rs("mandos/not_payable.scen.json", contract_map());
}

#[test]
fn totalsupply_positive_rs() {
    elrond_wasm_debug::mandos_rs("mandos/totalSupply_Positive.scen.json", contract_map());
}

#[test]
fn totalsupply_zero_rs() {
    elrond_wasm_debug::mandos_rs("mandos/totalSupply_Zero.scen.json", contract_map());
}

#[test]
fn transferfrom_alldistinct_balanceeqallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-BalanceEqAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_balanceneqallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_entireallowancemorethanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_entirebalanceeqallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_entirebalancemorethanallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_morethanallowancelessthanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_morethanbalancelessthanallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_nooverflow_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-NoOverflow.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_alldistinct_stillnooverflow_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllDistinct-StillNoOverflow.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_allequal_allowancerelevant_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllEqual-AllowanceRelevant.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_allequal_entirebalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_AllEqual-EntireBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_callereqfrom_allowancerelevant_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_callereqfrom_entirebalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_CallerEqFrom-EntireBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_callereqfrom_morethanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_CallerEqFrom-MoreThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_callereqto_balanceneqallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_callereqto_morethanallowancelessthanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_callereqto_morethanbalancelessthanallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_exploratory_multipletransferssucceed_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_exploratory_multipletransfersthrow_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_Exploratory-MultipleTransfersThrow.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_balanceeqallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-BalanceEqAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_balanceneqallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_entireallowancemorethanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_entirebalanceeqallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_entirebalancemorethanallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_morethanallowancelessthanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_morethanbalancelessthanallowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json",
        contract_map(),
    );
}

#[test]
fn transferfrom_fromeqto_nooverflow_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transferFrom_FromEqTo-NoOverflow.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_caller_allowanceirrelevant_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Caller-AllowanceIrrelevant.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_caller_entirebalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Caller-EntireBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_caller_morethanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Caller-MoreThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_caller_nooverflow_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Caller-NoOverflow.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_caller_positive_rs() {
    elrond_wasm_debug::mandos_rs("mandos/transfer_Caller-Positive.scen.json", contract_map());
}

#[test]
fn transfer_caller_stillnooverflow_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Caller-StillNoOverflow.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_caller_zero_rs() {
    elrond_wasm_debug::mandos_rs("mandos/transfer_Caller-Zero.scen.json", contract_map());
}

#[test]
fn transfer_other_allowanceirrelevant_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Other-AllowanceIrrelevant.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_other_entirebalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Other-EntireBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_other_morethanbalance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Other-MoreThanBalance.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_other_nooverflow_rs() {
    elrond_wasm_debug::mandos_rs("mandos/transfer_Other-NoOverflow.scen.json", contract_map());
}

#[test]
fn transfer_other_positive_rs() {
    elrond_wasm_debug::mandos_rs("mandos/transfer_Other-Positive.scen.json", contract_map());
}

#[test]
fn transfer_other_stillnooverflow_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/transfer_Other-StillNoOverflow.scen.json",
        contract_map(),
    );
}

#[test]
fn transfer_other_zero_rs() {
    elrond_wasm_debug::mandos_rs("mandos/transfer_Other-Zero.scen.json", contract_map());
}
