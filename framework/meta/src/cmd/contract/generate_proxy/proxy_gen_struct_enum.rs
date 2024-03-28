use std::{fs::File, io::Write};

use multiversx_sc::abi::{
    EnumVariantDescription, StructFieldDescription, TypeContents, TypeDescription,
    TypeDescriptionContainerImpl,
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

pub(crate) fn write_types(
    file: &mut File,
    types: &TypeDescriptionContainerImpl,
    proxy_crate: &str,
) {
    for (_, type_description) in &types.0 {
        if proxy_crate != extract_struct_crate(type_description.names.rust.as_str()) {
            continue;
        }

        let type_name = adjust_type_name(&type_description.names.rust, proxy_crate);
        if TYPES_FROM_FRAMEWORK.contains(&type_name.as_str()) {
            continue;
        }

        match &type_description.contents {
            TypeContents::Enum(enum_variants) => write_enum(
                file,
                enum_variants,
                type_description,
                &type_name,
                proxy_crate,
            ),
            TypeContents::Struct(struct_fields) => write_struct(
                file,
                struct_fields,
                type_description,
                &type_name,
                proxy_crate,
            ),
            TypeContents::NotSpecified => {},
            TypeContents::ExplicitEnum(_) => {},
        }
    }
}

fn start_write_type(
    file: &mut File,
    type_type: &str,
    type_description: &TypeDescription,
    name: &str,
) {
    writeln!(file).unwrap();
    write_macro_attributes(file, &type_description.macro_attributes);
    write!(file, r#"pub {type_type} {name}"#).unwrap();

    if name.contains("<Api>") {
        writeln!(
            file,
            r#"
where
    Api: ManagedTypeApi,"#
        )
        .unwrap();
    } else {
        write!(file, " ").unwrap();
    }

    writeln!(file, r#"{{"#).unwrap();
}

fn write_struct(
    file: &mut File,
    struct_fields: &Vec<StructFieldDescription>,
    type_description: &TypeDescription,
    name: &str,
    proxy_crate: &str,
) {
    start_write_type(file, "struct", type_description, name);

    for field in struct_fields {
        writeln!(
            file,
            "    pub {}: {},",
            field.name,
            adjust_type_name(&field.field_type.rust, proxy_crate)
        )
        .unwrap();
    }

    writeln!(file, "}}").unwrap();
}

fn write_enum(
    file: &mut File,
    enum_variants: &Vec<EnumVariantDescription>,
    type_description: &TypeDescription,
    name: &str,
    proxy_crate: &str,
) {
    start_write_type(file, "enum", type_description, name);

    for variant in enum_variants {
        write!(file, "    {}", variant.name).unwrap();
        if variant.fields.is_empty() {
            writeln!(file, ",").unwrap();
            continue;
        }

        if variant.fields[0].name == ZERO {
            write_tuple_in_variant(file, &variant.fields, proxy_crate);
        } else {
            write_struct_in_variant(file, &variant.fields, proxy_crate);
        }
    }
    writeln!(file, "}}").unwrap();
}

fn write_macro_attributes(file: &mut File, macro_attributes: &[String]) {
    if macro_attributes.is_empty() {
        writeln!(file, "#[derive(TopEncode, TopDecode)]").unwrap();
    } else {
        writeln!(file, "#[derive({})]", macro_attributes.join(", ")).unwrap();
    }
}

fn write_struct_in_variant(file: &mut File, fields: &[StructFieldDescription], proxy_crate: &str) {
    writeln!(file, " {{").unwrap();

    for field in fields {
        writeln!(
            file,
            "        {}: {},",
            field.name,
            adjust_type_name(&field.field_type.rust, proxy_crate)
        )
        .unwrap();
    }

    writeln!(file, "    }},").unwrap();
}

fn write_tuple_in_variant(file: &mut File, fields: &[StructFieldDescription], proxy_crate: &str) {
    write!(file, "(").unwrap();
    write!(
        file,
        "{}",
        adjust_type_name(&fields[0].field_type.rust, proxy_crate)
    )
    .unwrap();

    for field in &fields[1..] {
        write!(
            file,
            ", {}",
            adjust_type_name(&field.field_type.rust, proxy_crate)
        )
        .unwrap();
    }

    writeln!(file, "),").unwrap();
}

fn extract_struct_crate(struct_path: &str) -> String {
    let struct_crate_name = struct_path
        .replace('_', "-")
        .replace("multiversx_sc::api::uncallable::UncallableApi", "Api")
        .to_string();
    let crate_name = struct_crate_name
        .split("::")
        .next()
        .unwrap_or_else(|| &struct_crate_name);
    crate_name.to_string()
}

pub(crate) fn clean_paths(proxy_crate: &str, rust_type: &str) -> String {
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

pub fn adjust_type_name(original_rust_name: &str, proxy_crate: &str) -> String {
    clean_paths(
        proxy_crate,
        &original_rust_name
            .replace("multiversx_sc::api::uncallable::UncallableApi", "Api")
            .replace("$API", "Api"),
    )
}
