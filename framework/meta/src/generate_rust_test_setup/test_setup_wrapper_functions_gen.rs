use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi};

use crate::generate_snippets::{
    snippet_gen_common::write_newline,
    snippet_sc_functions_gen::{get_payable_type, map_output_types_to_rust_types, PayableType},
    snippet_type_map::map_abi_type_to_rust_type,
};

use super::{
    test_gen_common::is_last_element, test_setup_type_map::ABI_TYPES_TO_RUST_TEST_TYPES_MAP,
};

static EGLD_VALUE_ARG_NAME: &str = "egld_value";
static ESDT_TRANSFERS_ARG_NAME: &str = "esdt_transfers";

static OWNER_FIELD_NAME: &str = "owner";
static SC_WRAPPER_FIELD_NAME: &str = "sc_wrapper";

#[derive(PartialEq, Clone, Copy, Debug)]
enum FunctionType {
    View,
    Endpoint(PayableType),
}

impl FunctionType {
    fn get_payable_type(&self) -> PayableType {
        match *self {
            FunctionType::View => PayableType::NotPayable,
            FunctionType::Endpoint(payable_type) => payable_type,
        }
    }
}

fn get_function_type(endpoint_abi: &EndpointAbi) -> FunctionType {
    if endpoint_abi.mutability.is_view() {
        return FunctionType::View;
    }

    let payable_type = get_payable_type(endpoint_abi.payable_in_tokens);
    FunctionType::Endpoint(payable_type)
}

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
        let {OWNER_FIELD_NAME} = caller.clone();
        let {SC_WRAPPER_FIELD_NAME} = b_mock
            .borrow_mut()
            .create_sc_account(&rust_biguint!(0), Some(&{OWNER_FIELD_NAME}), builder, \"{crate_name}.wasm\");
            
        b_mock
            .borrow_mut()
            .execute_tx(&{OWNER_FIELD_NAME}, &{SC_WRAPPER_FIELD_NAME}, &rust_biguint!(0), |sc| {{
                let _ = sc.{init_fn_name}({});
            }})
            .assert_ok();
            
        Self {{
            b_mock,
            {OWNER_FIELD_NAME},
            {SC_WRAPPER_FIELD_NAME}
        }}
    }}
",
        get_wrapper_func_declaration_args(&init_abi, PayableType::NotPayable),
        get_lambda_endpoint_args_snippet(&init_abi.inputs)
    )
    .unwrap();
}

pub(crate) fn write_endpoint_wrapper_functions(file: &mut File, abi: &ContractAbi) {
    for endpoint_abi in &abi.endpoints {
        write_endpoint_wrapper(file, endpoint_abi);
        write_newline(file);
    }
}

fn write_endpoint_wrapper(file: &mut File, endpoint_abi: &EndpointAbi) {
    let fn_name = endpoint_abi.rust_method_name;
    let fn_type = get_function_type(&endpoint_abi);
    let result_type =
        map_output_types_to_rust_types(&endpoint_abi.outputs, &ABI_TYPES_TO_RUST_TEST_TYPES_MAP);

    writeln!(
        file,
        "    pub fn {fn_name}(&self, {}) -> WrappedTxResult<{result_type}> {{
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .{}({}, |sc| {{
                let res = sc.{fn_name}({});
                opt_endpoint_result = Some(res);
            }});

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }}",
        get_wrapper_func_declaration_args(&endpoint_abi, fn_type.get_payable_type()),
        get_executor_function_to_call(fn_type),
        get_wrapper_func_internal_call_args(endpoint_abi, fn_type),
        get_lambda_endpoint_args_snippet(&endpoint_abi.inputs)
    )
    .unwrap();
}

fn get_wrapper_func_declaration_args(
    endpoint_abi: &EndpointAbi,
    payable_type: PayableType,
) -> String {
    let mut result = get_caller_arg_snippet(endpoint_abi);
    if !result.is_empty() {
        result += ", ";
    }

    let payment_snippet = get_required_wrapper_func_payment_args(payable_type);
    result += &payment_snippet;

    let inputs = &endpoint_abi.inputs;
    if inputs.is_empty() {
        return result;
    }

    if !payment_snippet.is_empty() {
        result += ", ";
    }

    for (i, input) in inputs.iter().enumerate() {
        let arg_name = input.arg_name;
        let rust_type = map_abi_type_to_rust_type(
            input.type_name.to_string(),
            &ABI_TYPES_TO_RUST_TEST_TYPES_MAP,
        );
        let rust_type_name = rust_type.get_type_name();
        result += &format!("{arg_name}: {rust_type_name}");

        if !is_last_element(inputs, i) {
            result += ", ";
        }
    }

    result
}

fn get_wrapper_func_internal_call_args(
    endpoint_abi: &EndpointAbi,
    fn_type: FunctionType,
) -> String {
    let mut result = get_caller_arg_for_wrapper_fn(endpoint_abi);
    if !result.is_empty() {
        result += ", ";
    }

    result += &format!("&self.{SC_WRAPPER_FIELD_NAME}");

    let payment_snippet = get_executor_payment_arg_snippet(fn_type);
    if !payment_snippet.is_empty() {
        result += ", ";
        result += &payment_snippet;
    }

    result
}

fn get_lambda_endpoint_args_snippet(inputs: &[InputAbi]) -> String {
    let mut result = String::new();
    for (i, input) in inputs.iter().enumerate() {
        let arg_name = input.arg_name;
        result += &format!("{arg_name}.into()");

        if !is_last_element(inputs, i) {
            result += ", ";
        }
    }

    result
}

fn get_executor_function_to_call(fn_type: FunctionType) -> String {
    match fn_type {
        FunctionType::View => "execute_query".to_string(),
        FunctionType::Endpoint(payable_type) => match payable_type {
            PayableType::NotPayable | PayableType::Egld => "execute_tx".to_string(),
            PayableType::Any => "execute_esdt_multi_transfer".to_string(),
        },
    }
}

fn get_caller_arg_snippet(endpoint_abi: &EndpointAbi) -> String {
    if endpoint_abi.only_owner || endpoint_abi.mutability.is_view() {
        String::new()
    } else {
        "caller: &Address".to_string()
    }
}

fn get_caller_arg_for_wrapper_fn(endpoint_abi: &EndpointAbi) -> String {
    if endpoint_abi.only_owner {
        return format!("&self.{OWNER_FIELD_NAME}");
    }

    if endpoint_abi.mutability.is_view() {
        String::new()
    } else {
        "caller".to_string() // arg is already a reference
    }
}

fn get_required_wrapper_func_payment_args(payable_type: PayableType) -> String {
    match payable_type {
        PayableType::NotPayable => String::new(),
        PayableType::Egld => format!("{EGLD_VALUE_ARG_NAME}: &RustBigUint"),
        PayableType::Any => format!("{ESDT_TRANSFERS_ARG_NAME}: &[TxTokenTransfer]"),
    }
}

fn get_executor_payment_arg_snippet(fn_type: FunctionType) -> String {
    match fn_type {
        FunctionType::View => String::new(),
        FunctionType::Endpoint(payable_type) => match payable_type {
            PayableType::NotPayable => "&rust_biguint!(0)".to_string(),
            PayableType::Egld => format!("{EGLD_VALUE_ARG_NAME}"),
            PayableType::Any => format!("{ESDT_TRANSFERS_ARG_NAME}"),
        },
    }
}
