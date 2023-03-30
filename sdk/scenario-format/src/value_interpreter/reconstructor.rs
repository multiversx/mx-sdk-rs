use num_bigint::BigUint;

use crate::{
    reconstruct_trait::ReconstructorContext,
    serde_raw::ValueSubTree,
    value_interpreter::functions::{
        SC_ADDRESS_NUM_LEADING_ZEROS, SC_ADDRESS_RESERVED_PREFIX_LENGTH,
    },
};
pub enum ExprReconstructorHint {
    // NoHint indicates that the type if not known
    NoHint,

    // NumberHint hints that value should be a number
    UnsignedNumberHint,

    // AddressHint hints that value should be an address
    AddressHint,

    // StrHint hints that value should be a string expression, e.g. a username, "str:..."
    StrHint,

    // CodeHint hints that value should be a smart contract code, normally loaded from a file
    CodeHint,
}

const MAX_BYTES_INTERPRETED_AS_NUMBER: usize = 15;

const SC_ADDRESS_LENGTH: usize = 32;
const SC_CODE_LENGTH: usize = 20;

pub fn reconstruct(
    value: &[u8],
    hint: &ExprReconstructorHint,
    _context: &ReconstructorContext,
) -> ValueSubTree {
    let str: String = match hint {
        ExprReconstructorHint::UnsignedNumberHint => BigUint::from_bytes_be(value).to_string(),
        ExprReconstructorHint::StrHint => format!("str:{}", String::from_utf8_lossy(value)),
        ExprReconstructorHint::AddressHint => address_pretty(value),
        ExprReconstructorHint::CodeHint => code_pretty(value),
        _ => unknown_byte_array_pretty(value),
    };
    ValueSubTree::Str(str)
}

pub fn reconstruct_from_biguint(value: BigUint, context: &ReconstructorContext) -> ValueSubTree {
    reconstruct(
        &value.to_bytes_be(),
        &ExprReconstructorHint::UnsignedNumberHint,
        context,
    )
}

pub fn reconstruct_from_u64(value: u64, context: &ReconstructorContext) -> ValueSubTree {
    reconstruct(
        &BigUint::from(value).to_bytes_be(),
        &ExprReconstructorHint::UnsignedNumberHint,
        context,
    )
}

pub fn reconstruction_list(
    values: &[&[u8]],
    hint: &ExprReconstructorHint,
    context: &ReconstructorContext,
) -> ValueSubTree {
    let mut strings: Vec<ValueSubTree> = Vec::new();
    for value in values.iter() {
        strings.push(reconstruct(value, hint, context));
    }
    ValueSubTree::List(strings)
}

fn unknown_byte_array_pretty(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    // fully interpret as string
    if can_interpret_as_string(bytes) {
        return format!(
            "0x{} (str:{})",
            hex::encode(bytes),
            String::from_utf8_lossy(bytes)
        );
    }

    // interpret as number
    if bytes.len() < MAX_BYTES_INTERPRETED_AS_NUMBER {
        let as_uint = BigUint::from_bytes_be(bytes).to_string();
        return format!("0x{} ({})", hex::encode(bytes), as_uint);
    }

    // default interpret as string with escaped bytes
    format!(
        "0x{} (str:{:?})",
        hex::encode(bytes),
        String::from_utf8_lossy(bytes).to_string(),
    )
}

fn address_pretty(value: &[u8]) -> String {
    if value.len() != 32 {
        return unknown_byte_array_pretty(value);
    }

    // smart contract address
    if value[..SC_ADDRESS_NUM_LEADING_ZEROS] == [0; 8] {
        if value[SC_ADDRESS_LENGTH - 1] == b'_' {
            let address_str =
                String::from_utf8_lossy(&value[SC_ADDRESS_RESERVED_PREFIX_LENGTH..]).to_string();
            return format!("sc:{}", address_str.trim_end_matches('_').to_owned());
        } else {
            // last byte is the shard id and is explicit

            let address_str = String::from_utf8_lossy(
                &value[SC_ADDRESS_RESERVED_PREFIX_LENGTH..SC_ADDRESS_LENGTH - 1],
            )
            .to_string();
            let shard_id = value[SC_ADDRESS_LENGTH - 1];
            return format!(
                "sc:{}#{:x}",
                address_str.trim_end_matches('_').to_owned(),
                shard_id
            );
        }
    }

    // regular addresses
    if value[SC_ADDRESS_LENGTH - 1] == b'_' {
        let address_str = String::from_utf8_lossy(value).to_string();
        format!("address:{}", address_str.trim_end_matches('_').to_owned())
    } else {
        let mut address_str = String::from_utf8_lossy(&value[..SC_ADDRESS_LENGTH - 1]).to_string();
        address_str = address_str.trim_end_matches('_').to_owned();
        let shard_id = value[SC_ADDRESS_LENGTH - 1];
        let address_expr = format!("address:{address_str}#{shard_id:02x}");
        if !can_interpret_as_string(&[value[SC_ADDRESS_LENGTH - 1]]) {
            return format!("0x{} ({})", hex::encode(value), address_expr);
        }
        address_expr
    }
}

fn can_interpret_as_string(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }
    return !bytes.iter().any(|&b| !(32..=126).contains(&b));
}

fn code_pretty(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let encoded = hex::encode(bytes);

    if encoded.len() > SC_CODE_LENGTH {
        return format!("0x{}...", &encoded[..SC_CODE_LENGTH]);
    }

    format!("0x{encoded}")
}
