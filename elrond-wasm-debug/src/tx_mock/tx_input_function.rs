const TX_FUNC_NAME_UTF8_ERROR: &str = "error converting function name to utf-8";

/// Contains a SC function name (endpoint, "init", etc.)
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct TxFunctionName(String);

impl From<String> for TxFunctionName {
    fn from(value: String) -> Self {
        TxFunctionName(value)
    }
}

impl From<&str> for TxFunctionName {
    fn from(value: &str) -> Self {
        TxFunctionName(String::from(value))
    }
}

impl From<Vec<u8>> for TxFunctionName {
    fn from(value: Vec<u8>) -> Self {
        TxFunctionName(String::from_utf8(value).expect(TX_FUNC_NAME_UTF8_ERROR))
    }
}

impl From<&Vec<u8>> for TxFunctionName {
    fn from(value: &Vec<u8>) -> Self {
        TxFunctionName(String::from_utf8(value.clone()).expect(TX_FUNC_NAME_UTF8_ERROR))
    }
}

impl From<&[u8]> for TxFunctionName {
    fn from(value: &[u8]) -> Self {
        TxFunctionName(String::from_utf8(value.to_vec()).expect(TX_FUNC_NAME_UTF8_ERROR))
    }
}

impl TxFunctionName {
    pub fn empty() -> Self {
        Self::default()
    }

    /// The constructor name of any smart contract.
    pub fn init() -> Self {
        Self("init".to_string())
    }

    /// The the legacy async central callback name of any smart contract.
    pub fn callback() -> Self {
        Self("callBack".to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl core::fmt::Display for TxFunctionName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
