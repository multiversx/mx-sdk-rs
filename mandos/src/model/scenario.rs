use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ScenarioRaw,
};

use super::Step;

#[derive(Debug)]
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
