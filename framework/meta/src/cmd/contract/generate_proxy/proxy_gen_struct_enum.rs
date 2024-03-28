use std::{fs::File, io::Write};

use multiversx_sc::abi::{
    EnumVariantDescription, StructFieldDescription, TypeContents, TypeDescription,
    TypeDescriptionContainerImpl,
};

const ZERO: &str = "0";

/// Types defined in the framework don't need to be generated again in the proxy.
const TYPES_FROM_FRAMEWORK: &[&str] = &[
    "EsdtTokenPayment<$API>",
    "EgldOrEsdtTokenPayment<$API>",
    "EsdtTokenData<$API>",
    "EgldOrEsdtTokenIdentifier<$API>",
    "EgldOrEsdtTokenPayment<$API>",
    "EgldOrMultiEsdtPayment<$API>",
    "EsdtTokenData<$API>",
];

pub(crate) fn write_types(file: &mut File, types: &TypeDescriptionContainerImpl) {
    for (_, type_description) in &types.0 {
        if TYPES_FROM_FRAMEWORK.contains(&type_description.names.rust.as_str()) {
            continue;
        }

        match &type_description.contents {
            TypeContents::Enum(enum_variants) => write_enum(file, enum_variants, type_description),
            TypeContents::Struct(struct_fields) => {
                write_struct(file, struct_fields, type_description)
            },
            TypeContents::NotSpecified => {},
            TypeContents::ExplicitEnum(_) => {},
        }
    }
}

fn start_write_type(file: &mut File, type_type: &str, type_description: &TypeDescription) {
    writeln!(file).unwrap();
    let type_name = type_description.names.rust.replace("$API", "Api");
    write_macro_attributes(file, &type_description.macro_attributes);
    write!(file, r#"pub {type_type} {type_name}"#).unwrap();

    if type_name.contains("<Api>") {
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
) {
    start_write_type(file, "struct", type_description);

    for field in struct_fields {
        writeln!(
            file,
            "    pub {}: {},",
            field.name,
            field.field_type.rust.replace("$API", "Api")
        )
        .unwrap();
    }

    writeln!(file, "}}").unwrap();
}

fn write_enum(
    file: &mut File,
    enum_variants: &Vec<EnumVariantDescription>,
    type_description: &TypeDescription,
) {
    start_write_type(file, "enum", type_description);

    for variant in enum_variants {
        write!(file, "    {}", variant.name).unwrap();
        if variant.fields.is_empty() {
            writeln!(file, ",").unwrap();
            continue;
        }

        if variant.fields[0].name == ZERO {
            write_tuple_in_variant(file, &variant.fields);
        } else {
            write_struct_in_variant(file, &variant.fields);
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

fn write_struct_in_variant(file: &mut File, fields: &[StructFieldDescription]) {
    writeln!(file, " {{").unwrap();

    for field in fields {
        writeln!(
            file,
            "        {}: {},",
            field.name,
            field.field_type.rust.replace("$API", "Api")
        )
        .unwrap();
    }

    writeln!(file, "    }},").unwrap();
}

fn write_tuple_in_variant(file: &mut File, fields: &[StructFieldDescription]) {
    write!(file, "(").unwrap();
    write!(file, "{}", fields[0].field_type.rust.replace("$API", "Api")).unwrap();

    for field in &fields[1..] {
        write!(file, ", {}", field.field_type.rust.replace("$API", "Api")).unwrap();
    }

    writeln!(file, "),").unwrap();
}
