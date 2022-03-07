use multisig_rust_test_setup::MultisigSetup;

mod multisig_rust_test_setup;

#[test]
fn init_test() {
    let _ = MultisigSetup::new(multisig::contract_obj);
}
