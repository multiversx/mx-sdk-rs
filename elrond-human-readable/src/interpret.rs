use std::{error::Error, fmt::Display};

use crate::elrond_wasm::abi::{TypeContents, TypeDescription};
use elrond_wasm_debug::{abi_json::ContractAbiJson, num_bigint::BigUint};

use crate::{AnyValue, SingleValue::UnsignedNumber};

pub fn interpret_value_according_to_abi(
    input: &str,
    type_name: &str,
    contract_abi: &ContractAbiJson, // TODO: will need to convert to high-level ContractAbi first, this is just a prototype
) -> Result<AnyValue, Box<dyn Error>> {
    let type_description = if let Some(type_description_json) = contract_abi.types.get(type_name) {
        type_description_json.to_type_description(type_name)
    } else {
        TypeDescription {
            docs: &[],
            name: type_name.to_string(),
            contents: TypeContents::NotSpecified,
        }
    };
    interpret_any_value(input, &type_description)
}

pub fn interpret_any_value(
    input: &str,
    type_description: &TypeDescription,
) -> Result<AnyValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => interpret_single_value(input, type_description.name.as_str()),
        TypeContents::Enum(_) => todo!(),
        TypeContents::Struct(_) => todo!(),
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
