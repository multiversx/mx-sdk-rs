use std::borrow::Cow;

const TX_FUNC_NAME_UTF8_ERROR: &str = "error converting function name to utf-8";

/// Contains a SC function name (endpoint, "init", etc.)
///
/// Can be initialized statically and we can make constants out of it.
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct TxFunctionName(Cow<'static, str>);

impl From<String> for TxFunctionName {
    fn from(value: String) -> Self {
        TxFunctionName(value.into())
    }
}

impl From<&str> for TxFunctionName {
    fn from(value: &str) -> Self {
        TxFunctionName(String::from(value).into())
    }
}

impl From<Vec<u8>> for TxFunctionName {
    fn from(value: Vec<u8>) -> Self {
        TxFunctionName(
            String::from_utf8(value)
                .expect(TX_FUNC_NAME_UTF8_ERROR)
                .into(),
        )
    }
}

impl From<&[u8]> for TxFunctionName {
    fn from(value: &[u8]) -> Self {
        value.to_vec().into()
    }
}

impl From<&Vec<u8>> for TxFunctionName {
    fn from(value: &Vec<u8>) -> Self {
        value.clone().into()
    }
}

impl TxFunctionName {
    pub const fn from_static(name: &'static str) -> Self {
        TxFunctionName(Cow::Borrowed(name))
    }

    /// No SC transaction.
    pub const EMPTY: TxFunctionName = TxFunctionName::from_static("");

    /// The constructor name of any smart contract.
    pub const INIT: TxFunctionName = TxFunctionName::from_static("init");

    /// The the legacy async central callback name of any smart contract.
    pub const CALLBACK: TxFunctionName = TxFunctionName::from_static("callBack");

    /// Not a real function name (in fact it is an illegal name), just a flag to mark whitebox calls.
    pub const WHITEBOX_CALL: TxFunctionName = TxFunctionName::from_static("<whitebox-call>");

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_string(self) -> String {
        self.0.into_owned()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.into_string().into_bytes()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for TxFunctionName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
