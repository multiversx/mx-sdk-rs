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
        "message_otherShard_callback",
        "message_otherShard",
        "message_sameShard_callback",
        "message_sameShard",
        "payment_otherShard_callback",
        "payment_otherShard",
        "payment_sameShard_callback",
        "payment_sameShard",
    ]);
}
