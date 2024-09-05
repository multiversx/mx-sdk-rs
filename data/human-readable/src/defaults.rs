use std::{error::Error, fmt::Display};

use multiversx_sc_scenario::{
    multiversx_sc::abi::{
        ContractAbi, EnumVariantDescription, StructFieldDescription, TypeContents, TypeDescription,
    },
    num_bigint::{BigInt, BigUint},
};

use crate::{AnyValue, SingleValue, StructField, StructValue};

pub fn default_value_for_abi_type(
    type_name: &str,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    let type_description =
        if let Some(type_description) = contract_abi.type_descriptions.0.get(type_name) {
            type_description.to_owned()
        } else {
            TypeDescription {
                docs: Vec::new(),
                name: type_name.to_string(),
                contents: TypeContents::NotSpecified,
            }
        };

    default_value_for_any_value(&type_description, contract_abi)
}

pub fn default_value_for_any_value(
    type_description: &TypeDescription,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => {
            default_value_for_single_value(type_description.name.as_str())
        },
        TypeContents::Enum(variants) => default_value_for_enum(&variants, contract_abi),
        TypeContents::Struct(fields) => default_value_for_struct(&fields, contract_abi),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

pub fn default_value_for_single_value(type_name: &str) -> Result<AnyValue, Box<dyn Error>> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => Ok(AnyValue::SingleValue(
            SingleValue::UnsignedNumber(BigUint::default()),
        )),
        "BigInt" | "i64" | "i32" | "i16" | "isize" | "i8" => Ok(AnyValue::SingleValue(
            SingleValue::SignedNumber(BigInt::default()),
        )),
        "ManagedBuffer" => Ok(AnyValue::SingleValue(SingleValue::Bytes(Vec::new().into()))),
        "string" | "utf-8 string" => Ok(AnyValue::SingleValue(SingleValue::String("".to_owned()))),
        "Address" => Ok(AnyValue::SingleValue(SingleValue::Bytes(
            vec![0u8; 32].into(),
        ))),
        "bool" => Ok(AnyValue::SingleValue(SingleValue::Bool(false))),
        _ => Err(Box::new(DefaultValueError("unknown type"))),
    }
}

pub fn default_value_for_struct(
    fields: &Vec<StructFieldDescription>,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    let mut field_values: Vec<StructField> = vec![];

    for field in fields.iter() {
        let value = default_value_for_abi_type(&field.field_type, &contract_abi)?;
        field_values.push(StructField {
            name: field.name.clone(),
            value,
        });
    }

    Ok(AnyValue::Struct(StructValue(field_values)))
}

pub fn default_value_for_enum(
    variants: &Vec<EnumVariantDescription>,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    let variant = variants
        .iter()
        .find(|el| el.discriminant == 0)
        .ok_or_else(|| Box::new(DefaultValueError("enum variant not found")))?;

    if variant.is_empty_variant() {
        return Ok(AnyValue::Enum(Box::new(crate::EnumVariant {
            discriminant: variant.discriminant,
            value: AnyValue::None,
        })));
    }

    // handle tuple with only one field as a special case (we don't need a wrapper array)
    if variant.is_tuple_variant() && variant.fields.len() == 1 {
        let value = default_value_for_abi_type(&variant.fields[0].field_type, contract_abi)?;
        return Ok(AnyValue::Enum(Box::new(crate::EnumVariant {
            discriminant: variant.discriminant,
            value,
        })));
    } else if variant.is_tuple_variant() {
        let mut field_values: Vec<StructField> = vec![];
        for field in variant.fields.iter() {
            let value = default_value_for_abi_type(&field.field_type, &contract_abi)?;
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
    let value = default_value_for_struct(&variant.fields, contract_abi)?;

    Ok(AnyValue::Enum(Box::new(crate::EnumVariant {
        discriminant: variant.discriminant,
        value,
    })))
}

#[derive(Debug)]
pub struct DefaultValueError(&'static str);

impl Display for DefaultValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for DefaultValueError {}
