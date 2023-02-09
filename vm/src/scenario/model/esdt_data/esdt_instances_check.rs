use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckEsdtInstancesRaw,
};

use super::CheckEsdtInstance;

#[derive(Debug, Clone)]
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

impl IntoRaw<CheckEsdtInstancesRaw> for CheckEsdtInstances {
    fn into_raw(self) -> CheckEsdtInstancesRaw {
        match self {
            CheckEsdtInstances::Equal(eq) => {
                CheckEsdtInstancesRaw::Equal(eq.into_iter().map(|cei| cei.into_raw()).collect())
            },
            CheckEsdtInstances::Star => CheckEsdtInstancesRaw::Star,
        }
    }
}
