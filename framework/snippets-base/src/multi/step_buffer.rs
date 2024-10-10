use multiversx_sc_scenario::scenario_model::{ScCallStep, ScDeployStep};

use super::InteractorStepRef;

#[derive(Default)]
pub struct StepBuffer<'a> {
    pub refs: Vec<InteractorStepRef<'a>>,
}

impl<'a> StepBuffer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_sc_call<'b, S>(&'a mut self, step: &'b mut S)
    where
        'b: 'a,
        S: AsMut<ScCallStep>,
    {
        self.refs.push(InteractorStepRef::ScCall(step.as_mut()));
    }

    pub fn add_sc_call_vec<'b, S>(&'a mut self, steps: &'b mut Vec<S>)
    where
        'b: 'a,
        S: AsMut<ScCallStep>,
    {
        for step in steps {
            self.refs.push(InteractorStepRef::ScCall(step.as_mut()));
        }
    }

    pub fn from_sc_call_vec<'b, S>(steps: &'b mut Vec<S>) -> Self
    where
        'b: 'a,
        S: AsMut<ScCallStep>,
    {
        let mut buffer = Self::default();
        for step in steps {
            buffer.refs.push(InteractorStepRef::ScCall(step.as_mut()));
        }
        buffer
    }

    pub fn from_sc_deploy_vec<'b, S>(steps: &'b mut Vec<S>) -> Self
    where
        'b: 'a,
        S: AsMut<ScDeployStep>,
    {
        let mut buffer = Self::default();
        for step in steps {
            buffer.refs.push(InteractorStepRef::ScDeploy(step.as_mut()));
        }
        buffer
    }
}
