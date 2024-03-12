use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbi};

use crate::cmd::contract::generate_snippets::{
    snippet_gen_common::write_newline,
    snippet_type_map::{handle_abi_type, RustTypeString},
};

const ENV: &str = "Env:";
const COLON: char = ':';

pub(crate) fn write_content(file: &mut File, abi: ContractAbi) {
    write_header_impl_constructor(file);
    for constructor_abi in abi.constructors {
        write_constructor_header(file, constructor_abi.clone());
        write_constructor_content(file, constructor_abi.inputs);
        write_end_of_function(file);
    }
    writeln!(file, "}}").unwrap();

    write_header_impl_endpoints(file);
    for endpoint_abi in abi.endpoints {
        write_endpoint_header(file, endpoint_abi.clone());
        write_endpoint_content(file, endpoint_abi.name, endpoint_abi.inputs);
        write_end_of_function(file);
    }

    writeln!(file, "}}").unwrap();
}

fn write_header_impl_constructor(file: &mut File) {
    writeln!(
        file,
        r#"impl<Env, To, Gas> TxProxyMethods<Env, (), To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
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
    write!(
        file,
        "-> multiversx_sc::types::Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<"
    )
    .unwrap();

    if outputs.len() == 0 {
        write!(file, "()").unwrap();
    } else {
        parse_and_write_outputs(file, outputs);
    }

    writeln!(
        file,
        ">,
    > {{"
    )
    .unwrap();
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

fn write_endpoint_content(file: &mut File, function_name: String, inputs: Vec<InputAbi>) {
    writeln!(
        file,
        "\t\tself.wrapped_tx
            .raw_call()
            .function_name(\"{}\")",
        function_name
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
    let type_print = type_string
        .get_type_name()
        .replace("StaticApi", "Env::Api")
        .to_string();

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

fn parse_and_write_outputs(file: &mut File, outputs: Vec<OutputAbi>) {
    for output in outputs {
        let env_api = output
            .type_names
            .rust
            .replace("multiversx_sc::api::uncallable::UncallableApi", "Env::Api")
            .to_string();

        let mut current_string = String::new();
        let mut found_words = Vec::new();

        for character in env_api.chars() {
            if character == COLON {
                // adjust_on_colon_suffix(character, current_string, found_words);
                if current_string.ends_with(COLON) && !current_string.ends_with(ENV){
                    if current_string.len() > 0
                        && current_string.chars().next().unwrap().is_uppercase()
                    {
                        found_words.push(current_string[..current_string.len() - 1].to_string());
                    }
                    current_string.clear();
                } else {
                    current_string.push(character);
                }
            } else if character == ' ' && current_string.ends_with(',') {
                if current_string.len() > 0 && current_string.chars().next().unwrap().is_uppercase()
                {
                    current_string.push(character);
                    found_words.push(current_string.clone());
                    current_string.clear();
                } else {
                    current_string.push(character);
                }
            } else {
                current_string.push(character);
            }
        }

        found_words.push(current_string);
        write!(file, "{}", found_words.join("")).unwrap();
    }
}