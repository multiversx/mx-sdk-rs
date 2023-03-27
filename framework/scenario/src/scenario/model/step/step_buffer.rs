use super::ScCallStep;

#[derive(Default)]
pub struct ScCallStepBuffer<'a> {
    pub refs: Vec<&'a mut ScCallStep>,
}

impl<'a> ScCallStepBuffer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<'b, S>(&'a mut self, step: &'b mut S)
    where
        'b: 'a,
        S: AsMut<ScCallStep>,
    {
        self.refs.push(step.as_mut());
    }

    pub fn add_vec<'b, S>(&'a mut self, steps: &'b mut Vec<S>)
    where
        'b: 'a,
        S: AsMut<ScCallStep>,
    {
        for step in steps {
            self.refs.push(step.as_mut());
        }
    }

    pub fn from_vec<'b, S>(steps: &'b mut Vec<S>) -> Self
    where
        'b: 'a,
        S: AsMut<ScCallStep>,
    {
        let mut buffer = Self::default();
        for step in steps {
            buffer.refs.push(step.as_mut());
        }
        buffer
    }

    pub fn as_ref_vec(&'a self) -> Vec<&'a ScCallStep> {
        self.refs.iter().map(|r| &**r).collect()
    }
}
