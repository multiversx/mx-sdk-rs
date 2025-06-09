use std::{error::Error, fmt::Display};

use crate::{
    format::HumanReadableValue,
    multiversx_sc::abi::{TypeContents, TypeDescription},
    SingleValue, StructField, StructValue,
};
use multiversx_sc_scenario::{
    bech32,
    multiversx_sc::abi::{ContractAbi, EnumVariantDescription, StructFieldDescription},
    num_bigint::{BigInt, BigUint},
};

use crate::AnyValue;

pub fn decode_human_readable_value(
    input: &HumanReadableValue,
    type_name: &str,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    let type_description = contract_abi.type_descriptions.find_or_default(type_name);
    decode_any_value(input, &type_description, contract_abi)
}

pub fn decode_any_value(
    input: &HumanReadableValue,
    type_description: &TypeDescription,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => {
            decode_single_value(input, type_description.names.abi.as_str())
        },
        TypeContents::Enum(variants) => decode_enum(input, variants, contract_abi),
        TypeContents::Struct(fields) => decode_struct(input, fields, contract_abi),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

fn decode_single_value(
    input: &HumanReadableValue,
    type_name: &str,
) -> Result<AnyValue, Box<dyn Error>> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => {
            let number_value = input
                .get_value()
                .as_number()
                .ok_or_else(|| Box::new(DecodeError("expected unsigned number value")))?;

            let value = number_value.to_string().parse::<BigUint>()?;
            Ok(AnyValue::SingleValue(SingleValue::UnsignedNumber(value)))
        },
        "BigInt" | "i64" | "i32" | "i16" | "isize" | "i8" => {
            let number_value = input
                .get_value()
                .as_number()
                .ok_or_else(|| Box::new(DecodeError("expected number value")))?;

            let value = number_value.to_string().parse::<BigInt>()?;
            Ok(AnyValue::SingleValue(SingleValue::SignedNumber(value)))
        },
        "ManagedBuffer" => {
            let array_value = input
                .get_value()
                .as_array()
                .ok_or_else(|| Box::new(DecodeError("expected bytes value")))?;

            let mut bytes = vec![0u8; array_value.len()];
            for (i, value) in array_value.iter().enumerate() {
                let number_value = value
                    .as_u64()
                    .ok_or_else(|| Box::new(DecodeError("expected byte value")))?;
                if number_value > 255 {
                    return Err(Box::new(DecodeError("expected byte value")));
                }
                bytes[i] = number_value as u8;
            }

            Ok(AnyValue::SingleValue(SingleValue::Bytes(bytes.into())))
        },
        "string" | "utf-8 string" => {
            let str_value = input
                .get_value()
                .as_str()
                .ok_or_else(|| Box::new(DecodeError("expected string value")))?;

            Ok(AnyValue::SingleValue(SingleValue::String(
                str_value.to_string(),
            )))
        },
        "Address" => {
            let str_value = input
                .get_value()
                .as_str()
                .ok_or_else(|| Box::new(DecodeError("expected string value")))?;

            let address = bech32::try_decode(str_value)
                .map_err(|_| Box::new(DecodeError("failed to parse address")))?;

            Ok(AnyValue::SingleValue(SingleValue::Bytes(
                address.as_bytes().into()
            )))
        },
        "bool" => {
            let bool_value = input
                .get_value()
                .as_bool()
                .ok_or_else(|| Box::new(DecodeError("expected bool value")))?;

            Ok(AnyValue::SingleValue(SingleValue::Bool(bool_value)))
        },
        _ => Err(Box::new(DecodeError("unknown type"))),
    }
}

pub fn decode_struct(
    input: &HumanReadableValue,
    fields: &[StructFieldDescription],
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    let mut field_values: Vec<StructField> = vec![];

    for field in fields.iter() {
        let value = input
            .child(&field.name)
            .ok_or_else(|| Box::new(DecodeError("missing field")))?;
        let value = decode_human_readable_value(&value, &field.field_type.abi, contract_abi)?;
        field_values.push(StructField {
            name: field.name.clone(),
            value,
        });
    }

    Ok(AnyValue::Struct(StructValue(field_values)))
}

pub fn decode_enum(
    input: &HumanReadableValue,
    variants: &[EnumVariantDescription],
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    if input.get_value().is_string() {
        let discriminant_name = input.get_value().as_str().unwrap();
        let variant = variants
            .iter()
            .find(|el| el.name == discriminant_name)
            .ok_or_else(|| Box::new(DecodeError("enum variant not found")))?;

        if !variant.is_empty_variant() {
            return Err(Box::new(DecodeError("enum variant is not a tuple variant")));
        }

        return Ok(AnyValue::Enum(Box::new(crate::EnumVariant {
            discriminant: variant.discriminant,
            value: AnyValue::None,
        })));
    }

    if !input.get_value().is_object() {
        return Err(Box::new(DecodeError(
            "expected object or string value for enum",
        )));
    }

    let obj_value = input.get_value().as_object().unwrap();
    if obj_value.keys().len() != 1 {
        return Err(Box::new(DecodeError(
            "expected object with single key for enum",
        )));
    }

    let discriminant_name = obj_value.keys().next().unwrap().as_str();
    let variant = variants
        .iter()
        .find(|el| el.name == discriminant_name)
        .ok_or_else(|| Box::new(DecodeError("enum variant not found")))?;

    // handle tuple with only one field as a special case (we don't need a wrapper array)
    if variant.is_tuple_variant() && variant.fields.len() == 1 {
        let value = input.child(variant.name.as_str()).unwrap();
        let value =
            decode_human_readable_value(&value, &variant.fields[0].field_type.abi, contract_abi)?;
        return Ok(AnyValue::Enum(Box::new(crate::EnumVariant {
            discriminant: variant.discriminant,
            value,
        })));
    } else if variant.is_tuple_variant() {
        let value = input.child(variant.name.as_str()).unwrap();
        let value = value
            .get_value()
            .as_array()
            .ok_or_else(|| Box::new(DecodeError("expected array for enum tuple variant")))?;

        if value.len() != variant.fields.len() {
            return Err(Box::new(DecodeError(
                "expected array with the same length as the tuple variant fields",
            )));
        }

        let mut field_values: Vec<StructField> = vec![];
        for (i, field) in variant.fields.iter().enumerate() {
            let value = value.get(i).unwrap();
            let value = decode_human_readable_value(
                &(value.to_owned().into()),
                &field.field_type.abi,
                contract_abi,
            )?;
            field_values.push(StructField {
                name: field.name.clone(),
                value,
            });
        }

        return Ok(AnyValue::Enum(Box::new(crate::EnumVariant {
            discriminant: variant.discriminant,
            value: AnyValue::Struct(StructValue(field_values)),
        })));
    }

    // is not empty and is not a tuple so just try to parse a struct from the fields
    let value = input.child(variant.name.as_str()).unwrap();
    let value = decode_struct(&value, &variant.fields, contract_abi)?;

    Ok(AnyValue::Enum(Box::new(crate::EnumVariant {
        discriminant: variant.discriminant,
        value,
    })))
}

#[derive(Debug)]
pub struct DecodeError(&'static str);

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for DecodeError {}
