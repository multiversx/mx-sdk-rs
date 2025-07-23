use multiversx_chain_vm::host::runtime::RuntimeWeakRef;
use multiversx_chain_vm_executor::Executor;

/// Function that creates an executor from a runtime reference.
///
/// Created specifically to avoid referencing the Wasmer 2.2 crate from the VM.
pub type CustomExecutorFn = fn(RuntimeWeakRef) -> Box<dyn Executor + Send + Sync>;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum ExecutorConfig {
    /// Uses the debugger infrastructure: testing the smart contract code directly.
    #[default]
    Debugger,

    /// Use to add a custom executor builder function in the contract crate (via dependency inversion).
    ///
    /// Created specifically to avoid referencing the Wasmer 2.2 crate from the VM.
    Custom(CustomExecutorFn),

    /// Use the compiled contract in the experimental Wasmer 6 executor.
    Experimental,

    /// If feature `compiled-sc-tests` is active, use the given config. Otherwise use fallback config.
    CompiledFeatureIfElse {
        if_compiled: Box<ExecutorConfig>,
        fallback: Box<ExecutorConfig>,
    },

    /// Defines a list of executors, to be used in order.
    /// If one of them refuses to execute, the next one is used as fallback.
    Composite(Vec<ExecutorConfig>),
}

impl ExecutorConfig {
    /// Try using the current config, if it cannot be used, attempt the same with the next one.
    pub fn then(self, next: Self) -> Self {
        match self {
            Self::Composite(mut list) => {
                next.append_flattened_to_vec(&mut list);
                Self::from_list(list)
            }
            _ => {
                let mut list = Vec::new();
                self.append_flattened_to_vec(&mut list);
                next.append_flattened_to_vec(&mut list);
                Self::from_list(list)
            }
        }
    }

    /// Use the compiled contract only if feature `compiled-sc-tests` is active. Otherwise use fallback config.
    pub fn compiled_tests_if_else(if_compiled: Self, fallback: Self) -> Self {
        Self::CompiledFeatureIfElse {
            if_compiled: Box::new(if_compiled),
            fallback: Box::new(fallback),
        }
    }

    /// Run the compiled contract with the experimental executor if feature `compiled-sc-tests` is active. Otherwise use fallback config.
    pub fn compiled_tests_or(fallback: Self) -> Self {
        Self::compiled_tests_if_else(Self::Experimental, fallback)
    }

    /// Tests with:
    /// - compiled tests (if feature `compiled-sc-tests` is active),
    /// - otherwise:
    ///     - first try the debugger,
    ///     - then finally the experimental Wasmer.
    ///
    /// This means contracts will be tested natively in the wasm tests.
    pub fn full_suite() -> Self {
        Self::compiled_tests_or(Self::Debugger.then(Self::Experimental))
    }

    fn from_list(mut list: Vec<ExecutorConfig>) -> Self {
        if list.len() == 1 {
            list.pop().unwrap()
        } else {
            Self::Composite(list)
        }
    }

    fn append_flattened_to_vec(self, destination: &mut Vec<ExecutorConfig>) {
        match self {
            Self::Composite(list) => {
                for item in list {
                    item.append_flattened_to_vec(destination);
                }
            }
            _ => {
                destination.push(self);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executor_config_then_test() {
        assert_eq!(
            ExecutorConfig::Debugger.then(ExecutorConfig::Experimental),
            ExecutorConfig::Composite(vec![ExecutorConfig::Debugger, ExecutorConfig::Experimental])
        );

        assert_eq!(
            ExecutorConfig::Debugger
                .then(ExecutorConfig::Experimental)
                .then(ExecutorConfig::Debugger),
            ExecutorConfig::Composite(vec![
                ExecutorConfig::Debugger,
                ExecutorConfig::Experimental,
                ExecutorConfig::Debugger
            ])
        );
    }

    #[test]
    fn executor_config_flatten_test() {
        assert_eq!(
            ExecutorConfig::Debugger
                .then(ExecutorConfig::Experimental.then(ExecutorConfig::Debugger)),
            ExecutorConfig::Composite(vec![
                ExecutorConfig::Debugger,
                ExecutorConfig::Experimental,
                ExecutorConfig::Debugger
            ])
        );
    }

    #[test]
    fn executor_config_full_suite() {
        assert_eq!(
            ExecutorConfig::full_suite(),
            ExecutorConfig::CompiledFeatureIfElse {
                if_compiled: Box::new(ExecutorConfig::Experimental),
                fallback: Box::new(ExecutorConfig::Composite(vec![
                    ExecutorConfig::Debugger,
                    ExecutorConfig::Experimental,
                ]))
            }
        );
    }
}
