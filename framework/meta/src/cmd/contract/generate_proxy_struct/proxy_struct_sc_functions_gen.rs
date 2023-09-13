use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi};

use crate::cmd::contract::generate_snippets::snippet_gen_common::write_newline;

pub(crate) fn write_content(
    file: &mut File,
    abi: &ContractAbi,
)
{
    write_name(file, abi.name);

    for constructor_abi in &abi.constructors {
        write_endpoint(file, constructor_abi);
        write_newline(file);
    }

    for endpoint_abi in &abi.endpoints {
        write_endpoint(file, endpoint_abi);
        write_endpoint_content_macro(file, endpoint_abi.name);
        write_contract_call(file, &endpoint_abi.inputs);
        writeln!(file, "\t\t\t\t___contract_call___").unwrap();
        writeln!(file, "\t\t\t}}").unwrap();
        write_newline(file);
    }
    writeln!(file, "\t\t)").unwrap();
    writeln!(file, "\t}}").unwrap();


    // close impl block brackets
    writeln!(file, "}}").unwrap();
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
    writeln!(file, "\t\t\t\tmultiversx_sc::types::ContractCall::proxy_arg(&mut ___contract_call___, &{arg_name});").unwrap();
}

fn write_endpoint_content_macro(file: &mut File, name: &str) {
    writeln!(file, "\t\t\t\tmultiversx_sc::endpoint_proxy!{{\n\t\t\t\t\t{name}\n\t\t\t\t}}").unwrap();
}

fn write_endpoint(file: &mut File, endpoint_abi: &EndpointAbi) {
    write_info_endpoint(file, endpoint_abi.docs);
    write_endpoint_fn(file, endpoint_abi.name);
    write_generic_args(file, &endpoint_abi.inputs);
    write_parameters(file, &endpoint_abi.inputs);
    // write_endpoint_args_declaration(file, &endpoint_abi.inputs);
}

fn write_parameters(file: &mut File, inputs: &[InputAbi]) {
    writeln!(file, "(").unwrap();
    writeln!(file, "\t\t\t\t&mut self,").unwrap();
    for (index, input) in inputs.iter().enumerate() {
        write_parameter_arg(file, index, &input.arg_name);
    }
    writeln!(file, "\t\t\t) -> multiversx_sc::types::ContractDeploy<Self::Api, ()> {{").unwrap();
}

fn write_parameter_arg(file: &mut File, index: usize, arg_name: &str) {
    writeln!(file, "\t\t\t\t{arg_name}: Arg{index},").unwrap()
}

fn write_endpoint_fn(file: &mut File, endpoint_name: &str) {
    write!(file, "\t\t\tfn {endpoint_name}").unwrap();
}

fn write_info_endpoint(file: &mut File, abi_docs: &[&str]) {
    if !abi_docs.is_empty() {
        write!(file, "\t\t\t//").unwrap();
    }

    for &abi_doc in abi_docs {
        writeln!(file, "{abi_doc} ").unwrap();
    }
}

fn write_name(file: &mut File, name: &str) {
    writeln!(file, "        {name}").unwrap();
    writeln!(file, "        (").unwrap();
}

fn write_generic_args(file: &mut File, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        return;
    }

    writeln!(file, "<").unwrap();


    for (index, input) in inputs.iter().enumerate() {
        write_argument(file, index, &input.type_name.to_string());
    }

    write!(file, "\t\t\t>").unwrap();
}

fn write_argument(file: &mut File, index: usize, type_name: &str) {
    writeln!(file, "\t\t\t\tArg{index}: multiversx_sc::codec::CodecInto<multiversx_sc::types::{type_name}<Self::Api>>,").unwrap();
}
