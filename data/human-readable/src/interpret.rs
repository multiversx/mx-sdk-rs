use std::{error::Error, fmt::Display};

use crate::multiversx_sc::abi::{TypeContents, TypeDescription};
use multiversx_sc_meta_lib::abi_json::ContractAbiJson;
use multiversx_sc_scenario::{multiversx_sc::abi::TypeNames, num_bigint::BigUint};

use crate::{AnyValue, SingleValue::UnsignedNumber};

pub fn interpret_value_according_to_abi(
    input: &str,
    type_name: &str,
    contract_abi: &ContractAbiJson, // TODO: will need to convert to high-level ContractAbi first, this is just a prototype
) -> Result<AnyValue, Box<dyn Error>> {
    let type_description = if let Some(type_description_json) = contract_abi.types.get(type_name) {
        type_description_json.to_type_description(TypeNames::abi_only(type_name.to_owned()))
    } else {
        TypeDescription {
            docs: Vec::new(),
            names: TypeNames::abi_only(type_name.to_owned()),
            contents: TypeContents::NotSpecified,
            macro_attributes: Vec::new(),
        }
    };
    interpret_any_value(input, &type_description)
}

pub fn interpret_any_value(
    input: &str,
    type_description: &TypeDescription,
) -> Result<AnyValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => interpret_single_value(input, &type_description.names.abi),
        TypeContents::Enum(_) => todo!(),
        TypeContents::Struct(_) => todo!(),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

fn interpret_single_value(input: &str, type_name: &str) -> Result<AnyValue, Box<dyn Error>> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => {
            let value = input.parse::<BigUint>()?;
            Ok(AnyValue::SingleValue(UnsignedNumber(value)))
        },
        _ => Err(Box::new(InterpretError("unknown type"))),
    }
}

#[derive(Debug)]
pub struct InterpretError(&'static str);

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for InterpretError {}
