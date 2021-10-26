use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::CheckEsdtInstancesRaw,
};

use super::CheckEsdtInstance;

#[derive(Debug)]
pub enum CheckEsdtInstances {
    Star,
    Equal(Vec<CheckEsdtInstance>),
}

impl CheckEsdtInstances {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtInstances::Star)
    }

    pub fn contains_nonce(&self, nonce: u64) -> bool {
        match &self {
            CheckEsdtInstances::Equal(eq) => {
                for expected_value in eq.iter() {
                    if expected_value.nonce.value == nonce {
                        return true;
                    }
                }
            },
            CheckEsdtInstances::Star => {},
        }
        false
    }
}

#[allow(clippy::derivable_impls)]
impl Default for CheckEsdtInstances {
    fn default() -> Self {
        CheckEsdtInstances::Equal(Vec::new())
    }
}

impl InterpretableFrom<CheckEsdtInstancesRaw> for CheckEsdtInstances {
    fn interpret_from(from: CheckEsdtInstancesRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtInstancesRaw::Unspecified => CheckEsdtInstances::Star,
            CheckEsdtInstancesRaw::Star => CheckEsdtInstances::Star,
            CheckEsdtInstancesRaw::Equal(m) => CheckEsdtInstances::Equal(
                m.into_iter()
                    .map(|v| CheckEsdtInstance::interpret_from(v, context))
                    .collect(),
            ),
        }
    }
}
