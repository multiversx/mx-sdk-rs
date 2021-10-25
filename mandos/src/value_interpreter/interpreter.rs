use crate::{interpret_trait::InterpreterContext, serde_raw::ValueSubTree};

use super::{file_loader::load_file, functions::*, parse_num::*, prefixes::*};

pub fn interpret_subtree(vst: &ValueSubTree, context: &InterpreterContext) -> Vec<u8> {
    match vst {
        ValueSubTree::Str(s) => interpret_string(s, context),
        ValueSubTree::List(l) => {
            let mut concat = Vec::<u8>::new();
            for item in l.iter() {
                concat.extend_from_slice(interpret_subtree(item, context).as_slice());
            }
            concat
        },
        ValueSubTree::Map(m) => {
            let mut concat = Vec::<u8>::new();
            for (_, value) in m.iter() {
                concat.extend_from_slice(interpret_subtree(value, context).as_slice());
            }
            concat
        },
    }
}

pub fn interpret_string(s: &str, context: &InterpreterContext) -> Vec<u8> {
    if s.is_empty() {
        return Vec::new();
    }

    // concatenate values of different formats
    let split_parts: Vec<_> = s.split('|').collect();
    if split_parts.len() > 1 {
        let mut result = Vec::<u8>::new();
        for part in split_parts.iter() {
            result.extend_from_slice(interpret_string(part, context).as_slice());
        }
        return result;
    }

    if s == "true" {
        return [1u8].to_vec();
    }

    if s == "false" {
        return Vec::new();
    }

    for str_prefix in STR_PREFIXES.iter() {
        if let Some(stripped) = s.strip_prefix(str_prefix) {
            return stripped.as_bytes().to_vec();
        }
    }

    if let Some(stripped) = s.strip_prefix(ADDR_PREFIX) {
        return address_expression(stripped);
    }

    if let Some(stripped) = s.strip_prefix(SC_ADDR_PREFIX) {
        return sc_address_expression(stripped);
    }

    if let Some(stripped) = s.strip_prefix(FILE_PREFIX) {
        return load_file(stripped, context);
    }

    if let Some(stripped) = s.strip_prefix(KECCAK256_PREFIX) {
        let arg = interpret_string(stripped, context);
        return keccak256(arg.as_slice());
    }

    if let Some(stripped) = s.strip_prefix(NESTED_PREFIX) {
        return parse_nested(stripped, context);
    }

    if let Some(fixed_width) = try_parse_fixed_width(s) {
        return fixed_width;
    }

    parse_num(s)
}

fn parse_nested(s: &str, context: &InterpreterContext) -> Vec<u8> {
    let parsed = interpret_string(s, context);
    let encoded_length = (parsed.len() as u32).to_be_bytes();
    [&encoded_length[..], &parsed[..]].concat()
}
