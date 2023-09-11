use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi};

use crate::cmd::contract::generate_proxy_trait::proxy_trait_sc_functions_gen::write_endpoint_args_declaration;
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
        write_newline(file);
    }
    writeln!(file, "\t\t)").unwrap();
    writeln!(file, "\t}}").unwrap();


    // close impl block brackets
    writeln!(file, "}}").unwrap();
}

fn write_endpoint(file: &mut File, endpoint_abi: &EndpointAbi) {
    write_info_endpoint(file, endpoint_abi.docs);
    write_endpoint_name(file, endpoint_abi.rust_method_name, endpoint_abi.name);
    write_generic_args(file, &endpoint_abi.inputs);
    write_endpoint_args_declaration(file, &endpoint_abi.inputs);
}

fn write_endpoint_name(file: &mut File, rust_method_name: &str, endpoint_name: &str) {
    write!(file, "\t\t\t{endpoint_name} => {rust_method_name}").unwrap();
    //(").unwrap();
}

fn write_info_endpoint(file: &mut File, abi_docs: &[&str]) {
    if abi_docs.len() > 0 {
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
        write!(file, "(").unwrap();
        return;
    }

    for n in 0..inputs.len() {
        write!(file, "<Arg{n}>").unwrap();
    }
    write!(file, "(").unwrap();
}
