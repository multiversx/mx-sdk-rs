use crate::abi::TypeAbi;
use alloc::string::String;
use elrond_codec::multi_types::IgnoreValue;

/// Structure that allows taking a variable number of arguments,
/// but does nothing with them, not even deserialization.
pub type IgnoreVarArgs = IgnoreValue;

impl TypeAbi for IgnoreVarArgs {
    fn type_name() -> String {
        String::from("ignore")
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
