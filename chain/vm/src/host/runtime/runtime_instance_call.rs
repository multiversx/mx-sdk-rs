use multiversx_chain_vm_executor::Instance;

use crate::host::context::{TxContextRef, TxFunctionName};

pub struct RuntimeInstanceCall<'a> {
    pub instance: &'a dyn Instance,
    pub func_name: &'a str,
    pub tx_context_ref: &'a TxContextRef,
}

pub trait RuntimeInstanceCallLambda {
    fn call(self, instance_call: RuntimeInstanceCall<'_>);

    fn override_function_name(&self) -> Option<TxFunctionName>;
}
