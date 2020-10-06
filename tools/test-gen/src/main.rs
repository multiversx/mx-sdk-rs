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
        "balanceOf",
        "create",
        "exceptions",
        "joinGame",
        "rewardAndSendToWallet",
        "rewardWinner_Last",
        "rewardWinner",
        "topUp_ok",
        "topUp_outOfFunds",
        "topUp_withdraw",
        "withdraw_Ok",
        "withdraw_TooMuch",
    ]);
}
