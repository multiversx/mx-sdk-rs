use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbi};

use crate::cmd::contract::generate_snippets::{
    snippet_gen_common::write_newline,
    snippet_type_map::{handle_abi_type, RustTypeString},
};

use super::proxy_naming::proxy_methods_type_name;

pub(crate) fn write_content(file: &mut File, abi: ContractAbi) {
    write_header_impl_constructor(file, &abi.name);
    for constructor_abi in abi.constructors {
        write_constructor_header(file, constructor_abi.clone());
        write_constructor_content(file, constructor_abi.inputs);
        write_end_of_function(file);
    }
    writeln!(file, "}}").unwrap();

    write_header_impl_endpoints(file, &abi.name);
    for endpoint_abi in abi.endpoints {
        write_endpoint_header(file, endpoint_abi.clone());
        write_endpoint_content(file, endpoint_abi.name, endpoint_abi.inputs);
        write_end_of_function(file);
    }

    writeln!(file, "}}").unwrap();
}

fn write_header_impl_constructor(file: &mut File, name: &str) {
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"impl<Env, From, Gas> {proxy_methods_type_name}<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{{"#
    )
    .unwrap();
}

fn write_header_impl_endpoints(file: &mut File, name: &str) {
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"impl<Env, From, To, Gas> {proxy_methods_type_name}<Env, From, To, Gas>
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
    write_fn_signature(file, contructor_abi.clone());
    write_constructor_output(file, contructor_abi.outputs);
}

fn write_endpoint_header(file: &mut File, contructor_abi: EndpointAbi) {
    write_fn_signature(file, contructor_abi.clone());
    write_endpoint_output(file, contructor_abi.outputs);
}

fn write_fn_signature(file: &mut File, endpoint_abi: EndpointAbi) {
    write_endpoint_docs(file, endpoint_abi.docs);
    write_function_header_endpoint(file, endpoint_abi.rust_method_name);
    write_args(file, endpoint_abi.inputs.clone());
    write_parameters(file, endpoint_abi.inputs);
}

fn write_parameters(file: &mut File, inputs: Vec<InputAbi>) {
    writeln!(file, "(").unwrap();
    writeln!(file, "        self,").unwrap();
    for (index, input) in inputs.iter().enumerate() {
        writeln!(file, "        {}: Arg{index},", &input.arg_name).unwrap();
    }
    write!(file, "    ) ").unwrap();
}

fn write_constructor_output(file: &mut File, outputs: Vec<OutputAbi>) {
    write!(
        file,
        "-> Tx<
        Env,
        From,
        (),
        (),
        Gas,
        DeployCall<Env, ()>,
        OriginalResultMarker<"
    )
    .unwrap();

    parse_and_write_outputs(file, outputs);

    writeln!(
        file,
        ">,
    > {{"
    )
    .unwrap();
}

fn write_endpoint_output(file: &mut File, outputs: Vec<OutputAbi>) {
    write!(
        file,
        "-> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<"
    )
    .unwrap();

    parse_and_write_outputs(file, outputs);

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
        "        self.wrapped_tx
            .raw_deploy()"
    )
    .unwrap();
    for input in inputs.iter() {
        writeln!(file, "            .argument(&{})", input.arg_name).unwrap();
    }
    writeln!(file, "            .original_result()").unwrap();
}

fn write_endpoint_content(file: &mut File, function_name: String, inputs: Vec<InputAbi>) {
    writeln!(
        file,
        "        self.wrapped_tx
            .raw_call()
            .function_name(\"{}\")",
        function_name
    )
    .unwrap();

    for input in inputs.iter() {
        writeln!(file, "            .argument(&{})", input.arg_name).unwrap();
    }

    writeln!(file, "            .original_result()").unwrap();
}

fn write_function_header_endpoint(file: &mut File, rust_method_name: String) {
    write!(file, "    pub fn {rust_method_name}").unwrap();
}

fn write_endpoint_docs(file: &mut File, docs: Vec<String>) {
    if !docs.is_empty() {
        write!(file, "    /// ").unwrap();
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

    write!(file, "    >").unwrap();
}

fn write_argument(file: &mut File, index: usize, type_name: String) {
    let mut type_string = RustTypeString::default();
    handle_abi_type(&mut type_string, type_name);
    let type_print = type_string
        .get_type_name()
        .replace("StaticApi", "Env::Api")
        .to_string();

    writeln!(file, "        Arg{index}: CodecInto<{}>,", type_print).unwrap();
}

fn write_end_of_function(file: &mut File) {
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn adjust_type_name(original_rust_name: &str) -> String {
    original_rust_name
        .replace("multiversx_sc::api::uncallable::UncallableApi", "Env::Api")
        .replace("$API", "Env::Api")
        .to_string()
}

fn parse_and_write_outputs(file: &mut File, outputs: Vec<OutputAbi>) {
    match outputs.len() {
        0 => {
            write!(file, "()").unwrap();
        },
        1 => {
            let adjusted = adjust_type_name(&outputs[0].type_names.rust);
            write!(file, "{adjusted}").unwrap();
        },
        _ => panic!("multiple outputs not yet supported"),
    }
}
