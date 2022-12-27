use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ScenarioRaw,
};

use super::Step;

#[derive(Debug, Default)]
pub struct Scenario {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub check_gas: Option<bool>,
    pub steps: Vec<Step>,
}

impl InterpretableFrom<ScenarioRaw> for Scenario {
    fn interpret_from(from: ScenarioRaw, context: &InterpreterContext) -> Self {
        Scenario {
            name: from.name,
            comment: from.comment,
            check_gas: from.check_gas,
            steps: from
                .steps
                .into_iter()
                .map(|s| Step::interpret_from(s, context))
                .collect(),
        }
    }
}

impl IntoRaw<ScenarioRaw> for Scenario {
    fn into_raw(self) -> ScenarioRaw {
        ScenarioRaw {
            name: self.name,
            comment: self.comment,
            check_gas: self.check_gas,
            gas_schedule: None,
            steps: self.steps.into_iter().map(Step::into_raw).collect(),
        }
    }
}
