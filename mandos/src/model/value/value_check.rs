use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::{CheckBytesValueRaw, CheckValueListRaw, ValueSubTree},
};

use std::{fmt, fmt::Write};

use super::BytesValue;

#[derive(Debug)]
pub enum CheckValue<T: Default> {
    Star,
    Equal(T),
}

impl<T> CheckValue<T>
where
    T: InterpretableFrom<ValueSubTree> + Default,
{
    pub fn is_star(&self) -> bool {
        matches!(self, CheckValue::Star)
    }
}

impl<T> Default for CheckValue<T>
where
    T: Default,
{
    fn default() -> Self {
        CheckValue::Star
    }
}

impl<T> InterpretableFrom<CheckBytesValueRaw> for CheckValue<T>
where
    T: InterpretableFrom<ValueSubTree> + Default,
{
    fn interpret_from(from: CheckBytesValueRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckBytesValueRaw::Unspecified => CheckValue::Star,
            CheckBytesValueRaw::Star => CheckValue::Star,
            CheckBytesValueRaw::Equal(bytes_value) => {
                CheckValue::Equal(T::interpret_from(bytes_value, context))
            },
        }
    }
}

impl<T: fmt::Display + Default> fmt::Display for CheckValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckValue::Star => write!(f, "*"),
            CheckValue::Equal(eq_value) => eq_value.fmt(f),
        }
    }
}

/// Alias for a list of item checks that can be ignored altogether.
/// Valid values (with different behaviors): `"*"`, `["*"]`, `["1", "*"]`, `["*", "*", "*"]`
pub type CheckValueList = CheckValue<Vec<CheckValue<BytesValue>>>;

impl InterpretableFrom<CheckValueListRaw> for CheckValueList {
    fn interpret_from(from: CheckValueListRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckValueListRaw::Unspecified => CheckValue::Star,
            CheckValueListRaw::Star => CheckValue::Star,
            CheckValueListRaw::CheckList(list_raw) => CheckValue::Equal(
                list_raw
                    .into_iter()
                    .map(|check_raw| CheckValue::<BytesValue>::interpret_from(check_raw, context))
                    .collect(),
            ),
        }
    }
}

impl CheckValueList {
    pub fn pretty_str(&self) -> String {
        match self {
            CheckValue::Star => "*".to_string(),
            CheckValue::Equal(list) => {
                let mut s = String::new();
                s.push('[');
                for (i, check_value) in list.iter().enumerate() {
                    if i > 0 {
                        s.push(',');
                    }
                    write!(s, "{}", check_value).unwrap();
                }
                s.push(']');
                s
            },
        }
    }
}
