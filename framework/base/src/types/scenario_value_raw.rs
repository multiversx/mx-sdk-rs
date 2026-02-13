use crate::types::{
    BigUint, ManagedAddress, ManagedBuffer,
    interaction::{AnnotatedValue, TxEnv},
};
use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    codec::{NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput},
};
use alloc::string::String;
use multiversx_sc_codec::EncodeError;

pub type ScenarioValueRawStr = &'static str;

/// A placeholder type used in scenario-generated blackbox tests.
/// It mimics the structure of ValueSubTree from the scenario format,
/// but is designed to be used in generated code with static string literals.
///
/// This type implements TypeAbiFrom for all types, allowing it to be used
/// as a placeholder anywhere in the contract interface.
#[derive(Clone, Debug)]
pub enum ScenarioValueRaw {
    Str(ScenarioValueRawStr),
    List(&'static [ScenarioValueRaw]),
    Map(&'static [(&'static str, ScenarioValueRaw)]),
}

impl ScenarioValueRaw {
    /// Creates a string placeholder value.
    pub const fn str(s: ScenarioValueRawStr) -> Self {
        ScenarioValueRaw::Str(s)
    }

    /// Creates a list placeholder value.
    pub const fn list(items: &'static [ScenarioValueRaw]) -> Self {
        ScenarioValueRaw::List(items)
    }

    /// Creates a map placeholder value.
    pub const fn map(items: &'static [(&'static str, ScenarioValueRaw)]) -> Self {
        ScenarioValueRaw::Map(items)
    }

    /// Concatenates all contained values into a String.
    /// Useful for debugging and display purposes.
    pub fn to_concatenated_string(&self) -> String {
        let mut accumulator = String::new();
        self.append_to_concatenated_string(&mut accumulator);
        accumulator
    }

    fn append_to_concatenated_string(&self, accumulator: &mut String) {
        match self {
            ScenarioValueRaw::Str(s) => accumulator.push_str(s),
            ScenarioValueRaw::List(l) => {
                for item in l.iter() {
                    if !accumulator.is_empty() {
                        accumulator.push('|');
                    }
                    item.append_to_concatenated_string(accumulator);
                }
            }
            ScenarioValueRaw::Map(m) => {
                for (_, value) in m.iter() {
                    if !accumulator.is_empty() {
                        accumulator.push('|');
                    }
                    value.append_to_concatenated_string(accumulator);
                }
            }
        }
    }
}

// Implement TopEncode - this will panic if actually used
// Placeholders should only be used in scenario tests where they're not actually encoded
impl TopEncode for ScenarioValueRaw {
    fn top_encode_or_handle_err<O, H>(&self, _output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: crate::codec::EncodeErrorHandler,
    {
        Err(h.handle_error(EncodeError::from(
            "ScenarioValuePlaceholder cannot be encoded - it's only a placeholder for testing",
        )))
    }
}

// Implement NestedEncode - this will panic if actually used
impl NestedEncode for ScenarioValueRaw {
    fn dep_encode_or_handle_err<O, H>(&self, _dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: crate::codec::EncodeErrorHandler,
    {
        Err(h.handle_error(EncodeError::from(
            "ScenarioValuePlaceholder cannot be encoded - it's only a placeholder for testing",
        )))
    }
}

// Implement TypeAbi
impl TypeAbi for ScenarioValueRaw {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "ScenarioPlaceholder".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_accumulator: &mut TDC) {
        // No description needed for placeholder
    }
}

// Blanket implementation: ScenarioValueRaw can be used as a placeholder for ANY type
// This allows it to be used in place of any expected type in contract calls
impl<T> TypeAbiFrom<ScenarioValueRaw> for T {}

// Implementation for u64
impl<Env> AnnotatedValue<Env, u64> for ScenarioValueRaw
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(self.to_concatenated_string().as_bytes())
    }

    fn to_value(&self, _env: &Env) -> u64 {
        // Parse the placeholder value - this might fail, but that's expected for placeholders
        self.to_concatenated_string()
            .trim_start_matches("0x")
            .parse()
            .unwrap_or(0)
    }
}

// Implementation for BigUint
impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for ScenarioValueRaw
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(self.to_concatenated_string().as_bytes())
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        // Placeholder - actual value doesn't matter for code generation
        BigUint::zero()
    }
}

// Implementation for ManagedAddress
impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ScenarioValueRaw
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(self.to_concatenated_string().as_bytes())
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        // Placeholder - actual value doesn't matter for code generation
        ManagedAddress::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::test_util::check_top_encode;
    use alloc::string::String;

    #[test]
    fn test_encode_str() {
        let value = ScenarioValueRaw::str("hello");
        let encoded = check_top_encode(&value);
        let expected = check_top_encode(&String::from("hello"));
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_list() {
        const LIST_ITEMS: &[ScenarioValueRaw] =
            &[ScenarioValueRaw::Str("a"), ScenarioValueRaw::Str("b")];
        let value = ScenarioValueRaw::list(LIST_ITEMS);
        let encoded = check_top_encode(&value);
        // Should encode as "a|b"
        let expected = check_top_encode(&String::from("a|b"));
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_map() {
        const MAP_ITEMS: &[(&str, ScenarioValueRaw)] = &[
            ("key1", ScenarioValueRaw::Str("value1")),
            ("key2", ScenarioValueRaw::Str("value2")),
        ];
        let value = ScenarioValueRaw::map(MAP_ITEMS);
        let encoded = check_top_encode(&value);
        // Should encode as "value1|value2"
        let expected = check_top_encode(&String::from("value1|value2"));
        assert_eq!(encoded, expected);
    }
}
