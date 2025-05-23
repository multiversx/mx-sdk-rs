
#[derive(Default, Clone, Copy, Debug)]
pub enum ScenarioExecutorConfig {
    #[default]
    Debugger,
    WasmerProd,
    Experimental,
    TryDebuggerThenWasmerProd,
    TryWasmerProdThenDebugger,
    TryDebuggerThenExperimental,
    TryExperimentalThenDebugger,
}