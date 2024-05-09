use std::{fmt::Display, fs::File, io::Write};

use multiversx_sc::abi::{
    EndpointAbi, EnumVariantDescription, InputAbi, OutputAbi, StructFieldDescription, TypeContents,
    TypeDescription,
};

use crate::cmd::contract::{meta_config::MetaConfig, sc_config::ProxyConfigSerde};

use super::proxy_process_type_name::{
    extract_paths, extract_struct_crate, process_rust_type, proxy_methods_type_name,
    proxy_type_name,
};

const PRELUDE: &str = "// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]
";

const IMPORTS: &str = "use multiversx_sc::proxy_imports::*;";

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
    "EsdtTokenType",
];

pub struct ProxyGenerator<'a> {
    pub meta_config: &'a MetaConfig,
    pub file: Option<&'a mut File>,
    pub proxy_config: &'a ProxyConfigSerde,
}

impl<'a> ProxyGenerator<'a> {
    pub fn new(
        meta_config: &'a MetaConfig,
        file: &'a mut File,
        proxy_config: &'a ProxyConfigSerde,
    ) -> Self {
        Self {
            meta_config,
            file: Some(file),
            proxy_config,
        }
    }

    fn write(&mut self, s: impl Display) {
        let file = self.file.as_mut().expect("output not configured");
        write!(*file, "{s}").unwrap();
    }

    fn writeln(&mut self, s: impl Display) {
        self.write(s);
        self.write("\n");
    }

    pub fn write_proxy_to_file(&mut self) {
        self.write_header();
        self.write_tx_proxy_type_def();
        self.write_impl_for_tx_proxy();
        self.write_struct_tx_proxy_methods();
        self.write_content();
        self.write_types();
    }

    fn write_header(&mut self) {
        self.writeln(PRELUDE);
        match &self.proxy_config.override_import {
            Some(override_import) => self.writeln(override_import),
            None => self.writeln(IMPORTS),
        }
    }

    fn write_tx_proxy_type_def(&mut self) {
        let proxy_type_name = proxy_type_name(&self.meta_config.original_contract_abi.name);
        self.writeln(format!(
            r#"
pub struct {proxy_type_name};"#
        ));
    }

    fn write_impl_for_tx_proxy(&mut self) {
        let proxy_type_name = proxy_type_name(&self.meta_config.original_contract_abi.name);
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        self.writeln(format!(
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
        ));
    }

    fn write_struct_tx_proxy_methods(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        self.writeln(format!(
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
        ));
    }

    fn write_content(&mut self) {
        if !self
            .meta_config
            .original_contract_abi
            .constructors
            .is_empty()
        {
            self.write_constructors();
        }

        if !self
            .meta_config
            .original_contract_abi
            .upgrade_constructors
            .is_empty()
        {
            self.write_upgrades();
        }

        if !self.meta_config.original_contract_abi.endpoints.is_empty() {
            self.write_endpoints();
        }
    }

