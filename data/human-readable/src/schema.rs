use std::{error::Error, fmt::Display};

use multiversx_sc_scenario::multiversx_sc::abi::{
    ContractAbi, EnumVariantDescription, StructFieldDescription, TypeContents, TypeDescription,
};
use serde_json::Value as JsonValue;

pub fn build_schema_for_type(
    type_name: &str,
    contract_abi: &ContractAbi,
) -> Result<JsonValue, SchemaError> {
    let type_description = contract_abi.type_descriptions.find_or_default(type_name);
    build_schema_for_type_description(&type_description, contract_abi)
}

pub fn build_schema_for_type_description(
    type_description: &TypeDescription,
    contract_abi: &ContractAbi,
) -> Result<JsonValue, SchemaError> {
    match &type_description.contents {
        TypeContents::NotSpecified => build_schema_for_single_value(&type_description.names.abi),
        TypeContents::Enum(variants) => build_schema_for_enum(variants, contract_abi),
        TypeContents::Struct(fields) => build_schema_for_struct(fields, contract_abi),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

pub fn build_schema_for_single_value(type_name: &str) -> Result<JsonValue, SchemaError> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => Ok(JsonValue::Object(
            vec![("type".to_owned(), "integer".into())]
                .into_iter()
                .collect(),
        )),
        "BigInt" | "i64" | "i32" | "i16" | "isize" | "i8" => Ok(JsonValue::Object(
            vec![("type".to_owned(), "integer".into())]
                .into_iter()
                .collect(),
        )),
        "ManagedBuffer" => Ok(JsonValue::Object(
            vec![
                ("type".to_owned(), "array".into()),
                ("items".to_owned(), build_schema_for_single_value("u8")?),
            ]
            .into_iter()
            .collect(),
        )),
        "string" | "utf-8 string" | "Address" => Ok(JsonValue::Object(
            vec![("type".to_owned(), "string".into())]
                .into_iter()
                .collect(),
        )),
        "bool" => Ok(JsonValue::Object(
            vec![("type".to_owned(), "boolean".into())]
                .into_iter()
                .collect(),
        )),
        _ => Err(SchemaError("unknown type")),
    }
}

fn build_schema_for_struct(
    fields: &Vec<StructFieldDescription>,
    abi: &ContractAbi,
) -> Result<JsonValue, SchemaError> {
    let mut properties = Vec::new();

    for field in fields {
        let field_type = build_schema_for_type(&field.field_type.abi, abi)?;
        properties.push((field.name.clone(), field_type));
    }

    Ok(JsonValue::Object(
        vec![
            ("type".to_owned(), "object".into()),
            (
                "required".to_owned(),
                JsonValue::Array(fields.iter().map(|x| x.name.clone().into()).collect()),
            ),
            ("additionalProperties".to_owned(), false.into()),
            (
                "properties".to_owned(),
                JsonValue::Object(properties.into_iter().collect()),
            ),
        ]
        .into_iter()
        .collect(),
    ))
}

fn wrap_in_single_key_object(key: &str, value: JsonValue) -> JsonValue {
    JsonValue::Object(
        vec![
            ("type".to_owned(), "object".into()),
            (
                "required".to_owned(),
                JsonValue::Array(vec![key.to_owned().into()]),
            ),
            ("additionalProperties".to_owned(), false.into()),
            (
                "properties".to_owned(),
                JsonValue::Object(vec![(key.to_owned(), value)].into_iter().collect()),
            ),
        ]
        .into_iter()
        .collect(),
    )
}

