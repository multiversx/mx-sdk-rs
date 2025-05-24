#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum ExecutorConfig {
    /// Uses the debugger infrastructure: testing the smart contract code directly.
    #[default]
    Debugger,

    /// Use the compiled contract in the legacy Wasmer 2.2 executor.
    WasmerProd,

    /// Use the compiled contract in the experimental Wasmer 6 executor.
    Experimental,

    /// Use the compiled contract only if feature `compiled-sc-tests` is active.
    ///
    /// Forwards to the experimental Wasmer 6 executor.
    ///
    /// Cannot be used on its own.
    CompiledTests,

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
            },
            _ => {
                let mut list = Vec::new();
                self.append_flattened_to_vec(&mut list);
                next.append_flattened_to_vec(&mut list);
                Self::from_list(list)
            },
        }
    }

    /// Tests with:
    /// - compiled tests (if feature `compiled-sc-tests` is active),
    /// - then tries the debugger,
    /// - then finally the experimental Wasmer.
    ///
    /// This means contracts will be tested natively in the wasm tests.
    pub fn full_suite() -> Self {
        Self::CompiledTests
            .then(Self::Debugger)
            .then(Self::Experimental)
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
            Self::CompiledTests => {
                if cfg!(feature = "compiled-sc-tests") {
                    destination.push(Self::Experimental);
                }
            },
            Self::Composite(list) => {
                for item in list {
                    item.append_flattened_to_vec(destination);
                }
            },
            _ => {
                destination.push(self);
            },
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

    #[cfg_attr(feature = "compiled-sc-tests", ignore)]
    #[test]
    fn executor_config_then_not_compiled_test() {
        assert_eq!(
            ExecutorConfig::CompiledTests.then(ExecutorConfig::Debugger),
            ExecutorConfig::Debugger,
        );

        assert_eq!(
            ExecutorConfig::CompiledTests
                .then(ExecutorConfig::Experimental)
                .then(ExecutorConfig::Debugger),
            ExecutorConfig::Composite(vec![ExecutorConfig::Experimental, ExecutorConfig::Debugger])
        );
    }

    #[cfg_attr(not(feature = "compiled-sc-tests"), ignore)]
    #[test]
    fn executor_config_then_compiled_test() {
        assert_eq!(
            ExecutorConfig::CompiledTests.then(ExecutorConfig::Experimental),
            ExecutorConfig::Composite(vec![
                ExecutorConfig::CompiledTests,
                ExecutorConfig::Experimental
            ])
        );

        assert_eq!(
            ExecutorConfig::CompiledTests
                .then(ExecutorConfig::Experimental)
                .then(ExecutorConfig::Debugger),
            ExecutorConfig::Composite(vec![
                ExecutorConfig::CompiledTests,
                ExecutorConfig::Experimental,
                ExecutorConfig::Debugger
            ])
        );
    }
}
