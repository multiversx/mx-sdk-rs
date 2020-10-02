// Just a quick and dirty way to auto-generate some mandos tests on the Rust side.
// Will get converted to a tool at some point.

fn print_mandos_tests(names: &[&str]) {
    for name in names.iter() {
        print!("
#[test]
fn {}() {{
    parse_execute_mandos(\"mandos/{}.scen.json\", &contract_map());
}}
",
        name.replace('-', "_").to_lowercase(), 
        name);
    }
}

fn main() {
    print_mandos_tests(&[
        "allowance_CallerCaller",
        "allowance_CallerOther",
        "allowance_OtherCaller",
        "allowance_OtherEqOther",
        "allowance_OtherNEqOther",
        "approve_Caller-Positive",
        "approve_Caller-Zero",
        "approve_Other-Positive",
        "approve_Other-Zero",
        "approve_SwitchCaller",
        "balanceOf_Caller",
        "balanceOf_NonCaller",
        "not_payable",
        "totalSupply_Positive",
        "totalSupply_Zero",
        "transfer_Caller-AllowanceIrrelevant",
        "transfer_Caller-EntireBalance",
        "transfer_Caller-MoreThanBalance",
        "transfer_Caller-NoOverflow",
        "transfer_Caller-Positive",
        "transfer_Caller-StillNoOverflow",
        "transfer_Caller-Zero",
        "transferFrom_AllDistinct-BalanceEqAllowance",
        "transferFrom_AllDistinct-BalanceNEqAllowance",
        "transferFrom_AllDistinct-EntireAllowanceMoreThanBalance",
        "transferFrom_AllDistinct-EntireBalanceEqAllowance",
        "transferFrom_AllDistinct-EntireBalanceMoreThanAllowance",
        "transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance",
        "transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance",
        "transferFrom_AllDistinct-NoOverflow",
        "transferFrom_AllDistinct-StillNoOverflow",
        "transferFrom_AllEqual-AllowanceRelevant",
        "transferFrom_AllEqual-EntireBalance",
        "transferFrom_CallerEqFrom-AllowanceRelevant",
        "transferFrom_CallerEqFrom-EntireBalance",
        "transferFrom_CallerEqFrom-MoreThanBalance",
        "transferFrom_CallerEqTo-BalanceNEqAllowance",
        "transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance",
        "transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance",
        "transferFrom_Exploratory-MultipleTransfersSucceed",
        "transferFrom_Exploratory-MultipleTransfersThrow",
        "transferFrom_FromEqTo-BalanceEqAllowance",
        "transferFrom_FromEqTo-BalanceNEqAllowance",
        "transferFrom_FromEqTo-EntireAllowanceMoreThanBalance",
        "transferFrom_FromEqTo-EntireBalanceEqAllowance",
        "transferFrom_FromEqTo-EntireBalanceMoreThanAllowance",
        "transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance",
        "transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance",
        "transferFrom_FromEqTo-NoOverflow",
        "transfer_Other-AllowanceIrrelevant",
        "transfer_Other-EntireBalance",
        "transfer_Other-MoreThanBalance",
        "transfer_Other-NoOverflow",
        "transfer_Other-Positive",
        "transfer_Other-StillNoOverflow",
        "transfer_Other-Zero"]);
}
