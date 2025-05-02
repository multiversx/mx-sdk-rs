use std::rc::Rc;

use multiversx_chain_vm::host::{
    context::{TxContextRef, TxFunctionName, TxPanic},
    runtime::RuntimeInstanceCall,
};
use multiversx_chain_vm_executor::{BreakpointValue, ExecutorError, Instance, InstanceCallError};
use multiversx_sc::chain_core::types::ReturnCode;

use super::{
    catch_tx_panic, ContractContainer, ContractContainerRef, ContractDebugStack, StaticVarData,
};

/// Used as a flag to check the instance under lambda calls.
/// Since it is an invalid function name, any other instance should reject it.
const FUNC_CONTEXT_PUSH: &str = "<ContractDebugInstance-PushContext>";
const FUNC_CONTEXT_POP: &str = "<ContractDebugInstance-PopContext>";

#[derive(Clone, Debug)]
pub struct ContractDebugInstance {
    pub tx_context_ref: TxContextRef,
    pub contract_container_ref: ContractContainerRef,
    pub static_var_ref: Rc<StaticVarData>,
}

impl ContractDebugInstance {
    pub fn new(tx_context_ref: TxContextRef, contract_container: ContractContainerRef) -> Self {
        ContractDebugInstance {
            tx_context_ref,
            contract_container_ref: contract_container,
            static_var_ref: Default::default(),
        }
    }

    /// Dummy instance for tests where no proper context is created on stack.
    pub fn dummy() -> Self {
        ContractDebugInstance {
            tx_context_ref: TxContextRef::dummy(),
            contract_container_ref: ContractContainerRef::new(ContractContainer::dummy()),
            static_var_ref: Default::default(),
        }
    }

    pub(super) fn wrap_lambda_call<F>(
        panic_message_flag: bool,
        instance_call: RuntimeInstanceCall,
        f: F,
    ) where
        F: FnOnce(),
    {
        // assert!(
        //     instance_call.func_name == TxFunctionName::WHITEBOX_CALL.as_str()
        //         || instance_call.func_name == "init", // TODO make it also WHITEBOX_CALL or some whitebox init
        //     "misconfigured whitebox call: {}",
        //     instance_call.func_name,
        // );

        assert!(
            instance_call.instance.has_function(FUNC_CONTEXT_PUSH),
            "lambda call is not running on top of a DebugSCInstance instance"
        );

        let _ = instance_call.instance.call(FUNC_CONTEXT_PUSH);

        let result = catch_tx_panic(panic_message_flag, || {
            f();
            Ok(())
        });

        if let Err(tx_panic) = result {
            ContractDebugStack::static_peek()
                .tx_context_ref
                .replace_tx_result_with_error(tx_panic);
        }

        let _ = instance_call.instance.call(FUNC_CONTEXT_POP);
    }

    fn call_endpoint(&self, func_name: &str) {
        let tx_func_name = TxFunctionName::from(func_name);

        ContractDebugStack::static_push(self.clone());

        let result = catch_tx_panic(self.contract_container_ref.0.panic_message, || {
            let call_successful = self.contract_container_ref.0.call(&tx_func_name);
            if call_successful {
                Ok(())
            } else {
                Err(TxPanic::new(
                    ReturnCode::FunctionNotFound,
                    "invalid function (not found)",
                ))
            }
        });

        if let Err(tx_panic) = result {
            self.tx_context_ref
                .clone()
                .replace_tx_result_with_error(tx_panic);
        }

        ContractDebugStack::static_pop();
    }
}

impl Instance for ContractDebugInstance {
    fn call(&self, func_name: &str) -> Result<(), InstanceCallError> {
        match func_name {
            FUNC_CONTEXT_PUSH => {
                ContractDebugStack::static_push(self.clone());
            },
            FUNC_CONTEXT_POP => {
                ContractDebugStack::static_pop();
            },
            _ => self.call_endpoint(func_name),
        }
        Ok(())
    }

    fn check_signatures(&self) -> bool {
        true
    }

    fn has_function(&self, func_name: &str) -> bool {
        match func_name {
            FUNC_CONTEXT_PUSH => true,
            FUNC_CONTEXT_POP => true,
            _ => self.contract_container_ref.has_function(func_name),
        }
    }

    fn get_exported_function_names(&self) -> Vec<String> {
        panic!("ContractDebugInstance get_exported_function_names not yet supported")
    }

    fn set_points_limit(&self, _limit: u64) -> Result<(), ExecutorError> {
        Ok(())
    }

    fn get_points_used(&self) -> Result<u64, ExecutorError> {
        Ok(0)
    }

    fn get_breakpoint_value(&self) -> Result<BreakpointValue, ExecutorError> {
        Ok(BreakpointValue::None)
    }

    fn reset(&self) -> Result<(), ExecutorError> {
        panic!("ContractDebugInstance reset not supported")
    }

    fn cache(&self) -> Result<Vec<u8>, ExecutorError> {
        panic!("ContractDebugInstance cache not supported")
    }
}
