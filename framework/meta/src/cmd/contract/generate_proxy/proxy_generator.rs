use std::{fs::File, io::Write};

use multiversx_sc::abi::{
    EndpointAbi, EnumVariantDescription, InputAbi, OutputAbi, StructFieldDescription, TypeContents,
    TypeDescription,
};

use crate::cmd::contract::meta_config::MetaConfig;

use super::{
    proxy_naming::{proxy_methods_type_name, proxy_type_name},
    proxy_template_gen::{
        extract_struct_crate, start_write_type, write_constructor_content, write_end_of_function,
        write_endpoint_content, write_endpoint_docs, write_function_header_endpoint, write_header,
        write_parameters, write_upgrade_content,
    },
};

const ZERO: &str = "0";

/// Types defined in the framework don't need to be generated again in the proxy.
const TYPES_FROM_FRAMEWORK: &[&str] = &[
    "EsdtTokenPayment",
    "EgldOrEsdtTokenPayment",
    "EsdtTokenData",
    "EgldOrEsdtTokenIdentifier",
    "EgldOrEsdtTokenPayment",
    "EgldOrMultiEsdtPayment",
    "EsdtTokenData",
    "EsdtLocalRole",
];

pub struct ProxyGenerator<'a> {
    pub meta_config: &'a MetaConfig,
}

impl<'a> ProxyGenerator<'a> {
    pub const fn new(meta_config: &'a MetaConfig) -> Self {
        ProxyGenerator { meta_config }
    }

    pub fn write_proxy_to_file(&self, mut file: File) {
        write_header(&mut file);
        self.write_tx_proxy_type_def(&mut file);
        self.write_impl_for_tx_proxy(&mut file);
        self.write_struct_tx_proxy_methods(&mut file);
        self.write_content(&mut file);
        self.write_types(&mut file);
    }

    fn write_tx_proxy_type_def(&self, file: &mut File) {
        let proxy_type_name = proxy_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            file,
            r#"
pub struct {proxy_type_name};"#
        )
        .unwrap();
    }

    fn write_impl_for_tx_proxy(&self, file: &mut File) {
        let proxy_type_name = proxy_type_name(&self.meta_config.original_contract_abi.name);
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            file,
            r#"
impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for {proxy_type_name}
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    type TxProxyMethods = {proxy_methods_type_name}<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {{
        {proxy_methods_type_name} {{ wrapped_tx: tx }}
    }}
}}"#
        )
        .unwrap();
    }

    fn write_struct_tx_proxy_methods(&self, file: &mut File) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            file,
            r#"
pub struct {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}}"#
        )
        .unwrap();
    }

    fn write_content(&self, file: &mut File) {
        if !self
            .meta_config
            .original_contract_abi
            .constructors
            .is_empty()
        {
            self.write_constructors(file);
        }

        if !self
            .meta_config
            .original_contract_abi
            .upgrade_constructors
            .is_empty()
        {
            self.write_upgrades(file);
        }

        if !self.meta_config.original_contract_abi.endpoints.is_empty() {
            self.write_endpoints(file);
        }
    }

    fn write_types(&self, file: &mut File) {
        for (_, type_description) in &self.meta_config.original_contract_abi.type_descriptions.0 {
            if self
                .meta_config
                .original_contract_abi
                .build_info
                .contract_crate
                .name
                != extract_struct_crate(type_description.names.rust.as_str())
            {
                continue;
            }

            let type_name = self.adjust_type_name_with_api(&type_description.names.rust);
            if TYPES_FROM_FRAMEWORK.contains(&type_name.as_str()) {
                continue;
            }

            match &type_description.contents {
                TypeContents::Enum(enum_variants) => {
                    self.write_enum(file, enum_variants, type_description, &type_name)
                },
                TypeContents::Struct(struct_fields) => {
                    self.write_struct(file, struct_fields, type_description, &type_name)
                },
                TypeContents::NotSpecified => {},
                TypeContents::ExplicitEnum(_) => {},
            }
        }
    }

    fn write_constructors(&self, file: &mut File) {
        let constructors: Vec<EndpointAbi> =
            self.meta_config.original_contract_abi.constructors.clone();

        self.write_header_impl_constructor(file);
        for (i, constructor_abi) in constructors.into_iter().enumerate() {
            if i > 0 {
                writeln!(file).unwrap();
            }
            self.write_constructor_header(file, &constructor_abi);
            write_constructor_content(file, constructor_abi.inputs);
            write_end_of_function(file);
        }

        writeln!(file, "}}").unwrap();
    }

    fn write_upgrades(&self, file: &mut File) {
        self.write_header_impl_upgrade(file);
        for (i, upgrade) in self
            .meta_config
            .original_contract_abi
            .upgrade_constructors
            .clone()
            .into_iter()
            .enumerate()
        {
            if i > 0 {
                writeln!(file).unwrap();
            }
            self.write_upgrade_header(file, &upgrade);
            write_upgrade_content(file, upgrade.inputs);
            write_end_of_function(file);
        }

        writeln!(file, "}}").unwrap();
    }

    fn write_endpoints(&self, file: &mut File) {
        let endpoints: Vec<EndpointAbi> = self.meta_config.original_contract_abi.endpoints.clone();

        self.write_header_impl_endpoints(file);
        for (i, endpoint_abi) in endpoints.into_iter().enumerate() {
            if i > 0 {
                writeln!(file).unwrap();
            }
            self.write_endpoint_header(file, &endpoint_abi);
            write_endpoint_content(file, &endpoint_abi);
            write_end_of_function(file);
        }

        writeln!(file, "}}").unwrap();
    }

    fn write_header_impl_constructor(&self, file: &mut File) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            file,
            r#"
