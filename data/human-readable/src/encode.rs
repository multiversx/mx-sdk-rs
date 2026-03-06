use std::{error::Error, fmt::Display};

use multiversx_sc_scenario::{
    imports::{Address, Bech32Address},
    multiversx_sc::abi::{
        ContractAbi, EnumVariantDescription, StructFieldDescription, TypeContents, TypeDescription,
    },
};
use serde_json::{Map, Value as JsonValue};

use crate::{AnyValue, SingleValue, format::HumanReadableValue};

pub fn encode_human_readable_value(
    input: &AnyValue,
    type_name: &str,
    contract_abi: &ContractAbi,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    let type_description = contract_abi.type_descriptions.find_or_default(type_name);
    encode_any_value(input, &type_description, contract_abi)
}

pub fn encode_any_value(
    input: &AnyValue,
    type_description: &TypeDescription,
    contract_abi: &ContractAbi,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => {
            encode_single_value(input, type_description.names.abi.as_str())
        }
        TypeContents::Enum(variants) => encode_enum(input, variants, contract_abi),
        TypeContents::Struct(fields) => encode_struct(input, fields, contract_abi),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

fn encode_single_value(
    input: &AnyValue,
    type_name: &str,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::UnsignedNumber(value) = value else {
                return Err(Box::new(EncodeError("expected unsigned number value")));
            };

            // could be biguint, so we convert to string first
            let json_value: JsonValue = serde_json::from_str(&value.to_string())
                .map_err(|_| Box::new(EncodeError("expected number value")))?;

            Ok(json_value.into())
        }
        "BigInt" | "i64" | "i32" | "i16" | "isize" | "i8" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::SignedNumber(value) = value else {
                return Err(Box::new(EncodeError("expected signed number value")));
            };

            // could be bigint, so we convert to string first
            let json_value: JsonValue = serde_json::from_str(&value.to_string())
                .map_err(|_| Box::new(EncodeError("expected number value")))?;

            Ok(json_value.into())
        }
        "ManagedBuffer" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::Bytes(value) = value else {
                return Err(Box::new(EncodeError("expected bytes value")));
            };

            Ok(JsonValue::Array(
                value
                    .iter()
                    .map(|b| JsonValue::Number(b.to_owned().into()))
                    .collect(),
            )
            .into())
        }
        "string" | "utf-8 string" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::String(value) = value else {
                return Err(Box::new(EncodeError("expected string value")));
            };

            Ok(JsonValue::String(value.to_owned()).into())
        }
        "Address" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::Bytes(value) = value else {
                return Err(Box::new(EncodeError("expected bytes value")));
            };

            let bech32_address =
                Bech32Address::encode_address_default_hrp(Address::from_slice(value));
            Ok(JsonValue::String(bech32_address.bech32).into())
        }
        "bool" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::Bool(value) = value else {
                return Err(Box::new(EncodeError("expected bool value")));
            };

            Ok(JsonValue::Bool(value.to_owned()).into())
        }
        _ => {
            println!("unknown type: {}", type_name);
            Err(Box::new(EncodeError("unknown type")))
        }
    }
}

pub fn encode_struct(
    input: &AnyValue,
    fields: &[StructFieldDescription],
    contract_abi: &ContractAbi,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    let AnyValue::Struct(struct_value) = input else {
        return Err(Box::new(EncodeError("expected struct value")));
    };
    let mut struct_fields = struct_value.0.iter();

    let mut field_values: Map<String, JsonValue> = Map::new();

    for field in fields.iter() {
        let value = struct_fields
            .find(|f| f.name == field.name)
            .ok_or_else(|| Box::new(EncodeError("missing field")))?;

        let value = encode_human_readable_value(&value.value, &field.field_type.abi, contract_abi)?;
        field_values.insert(field.name.to_owned(), value.get_value().to_owned());
    }

    Ok(JsonValue::Object(field_values).into())
}

pub fn encode_enum(
    input: &AnyValue,
    variants: &[EnumVariantDescription],
    contract_abi: &ContractAbi,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    let AnyValue::Enum(enum_value) = input else {
        return Err(Box::new(EncodeError("expected enum value")));
    };
    let variant = variants
        .iter()
        .find(|v| v.discriminant == enum_value.discriminant)
        .ok_or_else(|| Box::new(EncodeError("missing variant")))?;

    if variant.is_empty_variant() {
        return Ok(JsonValue::String(variant.name.to_owned()).into());
    }

    if variant.is_tuple_variant() && variant.fields.len() == 1 {
        let value = encode_human_readable_value(
            &enum_value.value,
            &variant.fields[0].field_type.abi,
            contract_abi,
        )?;
        return Ok(JsonValue::Object(
            vec![(variant.name.to_owned(), value.get_value().to_owned())]
                .into_iter()
                .collect(),
        )
        .into());
    }

    if variant.is_tuple_variant() {
        let AnyValue::Struct(variant_fields) = &enum_value.value else {
            return Err(Box::new(EncodeError("expected struct value")));
        };

        let mut field_values: Vec<JsonValue> = vec![];

        for (field, field_type) in variant.fields.iter().zip(variant_fields.0.iter()) {
            let value = encode_human_readable_value(
                &field_type.value,
                &field.field_type.abi,
                contract_abi,
            )?;
            field_values.push(value.get_value().to_owned());
        }

        return Ok(JsonValue::Object(
            vec![(variant.name.to_owned(), JsonValue::Array(field_values))]
                .into_iter()
                .collect(),
        )
        .into());
    }

    let AnyValue::Struct(variant_fields) = &enum_value.value else {
        return Err(Box::new(EncodeError("expected struct value")));
    };

    let mut field_values: Map<String, JsonValue> = Map::new();
    for (field, field_type) in variant.fields.iter().zip(variant_fields.0.iter()) {
        let value =
            encode_human_readable_value(&field_type.value, &field.field_type.abi, contract_abi)?;
        field_values.insert(field.name.to_owned(), value.get_value().to_owned());
    }

    Ok(JsonValue::Object(
        vec![(variant.name.to_owned(), JsonValue::Object(field_values))]
            .into_iter()
            .collect(),
    )
    .into())
}

#[derive(Debug)]
pub struct EncodeError(&'static str);

impl Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for EncodeError {}
