use mx_sc_debug::*;

fn world() -> BlockchainMock {
    BlockchainMock::new()
}

#[test]
fn local_path_test() {
    mx_sc_debug::mandos_rs("tests/mandos-self/path_test.scen.json", world());
}

#[test]
fn nested_path_test() {
    mx_sc_debug::mandos_rs(
        "tests/mandos-self/external_steps/external_path_test.scen.json",
        world(),
    );
}
