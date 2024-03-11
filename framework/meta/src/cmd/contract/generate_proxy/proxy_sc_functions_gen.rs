use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi};

use crate::cmd::contract::generate_snippets::{
    snippet_gen_common::write_newline,
    snippet_type_map::{handle_abi_type, RustTypeString},
};

pub(crate) fn write_content(file: &mut File, abi: ContractAbi) {
    for constructor_abi in abi.constructors {
        write_endpoint(file, constructor_abi.clone());
        write_function_content(file, constructor_abi);
        write_end_of_function(file);
    }

    for endpoint_abi in abi.endpoints {
        write_endpoint(file, endpoint_abi.clone());
        write_function_content(file, endpoint_abi);
        write_end_of_function(file);
    }

    writeln!(file, "}}").unwrap();
}

fn write_function_content(file: &mut File, end: EndpointAbi) {
    writeln!(
        file,
        "\t\tTx::new_with_env(self.env.clone())
            .raw_call()
            .function_name(\"{}\")",
        end.name
    )
    .unwrap();

    for input in end.inputs.iter() {
        writeln!(
            file,
            "\t\t\t.argument(&{})",
            input.arg_name // .argument(&arg0)"
        )
        .unwrap();
    }
}

fn write_endpoint(file: &mut File, endpoint_abi: EndpointAbi) {
    write_info_endpoint(file, endpoint_abi.docs);
    write_function_header_endpoint(file, endpoint_abi.rust_method_name);
    write_args(file, endpoint_abi.inputs.clone());
    write_parameters_and_output(file, endpoint_abi.inputs);
}

#[rustfmt::skip]
fn write_parameters_and_output(file: &mut File, inputs: Vec<InputAbi>) {
    writeln!(file, "(").unwrap();
    writeln!(file, "\t\t&mut self,").unwrap();

    for (index, input) in inputs.iter().enumerate() {
        writeln!(file, "\t\t{}: Arg{index},", &input.arg_name).unwrap();
    }

    write!(
        file,
        "\t) -> multiversx_sc::types::Tx<Env,
        (),
        (),
        (),
        (),
        FunctionCall<<Env as multiversx_sc::types::TxEnv>::Api>,
        (),
    > {{\n"
    )
    .unwrap();
}

fn write_function_header_endpoint(file: &mut File, rust_method_name: String) {
    write!(file, "\tpub fn {rust_method_name}").unwrap();
}

fn write_info_endpoint(file: &mut File, docs: Vec<String>) {
    if !docs.is_empty() {
        write!(file, "\t//").unwrap();
    }

    for abi_doc in docs {
        writeln!(file, "{abi_doc} ").unwrap();
    }
}

fn write_args(file: &mut File, inputs: Vec<InputAbi>) {
    if inputs.is_empty() {
        return;
    }

    writeln!(file, "<").unwrap();

    for (index, input) in inputs.iter().enumerate() {
        write_argument(file, index, input.type_names.abi.to_string());
    }

    write!(file, "\t>").unwrap();
}

fn write_argument(file: &mut File, index: usize, type_name: String) {
    let mut type_string = RustTypeString::default();
    handle_abi_type(&mut type_string, type_name);
    let type_string_str = type_string.get_type_name().to_string();

    let type_print = type_string_str.replace("<StaticApi>", "<Env>");

    writeln!(
        file,
        "\t\tArg{index}: multiversx_sc::codec::CodecInto<{}>,",
        type_print
    )
    .unwrap();
}

fn write_end_of_function(file: &mut File) {
    writeln!(file, "\t}}").unwrap();
    write_newline(file);
}
