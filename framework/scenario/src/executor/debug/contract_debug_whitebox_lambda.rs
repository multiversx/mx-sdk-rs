use multiversx_chain_vm::host::{
    context::TxFunctionName,
    runtime::{RuntimeInstanceCall, RuntimeInstanceCallLambda},
};

use crate::executor::debug::ContractDebugInstance;

pub struct ContractDebugWhiteboxLambda<F>
where
    F: FnOnce(),
{
    function_name: TxFunctionName,
    whitebox_fn: F,
    panic_message_flag: bool,
}

impl<F> ContractDebugWhiteboxLambda<F>
where
    F: FnOnce(),
{
    pub fn new(function_name: TxFunctionName, whitebox_fn: F) -> Self {
        ContractDebugWhiteboxLambda {
            function_name,
            whitebox_fn,
            panic_message_flag: true,
        }
    }

    pub fn panic_message(mut self, panic_message_flag: bool) -> Self {
        self.panic_message_flag = panic_message_flag;
        self
    }
}

impl<F> RuntimeInstanceCallLambda for ContractDebugWhiteboxLambda<F>
where
    F: FnOnce(),
{
    fn call(self, instance_call: RuntimeInstanceCall<'_>) {
        assert_eq!(
            self.function_name,
            instance_call.tx_context_ref.input_ref().func_name,
            "unexpected whitebox function name"
        );

        ContractDebugInstance::wrap_lambda_call(
            self.panic_message_flag,
            instance_call,
            self.whitebox_fn,
        );
    }

    fn override_function_name(&self) -> Option<TxFunctionName> {
        Some(self.function_name.clone())
    }
}
