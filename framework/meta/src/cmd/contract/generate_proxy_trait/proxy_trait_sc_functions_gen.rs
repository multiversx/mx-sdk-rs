use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbi};

use crate::cmd::contract::generate_snippets::snippet_gen_common::write_newline;

pub(crate) fn write_state_struct_impl(
    file: &mut File,
    abi: &ContractAbi,
) {
    for constructor_abi in &abi.constructors {
        write_endpoint_impl(file, constructor_abi);
        write_newline(file);
    }

    for endpoint_abi in &abi.endpoints {
        write_endpoint_impl(file, endpoint_abi);
        write_newline(file);
    }

    // close impl block brackets
    writeln!(file, "}}").unwrap();
}

fn write_endpoint_impl(file: &mut File, endpoint_abi: &EndpointAbi) {
    write_method_attribute(file, endpoint_abi.name, endpoint_abi.rust_method_name);
    write_method_declaration(file, endpoint_abi.rust_method_name);
    write_comma(file, &endpoint_abi.inputs);
    write_endpoint_args_declaration(file, &endpoint_abi.inputs);
    write_endpoint_output(file, &endpoint_abi.outputs);
    write_newline(file);
}

fn write_comma(file: &mut File, inputs: &[InputAbi]) {
    if !inputs.is_empty() {
        write!(
            file,
            ", ",
        )
            .unwrap();
    }
}

fn write_method_attribute(file: &mut File, endpoint_name: &str, endpoint_method_name: &str) {
    if endpoint_name != endpoint_method_name {
        writeln!(file, "    #[view({endpoint_name})]").unwrap();
    } else {
        writeln!(file, "    #[endpoint]").unwrap();
    }
}

fn write_method_declaration(file: &mut File, endpoint_name: &str) {
    write!(file, "    fn {endpoint_name}(&mut self").unwrap();
}

fn write_endpoint_output(file: &mut File, outputs: &[OutputAbi]) {
    if outputs.is_empty() {
        write!(
            file,
            ";",
        )
            .unwrap();
        return;
    }

    for output in outputs {
        write!(
            file,
            " -> {}",
            output.type_name
        )
            .unwrap();
    }

    write!(
        file,
        ";",
    )
        .unwrap();
}

pub fn write_endpoint_args_declaration(file: &mut File, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        write!(
            file,
            ")",
        )
            .unwrap();
        return;
    }

    for (index, input) in inputs.iter().enumerate() {
        write!(
            file,
            "{}: {}",
            input.arg_name,
            input.type_name.to_string()
        )
            .unwrap();

        // Add a comma after each input except the last one
        if index < inputs.len() - 1 {
            write!(file, ", ").unwrap();
        }
    }

    write!(
        file,
        ")",
    )
        .unwrap();
}