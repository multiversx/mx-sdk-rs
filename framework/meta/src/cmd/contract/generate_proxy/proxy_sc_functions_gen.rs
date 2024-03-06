use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbi};

use crate::cmd::contract::generate_snippets::{
    snippet_gen_common::write_newline, snippet_sc_functions_gen::map_output_types_to_rust_types, snippet_type_map::{handle_abi_type, RustTypeString}
};

pub(crate) fn write_content(file: &mut File, abi: ContractAbi) {
    write_header_impl_constructors(file);
    for constructor_abi in abi.constructors {
        write_constructor_header(file, constructor_abi.clone());
        write_constructor_content(file, constructor_abi.inputs);
        write_end_of_function(file);
    }
    writeln!(file, "}}").unwrap();

    write_header_impl_endpoints(file);
    for endpoint_abi in abi.endpoints {
        write_endpoint_header(file, endpoint_abi.clone());
        // write_function_content(file, endpoint_abi);
        // write_end_of_function(file);
    }

    writeln!(file, "}}").unwrap();
}

fn write_header_impl_constructors(file: &mut File) {
    writeln!(
        file,
        r#"impl<Env, To, Gas> TxProxyMethods<Env, (), To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{"#
    )
    .unwrap();
}

fn write_header_impl_endpoints(file: &mut File) {
    writeln!(
        file,
        r#"impl<Env, From, To, Gas> TxProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{"#
    )
    .unwrap();
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

fn write_constructor_content(file: &mut File, inputs: Vec<InputAbi>) {
    writeln!(
        file,
        "\t\tself.wrapped_tx
            .raw_deploy()"
    )
    .unwrap();

    for input in inputs.iter() {
        writeln!(
            file,
            "\t\t\t.argument(&{})",
            input.arg_name // .argument(&arg0)"
        )
        .unwrap();
    }

    writeln!(file, "\t\t\t.original_result()").unwrap();
}

fn write_constructor_header(file: &mut File, contructor_abi: EndpointAbi) {
    write_fn_signature(file, contructor_abi);
    write_constructor_output(file);
}

fn write_endpoint_header(file: &mut File, contructor_abi: EndpointAbi) {
    write_fn_signature(file, contructor_abi.clone());
    write_endpoint_output(file, contructor_abi.outputs);
}

fn write_fn_signature(file: &mut File, endpoint_abi: EndpointAbi) {
    write_info_endpoint(file, endpoint_abi.docs);
    write_function_header_endpoint(file, endpoint_abi.rust_method_name);
    write_args(file, endpoint_abi.inputs.clone());
    write_parameters(file, endpoint_abi.inputs);
}

fn write_parameters(file: &mut File, inputs: Vec<InputAbi>) {
    writeln!(file, "(").unwrap();
    writeln!(file, "\t\tself,").unwrap();

    for (index, input) in inputs.iter().enumerate() {
        writeln!(file, "\t\t{}: Arg{index},", &input.arg_name).unwrap();
    }

    write!(file, "\t) ").unwrap();
}

fn write_constructor_output(file: &mut File) {
    writeln!(
        file,
        "-> multiversx_sc::types::Tx<Env, (), To, (), Gas, DeployCall<Env, ()>, OriginalResultMarker<()>>\n\t{{"
    )
    .unwrap();
}

fn write_endpoint_output(file: &mut File, outputs: Vec<OutputAbi>) {
    let output_type = map_output_types_to_rust_types(&outputs);

    println!("->>> output_type{}", output_type);

    let output_type_print = output_type.replace("<StaticApi>", "<Env::Api>");

    println!("{}", output_type_print);
    // write!(file, "{output_type_print}",).unwrap();
    // writeln!(file, "> {{").unwrap();
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
        write_argument(file, index, input.type_name.to_string());
    }

    write!(file, "\t>").unwrap();
}

fn write_argument(file: &mut File, index: usize, type_name: String) {
    let mut type_string = RustTypeString::default();
    handle_abi_type(&mut type_string, type_name);

    let type_print = type_string
        .get_type_name()
        .to_string()
        .replace("<StaticApi>", "<Env::Api>");

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