#[rustfmt::skip]
impl<Env, From, Gas> {proxy_methods_type_name}<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{{"#
        )
        .unwrap();
    }

    fn write_header_impl_upgrade(&self, file: &mut File) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            file,
            r#"
#[rustfmt::skip]
impl<Env, From, To, Gas> {proxy_methods_type_name}<Env, From, To, Gas>
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

    fn write_header_impl_endpoints(&self, file: &mut File) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            file,
            r#"
#[rustfmt::skip]
impl<Env, From, To, Gas> {proxy_methods_type_name}<Env, From, To, Gas>
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

    fn write_constructor_header(&self, file: &mut File, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(file, constructor_abi);
        self.write_constructor_output(file, &constructor_abi.outputs);
    }

    fn write_upgrade_header(&self, file: &mut File, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(file, constructor_abi);
        self.write_upgrade_output(file, &constructor_abi.outputs);
    }

    fn write_endpoint_header(&self, file: &mut File, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(file, constructor_abi);
        self.write_endpoint_output(file, &constructor_abi.outputs);
    }

    fn write_fn_signature(&self, file: &mut File, endpoint: &EndpointAbi) {
        write_endpoint_docs(file, &endpoint.docs);
        write_function_header_endpoint(file, &endpoint.rust_method_name);
        self.write_args(file, &endpoint.inputs);
        write_parameters(file, &endpoint.inputs);
    }

    fn write_args(&self, file: &mut File, inputs: &[InputAbi]) {
        if inputs.is_empty() {
            return;
        }

        writeln!(file, "<").unwrap();

        for (index, input) in inputs.iter().enumerate() {
            self.write_argument(file, index, &input.type_names.rust);
        }

        write!(file, "    >").unwrap();
    }

    fn write_argument(&self, file: &mut File, index: usize, rust_name: &str) {
        let adjusted = self.adjust_type_name_with_env_api(rust_name);
        writeln!(file, "        Arg{index}: CodecInto<{adjusted}>,").unwrap();
    }

    fn write_constructor_output(&self, file: &mut File, outputs: &[OutputAbi]) {
        write!(file, "-> TxProxyDeploy<Env, From, Gas, ").unwrap();

        self.parse_and_write_outputs(file, outputs);

        writeln!(file, "> {{").unwrap();
    }

    fn write_upgrade_output(&self, file: &mut File, outputs: &[OutputAbi]) {
        write!(file, "-> TxProxyUpgrade<Env, From, To, Gas, ").unwrap();

        self.parse_and_write_outputs(file, outputs);

        writeln!(file, "> {{").unwrap();
    }

    fn write_endpoint_output(&self, file: &mut File, outputs: &[OutputAbi]) {
        write!(file, "-> TxProxyCall<Env, From, To, Gas, ").unwrap();

        self.parse_and_write_outputs(file, outputs);

        writeln!(file, "> {{").unwrap();
    }

    fn parse_and_write_outputs(&self, file: &mut File, outputs: &[OutputAbi]) {
        match outputs.len() {
            0 => {
                write!(file, "()").unwrap();
            },
            1 => {
                let adjusted = self.adjust_type_name_with_env_api(&outputs[0].type_names.rust);
                write!(file, "{adjusted}").unwrap();
            },
            _ => {
                write!(file, "MultiValue{}<", outputs.len()).unwrap();
                for (i, output) in outputs.iter().enumerate() {
                    if i > 0 {
                        write!(file, ", ").unwrap();
                    }
                    let adjusted = self.adjust_type_name_with_env_api(&output.type_names.rust);
                    write!(file, "{adjusted}").unwrap();
                }
                write!(file, ">").unwrap();
            },
        }
    }

    fn write_enum(
        &self,
        file: &mut File,
        enum_variants: &Vec<EnumVariantDescription>,
        type_description: &TypeDescription,
        name: &str,
    ) {
        start_write_type(file, "enum", type_description, name);

        for variant in enum_variants {
            write!(file, "    {}", variant.name).unwrap();
            if variant.fields.is_empty() {
                writeln!(file, ",").unwrap();
                continue;
            }

            if variant.fields[0].name == ZERO {
                self.write_tuple_in_variant(file, &variant.fields);
            } else {
                self.write_struct_in_variant(file, &variant.fields);
            }
        }
        writeln!(file, "}}").unwrap();
    }

    fn write_struct(
        &self,
        file: &mut File,
        struct_fields: &Vec<StructFieldDescription>,
        type_description: &TypeDescription,
        name: &str,
    ) {
        start_write_type(file, "struct", type_description, name);

        for field in struct_fields {
            writeln!(
                file,
                "    pub {}: {},",
                field.name,
                self.adjust_type_name_with_api(&field.field_type.rust)
            )
            .unwrap();
        }

        writeln!(file, "}}").unwrap();
    }

    fn write_tuple_in_variant(&self, file: &mut File, fields: &[StructFieldDescription]) {
        write!(file, "(").unwrap();
        write!(
            file,
            "{}",
            self.adjust_type_name_with_api(&fields[0].field_type.rust)
        )
        .unwrap();

        for field in &fields[1..] {
            write!(
                file,
                ", {}",
                self.adjust_type_name_with_api(&field.field_type.rust)
            )
            .unwrap();
        }

        writeln!(file, "),").unwrap();
    }

    fn write_struct_in_variant(&self, file: &mut File, fields: &[StructFieldDescription]) {
        writeln!(file, " {{").unwrap();

        for field in fields {
            writeln!(
                file,
                "        {}: {},",
                field.name,
                self.adjust_type_name_with_api(&field.field_type.rust)
            )
            .unwrap();
        }

        writeln!(file, "    }},").unwrap();
    }

    fn clean_paths(&self, proxy_crate: &str, rust_type: &str) -> String {
        let delimiters = "<>,()[] ";
        let words: Vec<&str> = rust_type
            .split(|c| delimiters.contains(c))
            .filter(|s| !s.is_empty())
            .collect();

        let mut words_replacer: Vec<String> = Vec::new();
        for word in &words {
            let type_rust_name = word.split("::").last().unwrap().to_string();
            if proxy_crate == extract_struct_crate(word)
                || TYPES_FROM_FRAMEWORK.contains(&type_rust_name.as_str())
            {
                words_replacer.push(type_rust_name);
            } else {
                words_replacer.push(word.to_string());
            }
        }

        let mut rust_type_with_cleaned_path: String = rust_type.to_string().clone();
        for index in 0..words.len() {
            rust_type_with_cleaned_path = rust_type_with_cleaned_path.replace(
                words.get(index).unwrap(),
                words_replacer.get(index).unwrap(),
            );
        }

        rust_type_with_cleaned_path
    }

    fn adjust_type_name_with_env_api(&self, original_rust_name: &str) -> String {
        self.clean_paths(
            self.meta_config
                .original_contract_abi
                .build_info
                .contract_crate
                .name,
            &original_rust_name
                .replace("multiversx_sc::api::uncallable::UncallableApi", "Env::Api")
                .replace("$API", "Env::Api"),
        )
    }

    fn adjust_type_name_with_api(&self, original_rust_name: &str) -> String {
        self.clean_paths(
            self.meta_config
                .original_contract_abi
                .build_info
                .contract_crate
                .name,
            &original_rust_name
                .replace("multiversx_sc::api::uncallable::UncallableApi", "Api")
                .replace("$API", "Api"),
        )
    }
}