    fn write_types(&mut self) {
        for (_, type_description) in &self.meta_config.original_contract_abi.type_descriptions.0 {
            if self
                .meta_config
                .original_contract_abi
                .get_crate_name_for_code()
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
                    self.write_enum(enum_variants, type_description, &type_name)
                },
                TypeContents::Struct(struct_fields) => {
                    self.write_struct(struct_fields, type_description, &type_name)
                },
                TypeContents::NotSpecified => {},
                TypeContents::ExplicitEnum(_) => {},
            }
        }
    }

    fn write_constructors(&mut self) {
        let constructors: Vec<EndpointAbi> =
            self.meta_config.original_contract_abi.constructors.clone();

        self.write_header_impl_constructor();
        for (i, constructor_abi) in constructors.into_iter().enumerate() {
            if i > 0 {
                self.writeln("");
            }
            self.write_constructor_header(&constructor_abi);
            self.write_constructor_content(constructor_abi.inputs);
            self.write_end_of_function();
        }

        self.writeln("}");
    }

    fn write_upgrades(&mut self) {
        self.write_header_impl_upgrade();
        for (i, upgrade) in self
            .meta_config
            .original_contract_abi
            .upgrade_constructors
            .clone()
            .into_iter()
            .enumerate()
        {
            if i > 0 {
                self.writeln("");
            }
            self.write_upgrade_header(&upgrade);
            self.write_upgrade_content(upgrade.inputs);
            self.write_end_of_function();
        }

        self.writeln("}");
    }

    fn write_endpoints(&mut self) {
        let endpoints: Vec<EndpointAbi> = self.meta_config.original_contract_abi.endpoints.clone();

        self.write_header_impl_endpoints();
        for (i, endpoint_abi) in endpoints.into_iter().enumerate() {
            if i > 0 {
                self.writeln("");
            }
            self.write_endpoint_header(&endpoint_abi);
            self.write_endpoint_content(&endpoint_abi);
            self.write_end_of_function();
        }

        self.writeln("}");
    }

    fn write_header_impl_constructor(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        self.writeln(format!(
            r#"
#[rustfmt::skip]
impl<Env, From, Gas> {proxy_methods_type_name}<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{{"#
        ));
    }

    fn write_header_impl_upgrade(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        self.writeln(format!(
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
        ));
    }

    fn write_header_impl_endpoints(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        self.writeln(format!(
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
        ));
    }

    fn write_constructor_header(&mut self, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(constructor_abi);
        self.write_constructor_output(&constructor_abi.outputs);
    }

    fn write_upgrade_header(&mut self, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(constructor_abi);
        self.write_upgrade_output(&constructor_abi.outputs);
    }

    fn write_endpoint_header(&mut self, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(constructor_abi);
        self.write_endpoint_output(&constructor_abi.outputs);
    }

    fn write_constructor_content(&mut self, inputs: Vec<InputAbi>) {
        self.writeln(
            "        self.wrapped_tx
            .raw_deploy()",
        );
        for input in inputs.iter() {
            self.writeln(format!("            .argument(&{})", input.arg_name));
        }
        self.writeln("            .original_result()");
    }

    fn write_upgrade_content(&mut self, inputs: Vec<InputAbi>) {
        self.writeln(
            "        self.wrapped_tx
            .raw_upgrade()",
        );
        for input in inputs.iter() {
            self.writeln(format!("            .argument(&{})", input.arg_name));
        }
        self.writeln("            .original_result()");
    }

    fn write_endpoint_content(&mut self, endpoint: &EndpointAbi) {
        self.writeln(format!(
            "        self.wrapped_tx
            .raw_call(\"{}\")",
            endpoint.name
        ));

        for input in endpoint.inputs.iter() {
            self.writeln(format!("            .argument(&{})", input.arg_name));
        }

        self.writeln("            .original_result()");
    }

    fn write_fn_signature(&mut self, endpoint: &EndpointAbi) {
        self.write_endpoint_docs(&endpoint.docs);
        self.write_function_header_endpoint(&endpoint.rust_method_name);
        self.write_args(&endpoint.inputs);
        self.write_parameters(&endpoint.inputs);
    }

    fn write_endpoint_docs(&mut self, docs: &Vec<String>) {
        for doc in docs {
            self.writeln(format!("    /// {doc} "));
        }
    }

    fn write_function_header_endpoint(&mut self, rust_method_name: &String) {
        self.write(format!("    pub fn {rust_method_name}"));
    }

    fn write_args(&mut self, inputs: &[InputAbi]) {
        if inputs.is_empty() {
            return;
        }

        self.writeln("<");

        for (index, input) in inputs.iter().enumerate() {
            self.write_argument(index, &input.type_names.rust);
        }

        self.write("    >");
    }

    fn write_argument(&mut self, index: usize, rust_name: &str) {
        let adjusted = self.adjust_type_name_with_env_api(rust_name);
        self.writeln(format!("        Arg{index}: ProxyArg<{adjusted}>,"));
    }

    fn write_parameters(&mut self, inputs: &[InputAbi]) {
        self.writeln("(");
        self.writeln("        self,");
        for (index, input) in inputs.iter().enumerate() {
            self.writeln(format!("        {}: Arg{index},", &input.arg_name));
        }
        self.write("    ) ");
    }

    fn write_constructor_output(&mut self, outputs: &[OutputAbi]) {
        self.write("-> TxProxyDeploy<Env, From, Gas, ");

        self.parse_and_write_outputs(outputs);

        self.writeln("> {");
    }

    fn write_upgrade_output(&mut self, outputs: &[OutputAbi]) {
        self.write("-> TxProxyUpgrade<Env, From, To, Gas, ");

        self.parse_and_write_outputs(outputs);

        self.writeln("> {");
    }

    fn write_endpoint_output(&mut self, outputs: &[OutputAbi]) {
        self.write("-> TxProxyCall<Env, From, To, Gas, ");

        self.parse_and_write_outputs(outputs);

        self.writeln("> {");
    }

    fn parse_and_write_outputs(&mut self, outputs: &[OutputAbi]) {
        match outputs.len() {
            0 => {
                self.write("()");
            },
            1 => {
                let adjusted = self.adjust_type_name_with_env_api(&outputs[0].type_names.rust);
                self.write(adjusted);
            },
            _ => {
                self.write(format!("MultiValue{}<", outputs.len()));
                for (i, output) in outputs.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    let adjusted = self.adjust_type_name_with_env_api(&output.type_names.rust);
                    self.write(adjusted);
                }
                self.write(">");
            },
        }
    }

    fn write_enum(
        &mut self,
        enum_variants: &Vec<EnumVariantDescription>,
        type_description: &TypeDescription,
        name: &str,
    ) {
        if self.enum_contains_struct_variant(enum_variants) {
            self.write("\n#[rustfmt::skip]");
        }

        self.start_write_type("enum", type_description, name);

        if enum_variants.is_empty() {
            self.writeln("}");
            return;
        }

        self.writeln("");

        for variant in enum_variants {
            self.write(format!("    {}", variant.name));
            if variant.fields.is_empty() {
                self.writeln(",");
                continue;
            }

            if variant.fields[0].name == ZERO {
                self.write_tuple_in_variant(&variant.fields);
            } else {
                self.write_struct_in_variant(&variant.fields);
            }
        }
        self.writeln("}");
    }

    fn write_struct(
        &mut self,
        struct_fields: &Vec<StructFieldDescription>,
        type_description: &TypeDescription,
        name: &str,
    ) {
        self.start_write_type("struct", type_description, name);

        if struct_fields.is_empty() {
            self.writeln("}");
            return;
        }

        self.writeln("");

        for field in struct_fields {
            let adjusted_type_name = self.adjust_type_name_with_api(&field.field_type.rust);
            self.writeln(format!("    pub {}: {adjusted_type_name},", field.name));
        }

        self.writeln("}");
    }

    fn write_tuple_in_variant(&mut self, fields: &[StructFieldDescription]) {
        self.write("(");
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            let adjusted_type_name = self.adjust_type_name_with_api(&field.field_type.rust);
            self.write(adjusted_type_name);
        }

        self.writeln("),");
    }

    fn write_struct_in_variant(&mut self, fields: &[StructFieldDescription]) {
        self.writeln(" {");

        for field in fields {
            let adjusted_type_name = self.adjust_type_name_with_api(&field.field_type.rust);
            self.writeln(format!("        {}: {adjusted_type_name},", field.name,));
        }

        self.writeln("    },");
    }

    pub fn clean_paths(&mut self, rust_type: &str) -> String {
        let paths = extract_paths(rust_type);

        let processed_paths = self.process_paths(&paths);

        let processed_rust_type = process_rust_type(rust_type.to_string(), paths, processed_paths);

        self.rename_path_with_custome_config(&processed_rust_type)
    }

    fn start_write_type(
        &mut self,
        type_type: &str,
        type_description: &TypeDescription,
        name: &str,
    ) {
        self.writeln("");
        self.writeln("#[type_abi]");
        self.write_macro_attributes(&type_description.macro_attributes);
        self.write(format!(r#"pub {type_type} {name}"#));

        if name.contains("<Api>") {
            self.writeln(
                "
where
    Api: ManagedTypeApi,",
            );
        } else {
            self.write(" ");
        }

        self.write("{");
    }

    pub fn write_macro_attributes(&mut self, macro_attributes: &[String]) {
        if macro_attributes.is_empty() {
            self.writeln("#[derive(TopEncode, TopDecode)]");
        } else {
            self.writeln(format!("#[derive({})]", macro_attributes.join(", ")));
        }
    }

    fn adjust_type_name_with_env_api(&mut self, original_rust_name: &str) -> String {
        self.clean_paths(
            &original_rust_name
                .replace("multiversx_sc::api::uncallable::UncallableApi", "Env::Api")
                .replace("$API", "Env::Api"),
        )
    }

    fn adjust_type_name_with_api(&mut self, original_rust_name: &str) -> String {
        self.clean_paths(
            &original_rust_name
                .replace("multiversx_sc::api::uncallable::UncallableApi", "Api")
                .replace("$API", "Api"),
        )
    }

    fn write_end_of_function(&mut self) {
        self.writeln("    }");
    }

    fn rename_path_with_custome_config(&self, processed_type: &str) -> String {
        let mut renamed_processed_type = processed_type.to_owned();

        if let Some(paths_rename) = &self.proxy_config.path_rename {
            for path_rename in paths_rename {
                if processed_type.contains(&path_rename.from) {
                    renamed_processed_type =
                        renamed_processed_type.replace(&path_rename.from, &path_rename.to);
                }
            }
        }

        renamed_processed_type
    }

    fn process_paths(&self, paths: &Vec<String>) -> Vec<String> {
        let mut processed_paths: Vec<String> = Vec::new();
        let crate_name = self
            .meta_config
            .original_contract_abi
            .get_crate_name_for_code();

        for path in paths {
            let type_rust_name = path.split("::").last().unwrap();
            if crate_name == extract_struct_crate(path)
                || TYPES_FROM_FRAMEWORK.contains(&type_rust_name)
            {
                processed_paths.push(type_rust_name.to_string());
            } else {
                processed_paths.push(path.to_string());
            }
        }

        processed_paths
    }

    fn enum_contains_struct_variant(&self, enum_variants: &Vec<EnumVariantDescription>) -> bool {
        for variant in enum_variants {
            if variant.fields.is_empty() {
                continue;
            }

            if variant.fields[0].name != ZERO {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
pub mod tests {
    use multiversx_sc::abi::{BuildInfoAbi, ContractAbi, ContractCrateBuildAbi, FrameworkBuildAbi};

    use crate::cmd::contract::{meta_config::MetaConfig, sc_config::ProxyConfigSerde};

    use super::ProxyGenerator;

    #[test]
    fn clean_paths_unsanitized_test() {
        let build_info = BuildInfoAbi {
            contract_crate: ContractCrateBuildAbi {
                name: "contract-crate",
                version: "0.0.0",
                git_version: "0.0.0",
            },
            framework: FrameworkBuildAbi::create(),
        };

        let original_contract_abi = ContractAbi::new(build_info, &[""], "contract-crate", false);
        let meta_config = MetaConfig::create(original_contract_abi, false);
        let mut proxy_generator = ProxyGenerator {
            meta_config: &meta_config,
            file: None,
            proxy_config: &ProxyConfigSerde::new(),
        };

        let cleaned_path_unsanitized = proxy_generator.clean_paths(
            "(other_crate::contract_crate::TestStruct, Option<Box<contract_crate::other_crate::AbiTestType>>)",
        );
        let expected_result_unsanitized =
            "(other_crate::contract_crate::TestStruct, Option<Box<AbiTestType>>)";

        assert_eq!(
            expected_result_unsanitized,
            cleaned_path_unsanitized.as_str()
        );
    }

    #[test]
    fn clean_paths_sanitized_test() {
        let build_info = BuildInfoAbi {
            contract_crate: ContractCrateBuildAbi {
                name: "contract-crate",
                version: "0.0.0",
                git_version: "0.0.0",
            },
            framework: FrameworkBuildAbi::create(),
        };

        let original_contract_abi = ContractAbi::new(build_info, &[""], "contract-crate", false);
        let meta_config = MetaConfig::create(original_contract_abi, false);
        let mut proxy_generator = ProxyGenerator {
            meta_config: &meta_config,
            file: None,
            proxy_config: &ProxyConfigSerde::new(),
        };

        let cleaned_path_sanitized = proxy_generator.clean_paths(
            "(contract_crate::other_crate::TestStruct, Option<Box<contract_crate::AbiTestType>>)",
        );
        let expected_result_sanitized = "(TestStruct, Option<Box<AbiTestType>>)";

        assert_eq!(expected_result_sanitized, cleaned_path_sanitized.as_str());
    }
}
