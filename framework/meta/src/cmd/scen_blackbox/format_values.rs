use multiversx_sc::abi::TypeNames;
use multiversx_sc::chain_core::EGLD_000000_TOKEN_IDENTIFIER;
use multiversx_sc_scenario::scenario::model::{AddressValue, BytesKey, BytesValue, CheckValue};
use multiversx_sc_scenario::scenario_format::serde_raw::ValueSubTree;

use super::{num_format, parse_abi::parse_abi_type, test_generator::TestGenerator};

/// A wrapper around a slice of `BytesValue` for sequential consumption during argument formatting.
pub struct BytesValueMultiInput<'a>(pub &'a [BytesValue]);

impl<'a> BytesValueMultiInput<'a> {
    /// Consumes and returns the next item.
    pub fn next_item(&mut self) -> &'a BytesValue {
        let first = self
            .0
            .first()
            .expect("Expected more arguments for multi input");
        self.0 = &self.0[1..];
        first
    }

    /// Returns `true` if there are no remaining items.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl TestGenerator {
    /// Formats a list of arguments using ABI input type info into a single comma-separated string.
    ///
    /// Creates a `BytesValueMultiInput` from `args` and iterates over ABI inputs, passing
    /// the multi-input by mutable reference to `format_arg_value`. Each ABI input may consume
    /// one or more raw arguments depending on its type (e.g. `variadic<T>` consumes all
    /// remaining, `multi<A,B>` consumes two, scalar types consume one).
    ///
    /// Any raw arguments that remain after all ABI inputs have been processed (i.e. those
    /// for which no type info is available) are appended as `/* <raw> */` inline comments,
    /// also separated by commas.
    pub(super) fn format_inputs(
        &mut self,
        args: &[BytesValue],
        inputs: Option<&[multiversx_sc::abi::InputAbi]>,
    ) -> String {
        let mut input = BytesValueMultiInput(args);
        let mut result = String::new();

        let mut first = true;
        if let Some(inputs) = inputs {
            for input_abi in inputs {
                if !first {
                    result.push_str(", ");
                }
                first = false;
                result.push_str(&self.format_arg_value(&input_abi.type_names, &mut input));
            }
        }

        // Any remaining args without ABI info — emit as comments
        while !input.is_empty() {
            let arg = input.next_item();

            result.push_str(&format!("/* {} */", arg.original.to_concatenated_string()));
        }

        result
    }

    /// Formats expected output values using ABI type information into a single string.
    ///
    /// Similar to `format_inputs` but uses `OutputAbi` instead of `InputAbi`.
    /// Extracts `BytesValue`s from the `CheckValue` wrappers (Star values must already be
    /// filtered by the caller), builds a `BytesValueMultiInput`, then iterates over ABI
    /// outputs letting each output type consume as many raw values as it needs.
    ///
    /// If there are multiple output parts the result is wrapped in `MultiValueN::new(...)`.
    /// Any remaining raw values without ABI info are appended as `/* <raw> */` comments.
    pub(super) fn format_out_values(
        &mut self,
        out_values: &[CheckValue<BytesValue>],
        endpoint_name: &str,
    ) -> String {
        let outputs = self.abi.find_endpoint_outputs(endpoint_name);

        // Extract BytesValues – Star values are guaranteed to be pre-filtered.
        let bv_vec: Vec<BytesValue> = out_values
            .iter()
            .map(|v| match v {
                CheckValue::Equal(bv) => bv.clone(),
                CheckValue::Star => {
                    unreachable!("Star values should be filtered before calling format_out_values")
                }
            })
            .collect();

        let mut input = BytesValueMultiInput(&bv_vec);
        let mut parts = Vec::new();

        if let Some(outputs) = &outputs {
            for output in outputs.iter() {
                if input.is_empty() {
                    break;
                }
                parts.push(self.format_arg_value(&output.type_names, &mut input));
            }
        }

        // Any remaining values without ABI info — emit as comments
        while !input.is_empty() {
            let arg = input.next_item();
            parts.push(format!("/* {} */", arg.original.to_concatenated_string()));
        }

        let n = parts.len();
        if n == 1 {
            parts.into_iter().next().unwrap_or_default()
        } else {
            format!("MultiValue{}::new({})", n, parts.join(", "))
        }
    }

    /// Formats an argument value based on ABI type info, consuming items from `input`.
    ///
    /// Calls `parse_abi_type` to determine the structural kind of the type, then dispatches:
    /// - `variadic<T>`: consumes all remaining items and wraps in `MultiValueVec::from(vec![...])`.
    /// - `optional<T>` / `OptionalValue<T>`: consumes one item if available.
    /// - `multi<A,B,...>`: consumes one item per field type and wraps in `MultiValueN::new(...)`.
    /// - `array<N><u8>`: consumes one item and formats as a byte-array constant.
    /// - Everything else (scalar types): consumes exactly one item and formats it using
    ///   `specific_or_abi()` for fine-grained type matching.
    pub(super) fn format_arg_value(
        &mut self,
        type_names: &TypeNames,
        input: &mut BytesValueMultiInput,
    ) -> String {
        let (base, type_args) = parse_abi_type(type_names.specific_or_abi());

        match base.as_str() {
            "variadic" => {
                let inner = type_args[0].clone();
                let inner_type_names = TypeNames::from_abi(inner);
                let mut items = Vec::new();
                while !input.is_empty() {
                    items.push(self.format_arg_value(&inner_type_names, input));
                }

                format!("MultiValueVec::from(vec![{}])", items.join(", "))
            }

            "optional" => {
                if input.is_empty() {
                    "IgnoreValue".to_string()
                } else {
                    let inner = type_args[0].clone();
                    let inner_type_names = TypeNames::from_abi(inner);
                    let inner_formatted = self.format_arg_value(&inner_type_names, input);
                    format!("OptionalValue::Some({inner_formatted})")
                }
            }

            "multi" => {
                let n = type_args.len();
                let fields: Vec<String> = type_args
                    .into_iter()
                    .map(|t| {
                        let tn = TypeNames::from_abi(t);
                        self.format_arg_value(&tn, input)
                    })
                    .collect();
                format!("MultiValue{}::new({})", n, fields.join(", "))
            }

            "ignore" => {
                if !input.is_empty() {
                    let _ = input.next_item();
                }
                "IgnoreValue".to_string()
            }

            _ => {
                // Scalar type: consume exactly one BytesValue.
                let arg = input.next_item();
                match base.as_str() {
                    "bool" => {
                        let is_true = arg.value.len() == 1 && arg.value[0] == 1;
                        if is_true {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        }
                    }
                    "u8" | "u16" | "u32" | "u64" | "usize" | "BigUint" => {
                        num_format::format_unsigned(&arg.value, &type_names.abi)
                    }
                    "i8" | "i16" | "i32" | "i64" | "isize" | "BigInt" => {
                        num_format::format_signed(&arg.value, &type_names.abi)
                    }
                    "NonZeroBigUint" => {
                        let inner = num_format::format_unsigned(&arg.value, "BigUint");
                        format!("NonZeroBigUint::try_from({inner}).unwrap()")
                    }
                    "TokenIdentifier" | "EgldOrEsdtTokenIdentifier" | "TokenId" => {
                        self.format_token_id_value(arg)
                    }
                    "H256" if arg.value.len() == 32 => self.format_h256(arg),
                    "TimestampMillis" | "TimestampSeconds" | "DurationMillis"
                    | "DurationSeconds" => {
                        let inner = num_format::format_unsigned(&arg.value, "u64");
                        format!("{}::new({})", base, inner)
                    }
                    "array" => {
                        // e.g. array32<u8> → type_args = ["32", "u8"]
                        let size = type_args
                            .first()
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or(0);
                        if type_args[1] == "u8" && size > 0 && arg.value.len() == size {
                            self.format_byte_array(arg, size)
                        } else {
                            Self::format_unknown_value(&arg.original)
                        }
                    }
                    // TODO: add more type cases here
                    _ => Self::format_unknown_value(&arg.original),
                }
            }
        }
    }

    /// Formats a BigUint value for use as a payment amount.
    pub(super) fn format_biguint_value(
        value: &multiversx_sc_scenario::num_bigint::BigUint,
    ) -> String {
        let bytes = value.to_bytes_be();
        num_format::format_unsigned(&bytes, "BigUint")
    }

    /// Formats a 32-byte H256 value as a named constant.
    /// Generates a `const H256_N: H256 = H256::from_hex("...");` declaration.
    pub(super) fn format_h256(&mut self, arg: &BytesValue) -> String {
        let hex_str = hex::encode(&arg.value);
        self.consts.get_or_create_h256(&hex_str)
    }

    /// Formats a fixed-size byte array as a named constant using `hex!()`.
    /// Generates `const HEX_{size}_{N}: [u8; {size}] = hex!("...");`
    /// and returns `&HEX_{size}_{N}`.
    pub(super) fn format_byte_array(&mut self, arg: &BytesValue, size: usize) -> String {
        let hex_str = hex::encode(&arg.value);
        self.consts.get_or_create_byte_array(&hex_str, size)
    }

    // -------------------------------------------------------------------------
    // Address / value formatting (shared across all generators)
    // -------------------------------------------------------------------------

    pub(super) fn format_address(&mut self, addr: &str) -> String {
        // Remove quotes if present
        let clean = addr.trim_matches('"');

        // Handle address: and sc: prefixes
        if let Some(name) = clean.strip_prefix("address:") {
            self.consts.get_or_create_address(addr, name)
        } else if let Some(name) = clean.strip_prefix("sc:") {
            self.consts.get_or_create_sc_address(addr, name)
        } else if clean.starts_with("0x")
            || clean.starts_with("0X")
            || (clean.len() == 64 && clean.chars().all(|c| c.is_ascii_hexdigit()))
        {
            self.consts.get_or_create_hex_address(clean)
        } else {
            // Raw address - wrap in ScenarioValueRaw
            format!("ScenarioValueRaw::new(\"{}\")", clean)
        }
    }

    pub(super) fn format_address_value(&mut self, value: &AddressValue) -> String {
        match &value.original {
            ValueSubTree::Str(s) => self.format_address(s),
            _ => {
                // Fallback for non-string addresses
                Self::format_unknown_value(&value.original)
            }
        }
    }

    pub(super) fn format_unknown_value(value: &ValueSubTree) -> String {
        match value {
            ValueSubTree::Str(s) => {
                format!("ScenarioValueRaw::new(\"{}\")", Self::escape_string(s))
            }
            _ => {
                format!(
                    "ScenarioValueRaw::new({})",
                    Self::format_value_subtree(value)
                )
            }
        }
    }

    fn format_value_subtree(value: &ValueSubTree) -> String {
        match value {
            ValueSubTree::Str(s) => {
                format!(
                    "ValueSubTree::Str(\"{}\".to_string())",
                    Self::escape_string(s)
                )
            }
            ValueSubTree::List(items) => {
                if items.is_empty() {
                    "ValueSubTree::List(vec![])".to_string()
                } else {
                    let formatted_items: Vec<String> =
                        items.iter().map(Self::format_value_subtree).collect();
                    format!("ValueSubTree::List(vec![{}])", formatted_items.join(", "))
                }
            }
            ValueSubTree::Map(map) => {
                if map.is_empty() {
                    "ValueSubTree::Map(BTreeMap::new())".to_string()
                } else {
                    let formatted_entries: Vec<String> = map
                        .iter()
                        .map(|(k, v)| {
                            format!(
                                "(\"{}\".to_string(), {})",
                                Self::escape_string(k),
                                Self::format_value_subtree(v)
                            )
                        })
                        .collect();
                    format!(
                        "ValueSubTree::Map(BTreeMap::from([{}]))",
                        formatted_entries.join(", ")
                    )
                }
            }
        }
    }

    pub(super) fn format_value_as_string(value: &ValueSubTree) -> String {
        match value {
            ValueSubTree::Str(s) => s.clone(),
            ValueSubTree::List(items) => {
                let strs: Vec<String> = items.iter().map(Self::format_value_as_string).collect();
                strs.join("|")
            }
            ValueSubTree::Map(map) => {
                let strs: Vec<String> = map.values().map(Self::format_value_as_string).collect();
                strs.join("|")
            }
        }
    }

    pub(super) fn escape_string(s: &str) -> String {
        s.chars().flat_map(char::escape_default).collect()
    }

    // -------------------------------------------------------------------------
    // Token ID formatting (shared across all generators)
    // -------------------------------------------------------------------------

    /// Formats a token identifier from a BytesValue into a constant reference.
    /// Generates a `TestTokenId` constant if one doesn't already exist.
    pub(super) fn format_token_id_value(&mut self, token_id: &BytesValue) -> String {
        self.format_token_id_str(&String::from_utf8_lossy(&token_id.value))
    }

    /// Formats a token identifier from a BytesKey (used in setState ESDT maps).
    pub(super) fn format_token_id_key(&mut self, key: &BytesKey) -> String {
        self.format_token_id_str(&String::from_utf8_lossy(&key.value))
    }

    /// Core token ID formatting logic, shared by `format_token_id_value` and `format_token_id_key`.
    fn format_token_id_str(&mut self, name: &str) -> String {
        if name == EGLD_000000_TOKEN_IDENTIFIER {
            // Use the built-in constant for EGLD-000000
            "TestTokenId::EGLD_000000".to_string()
        } else {
            self.consts.get_or_create_token_id(name)
        }
    }

    pub(super) fn format_balance_value(
        value: &multiversx_sc_scenario::num_bigint::BigUint,
    ) -> String {
        let bytes = value.to_bytes_be();
        num_format::format_unsigned(&bytes, "BigUint")
    }

    pub(super) fn format_nonce_value(value: u64) -> String {
        let bytes = value.to_be_bytes();
        num_format::format_unsigned(&bytes, "u64")
    }
}
