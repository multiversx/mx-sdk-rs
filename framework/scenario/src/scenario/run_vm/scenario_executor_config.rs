#[derive(Default, Clone, Debug)]
pub enum ScenarioExecutorConfig {
    #[default]
    Debugger,
    WasmerProd,
    Experimental,
    Composite(Vec<ScenarioExecutorConfig>),
}

impl ScenarioExecutorConfig {
    /// Try using the current config, if it cannot be used, attempt the same with the next one.
    pub fn then(self, next: Self) -> Self {
        if let Self::Composite(mut list) = self {
            list.push(next);
            Self::Composite(list)
        } else {
            Self::Composite(vec![self, next])
        }
    }
}
