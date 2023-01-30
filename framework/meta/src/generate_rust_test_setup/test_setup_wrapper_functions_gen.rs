use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi};

use super::{
    test_gen_common::is_last_element, test_setup_type_map::map_abi_type_to_unmanaged_rust_type,
};

pub(crate) fn write_struct_constructor(
    file: &mut File,
    crate_name: &str,
    builder_fn_name: &str,
    init_abi: &EndpointAbi,
) {
    let init_fn_name = init_abi.rust_method_name;

    writeln!(
        file,
        "    pub fn new(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        builder: {builder_fn_name},
        {}
    ) -> Self {{
        let owner = b_mock.borrow_mut().create_user_account(&rust_biguint!(0));
        let sc_wrapper = b_mock
            .borrow_mut()
            .create_sc_account(&rust_biguint!(0), Some(&owner), builder, \"{crate_name}.wasm\");
            
        b_mock
            .borrow_mut()
            .execute_tx(&owner, &sc_wrapper, &rust_biguint!(0), |sc| {{
                sc.{init_fn_name}({});
            }})
            .assert_ok();
            
        Self {{
            b_mock,
            owner,
            sc_wrapper
        }}
    }}",
        get_wrapper_func_declaration_args(&init_abi.inputs),
        get_wrapper_func_internal_call_args(&init_abi.inputs)
    )
    .unwrap();
}

fn get_wrapper_func_declaration_args(inputs: &[InputAbi]) -> String {
    let mut result = String::new();
    for (i, input) in inputs.iter().enumerate() {
        let arg_name = input.arg_name;
        let rust_type = map_abi_type_to_unmanaged_rust_type(input.type_name.to_string());
        let rust_type_name = rust_type.get_type_name();
        result += &format!("{arg_name}: {rust_type_name}");

        if !is_last_element(inputs, i) {
            result += ", ";
        }
    }

    result
}

fn get_wrapper_func_internal_call_args(inputs: &[InputAbi]) -> String {
    let mut result = String::new();
    for (i, input) in inputs.iter().enumerate() {
        result += input.arg_name;

        if !is_last_element(inputs, i) {
            result += ", ";
        }
    }

    result
}
