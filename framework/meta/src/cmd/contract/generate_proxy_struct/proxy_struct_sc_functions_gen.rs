use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbis};

use crate::cmd::contract::generate_snippets::{
    snippet_gen_common::write_newline,
    snippet_sc_functions_gen::map_output_types_to_rust_types,
    snippet_type_map::{handle_abi_type, RustTypeString},
};

pub(crate) fn write_content(file: &mut File, abi: &ContractAbi) {
    for constructor_abi in &abi.constructors {
        write_endpoint(file, constructor_abi, "ContractDeploy");
        write_constructor_content_macro(file);
        write_constructor_contract_deploy(file, &constructor_abi.inputs);
        writeln!(file, "\t\t___contract_deploy___").unwrap();
        writeln!(file, "\t}}").unwrap();
        write_newline(file);
    }

    for endpoint_abi in &abi.endpoints {
        write_endpoint(file, endpoint_abi, "ContractCallNoPayment");
        write_endpoint_content_macro(file, endpoint_abi.name);
        write_contract_call(file, &endpoint_abi.inputs);
        writeln!(file, "\t\t___contract_call___").unwrap();
        writeln!(file, "\t}}").unwrap();
        write_newline(file);
    }

    writeln!(file, "}}").unwrap();
}

fn write_constructor_contract_deploy(file: &mut File, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        return;
    }

    for input in inputs.iter() {
        write_constructor_contract_call(file, &input.arg_name);
    }
}

fn write_contract_call(file: &mut File, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        return;
    }

    for input in inputs.iter() {
        write_contract_call_input(file, &input.arg_name);
    }
}

fn write_contract_call_input(file: &mut File, arg_name: &&str) {
    writeln!(
        file,
        "\t\tContractCall::proxy_arg(&mut ___contract_call___, &{arg_name});"
    )
    .unwrap();
}

fn write_constructor_contract_call(file: &mut File, arg_name: &&str) {
    writeln!(
        file,
        "\t\t___contract_deploy___.push_endpoint_arg(&{arg_name});"
    )
    .unwrap();
}

fn write_endpoint_content_macro(file: &mut File, name: &str) {
    writeln!(
        file,
        "\t\tlet ___address___ = multiversx_sc::extract_address!(self);"
    )
    .unwrap();
    writeln!(
        file,
        "\t\tlet mut ___contract_call___ = multiversx_sc::endpoints_proxy!({name}, ___address___);"
    )
    .unwrap();
}

fn write_constructor_content_macro(file: &mut File) {
    writeln!(
        file,
        "\t\tlet ___opt_address___ = multiversx_sc::extract_opt_address!(self);"
    )
    .unwrap();
    writeln!(file, "\t\tlet mut ___contract_deploy___ = multiversx_sc::constructors_proxy!(___opt_address___);").unwrap();
}

fn write_endpoint(file: &mut File, endpoint_abi: &EndpointAbi, interaction_deploy: &str) {
    write_info_endpoint(file, endpoint_abi.docs);
    write_endpoint_fn(file, endpoint_abi.rust_method_name);
    write_generic_args(file, &endpoint_abi.inputs);
    write_parameters(file, &endpoint_abi.inputs, interaction_deploy);
    write_output(file, &endpoint_abi.outputs);
}

fn write_output(file: &mut File, outputs: &OutputAbis) {
    let output_type = map_output_types_to_rust_types(outputs);

    let output_type_print = output_type.replace("<StaticApi>", "<A>");
    write!(file, "{output_type_print}",).unwrap();
    writeln!(file, "> {{").unwrap();
}

fn write_parameters(file: &mut File, inputs: &[InputAbi], interaction_deploy: &str) {
    writeln!(file, "(").unwrap();
    writeln!(file, "\t\t&mut self,").unwrap();

    for (index, input) in inputs.iter().enumerate() {
        write_parameter_arg(file, index, &input.arg_name);
    }

    write!(file, "\t) -> {interaction_deploy}<A, ").unwrap();
}

fn write_parameter_arg(file: &mut File, index: usize, arg_name: &str) {
    writeln!(file, "\t\t{arg_name}: Arg{index},").unwrap()
}

fn write_endpoint_fn(file: &mut File, rust_method_name: &str) {
    write!(file, "\tfn {rust_method_name}").unwrap();
}

fn write_info_endpoint(file: &mut File, abi_docs: &[&str]) {
    if !abi_docs.is_empty() {
        write!(file, "\t//").unwrap();
    }

    for &abi_doc in abi_docs {
        writeln!(file, "{abi_doc} ").unwrap();
    }
}

fn write_generic_args(file: &mut File, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        return;
    }

    writeln!(file, "<").unwrap();

    for (index, input) in inputs.iter().enumerate() {
        write_argument(file, index, input.type_name.to_string());
    }

    write!(file, "\t>").unwrap();
}

fn write_argument(file: &mut File, index: usize, type_name: String) {
    let mut type_string = RustTypeString::default();
    handle_abi_type(&mut type_string, type_name);
    let type_string_str = type_string.get_type_name().to_string();

    let type_print = type_string_str.replace("<StaticApi>", "<A>");

    writeln!(
        file,
        "\t\tArg{index}: multiversx_sc::codec::CodecInto<{}>,",
        type_print
    )
    .unwrap();
}
