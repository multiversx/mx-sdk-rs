use std::{cell::RefCell, rc::Rc};

use multiversx_sc_scenario::{rust_biguint, testing_framework::BlockchainStateWrapper, DebugApi};

mod rust_snippets_generator_test_setup;

use rust_snippets_generator_test_setup::*;

#[test]
fn test_mod_compile() {
    let _ = DebugApi::dummy();
    let b_mock = Rc::new(RefCell::new(BlockchainStateWrapper::new()));
    let owner = b_mock.borrow_mut().create_user_account(&rust_biguint!(0));
    let _ = RustSnippetsGeneratorTestSetup::new(
        b_mock,
        rust_snippets_generator_test::contract_obj,
        &owner,
    );
}