fn build_schema_for_enum(
    variants: &[EnumVariantDescription],
    abi: &ContractAbi,
) -> Result<JsonValue, SchemaError> {
    let variants_simple = variants
        .iter()
        .filter(|x| x.is_empty_variant())
        .collect::<Vec<_>>();
    let variant_simple_values = JsonValue::Object(
        vec![(
            "enum".to_owned(),
            JsonValue::Array(
                variants_simple
                    .iter()
                    .map(|x| x.name.clone().into())
                    .collect(),
            ),
        )]
        .into_iter()
        .collect(),
    );

    let variants_complex = variants
        .iter()
        .filter(|x| !x.is_empty_variant())
        .collect::<Vec<_>>();
    let mut variant_complex_values = Vec::new();

    for variant in variants_complex.iter() {
        if variant.is_tuple_variant() && variant.fields.len() == 1 {
            let type_name = variant.fields[0].field_type.abi.as_str();

            variant_complex_values.push(wrap_in_single_key_object(
                &variant.name,
                build_schema_for_type(type_name, abi)?,
            ));

            continue;
        }

        if variant.is_tuple_variant() {
            let mut ordered_fields = variant.fields.clone();
            ordered_fields.sort_by_key(|e| e.name.parse::<i32>().unwrap());

            let mut values = Vec::new();
            for field in ordered_fields {
                values.push(build_schema_for_type(&field.field_type.abi, abi)?);
            }

            variant_complex_values.push(wrap_in_single_key_object(
                &variant.name,
                JsonValue::Object(
                    vec![
                        ("type".to_owned(), "array".into()),
                        ("additionalItems".to_owned(), false.into()),
                        ("items".to_owned(), JsonValue::Array(values)),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ));

            continue;
        }

        let mut properties = Vec::new();

        for field in variant.fields.iter() {
            let field_type = build_schema_for_type(&field.field_type.abi, abi)?;
            properties.push((field.name.clone(), field_type));
        }

        variant_complex_values.push(wrap_in_single_key_object(
            &variant.name,
            JsonValue::Object(
                vec![
                    ("type".to_owned(), "object".into()),
                    (
                        "required".to_owned(),
                        JsonValue::Array(
                            variant
                                .fields
                                .iter()
                                .map(|x| x.name.clone().into())
                                .collect(),
                        ),
                    ),
                    ("additionalProperties".to_owned(), false.into()),
                    (
                        "properties".to_owned(),
                        JsonValue::Object(properties.into_iter().collect()),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
        ));

        continue;
    }

    if !variants_simple.is_empty() {
        variant_complex_values.push(variant_simple_values);
    }

    if variants_complex.len() == 1 {
        return Ok(variant_complex_values[0].clone());
    }

    Ok(JsonValue::Object(
        vec![("oneOf".to_owned(), JsonValue::Array(variant_complex_values))]
            .into_iter()
            .collect(),
    ))
}

#[derive(Debug)]
pub struct SchemaError(&'static str);

impl Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for SchemaError {}

#[cfg(test)]
mod tests {
    use super::*;
    use multiversx_sc_scenario::multiversx_sc::abi::*;
    use serde_json::json;

    #[test]
    fn test_build_schema_for_single_value_integer() {
        let schema = build_schema_for_single_value("u64").unwrap();
        assert_eq!(schema, json!({"type": "integer"}));
    }

    #[test]
    fn test_build_schema_for_single_value_string() {
        let schema = build_schema_for_single_value("string").unwrap();
        assert_eq!(schema, json!({"type": "string"}));
    }

    #[test]
    fn test_build_schema_for_single_value_bool() {
        let schema = build_schema_for_single_value("bool").unwrap();
        assert_eq!(schema, json!({"type": "boolean"}));
    }

    #[test]
    fn test_build_schema_for_single_value_managed_buffer() {
        let schema = build_schema_for_single_value("ManagedBuffer").unwrap();
        assert_eq!(
            schema,
            json!({
                "type": "array",
                "items": {"type": "integer"}
            })
        );
    }

    #[test]
    fn test_build_schema_for_single_value_unknown() {
        let err = build_schema_for_single_value("UnknownType");
        assert!(err.is_err());
    }

    #[test]
    fn test_build_schema_for_struct() {
        let fields = vec![
            StructFieldDescription {
                name: "field1".to_string(),
                field_type: TypeNames::from_abi("u64".to_string()),
                docs: vec![],
            },
            StructFieldDescription {
                name: "field2".to_string(),
                field_type: TypeNames::from_abi("string".to_string()),
                docs: vec![],
            },
        ];
        let abi = ContractAbi::default();
        let schema = build_schema_for_struct(&fields, &abi).unwrap();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["required"], json!(["field1", "field2"]));
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(schema["properties"]["field1"], json!({"type": "integer"}));
        assert_eq!(schema["properties"]["field2"], json!({"type": "string"}));
    }

    #[test]
    fn test_build_schema_for_enum_simple() {
        let variants = vec![
            EnumVariantDescription {
                name: "A".to_string(),
                fields: vec![],
                docs: vec![],
                discriminant: 0,
            },
            EnumVariantDescription {
                name: "B".to_string(),
                fields: vec![],
                docs: vec![],
                discriminant: 1,
            },
        ];
        let abi = ContractAbi::default();
        let schema = build_schema_for_enum(&variants, &abi).unwrap();
        // Should contain an enum with values A and B
        let one_of = &schema["oneOf"][0]["enum"];
        assert_eq!(one_of, &json!(["A", "B"]));
    }

    #[test]
    fn test_build_schema_for_enum_tuple_variant() {
        let variants = vec![EnumVariantDescription {
            name: "Tuple".to_string(),
            fields: vec![StructFieldDescription {
                name: "0".to_string(),
                field_type: TypeNames::from_abi("u64".to_string()),
                docs: vec![],
            }],
            docs: vec![],
            discriminant: 0,
        }];
        let abi = ContractAbi::default();
        let schema = build_schema_for_enum(&variants, &abi).unwrap();
        // Should be a single-key object with an integer
        assert_eq!(schema["properties"]["Tuple"]["type"], "integer");
    }
}
