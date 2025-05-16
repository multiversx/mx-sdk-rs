mod runtime_instance_call;
mod runtime_instance_call_default;

pub use runtime_instance_call::{RuntimeInstanceCall, RuntimeInstanceCallLambda};
pub use runtime_instance_call_default::RuntimeInstanceCallLambdaDefault;

use std::{
    ops::Deref,
    sync::{Arc, Mutex, Weak},
};

use multiversx_chain_vm_executor::{CompilationOptions, Executor};

use crate::{
    blockchain::VMConfigRef,
    display_util::address_hex,
    host::context::{TxContext, TxContextRef},
};

pub struct Runtime {
    pub vm_ref: VMConfigRef,
    pub executor: Box<dyn Executor + Send + Sync>,
    pub executor_context_cell: Mutex<Option<TxContextRef>>,
}

#[derive(Clone)]
pub struct RuntimeRef(Arc<Runtime>);

#[derive(Clone)]
pub struct RuntimeWeakRef(Weak<Runtime>);

impl Runtime {
    pub fn new(vm_ref: VMConfigRef, executor: Box<dyn Executor + Send + Sync>) -> Self {
        Runtime {
            vm_ref,
            executor,
            executor_context_cell: Mutex::new(None),
        }
    }

    pub fn get_executor_context(&self) -> TxContextRef {
        self.executor_context_cell
            .lock()
            .unwrap()
            .clone()
            .expect("no executor context configured")
    }

    fn set_executor_context(&self, value: Option<TxContextRef>) {
        let mut cell_ref = self.executor_context_cell.lock().unwrap();
        *cell_ref = value;
    }
}

impl RuntimeRef {
    pub fn new(vm_ref: VMConfigRef, executor: Box<dyn Executor + Send + Sync>) -> Self {
        RuntimeRef(Arc::new(Runtime::new(vm_ref, executor)))
    }

    pub fn downgrade(&self) -> RuntimeWeakRef {
        RuntimeWeakRef(Arc::downgrade(&self.0))
    }

    pub fn get_mut(&mut self) -> &mut Runtime {
        Arc::get_mut(&mut self.0).expect(
            "RuntimeRef cannot grant mutable access, because more than one strong reference exists",
        )
    }

    /// Helpful for initializing a runtime that contain executors
    /// that need to reference back to the runtime itself.
    ///
    /// The initializer function receives weak pointer references,
    /// because circular strong references ensure a memory leak.
    pub fn new_cyclic<F>(init_fn: F) -> RuntimeRef
    where
        F: FnOnce(RuntimeWeakRef) -> Runtime,
    {
        let runtime_arc = Arc::new_cyclic(|weak| init_fn(RuntimeWeakRef(weak.clone())));
        RuntimeRef(runtime_arc)
    }
}

impl Deref for RuntimeRef {
    type Target = Runtime;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl RuntimeWeakRef {
    pub fn upgrade(&self) -> RuntimeRef {
        RuntimeRef(
            self.0
                .upgrade()
                .expect("RuntimeWeakRef points to a dropped reference"),
        )
    }
}

impl RuntimeRef {
    /// Executes smart contract call using the given tx context, and the configured executor.
    ///
    /// It is possible to customize the specific instance call using the given lambda argument.
    /// Default it is the `instance_call` function.
    pub fn execute<F>(&self, tx_context: TxContext, call_lambda: F) -> TxContext
    where
        F: RuntimeInstanceCallLambda,
    {
        let func_name = tx_context.tx_input_box.func_name.clone();
        let contract_code = get_contract_identifier(&tx_context);
        let gas_limit = tx_context.input_ref().gas_limit;

        let tx_context_ref = TxContextRef::new(Arc::new(tx_context));

        self.set_executor_context(Some(tx_context_ref.clone()));

        let compilation_options = CompilationOptions {
            gas_limit,
            unmetered_locals: 0,
            max_memory_grow: 0,
            max_memory_grow_delta: 0,
            opcode_trace: false,
            metering: true,
            runtime_breakpoints: true,
        };

        let instance = self
            .executor
            .new_instance(contract_code.as_slice(), &compilation_options)
            .expect("error instantiating executor instance");

        self.set_executor_context(None);

        call_lambda.call(RuntimeInstanceCall {
            instance: &*instance,
            func_name: func_name.as_str(),
            tx_context_ref: &tx_context_ref,
        });

        std::mem::drop(instance);

        Arc::into_inner(tx_context_ref.0)
            .expect("cannot extract final TxContext from stack because of lingering references")
    }
}

fn get_contract_identifier(tx_context: &TxContext) -> Vec<u8> {
    tx_context
        .tx_cache
        .with_account(&tx_context.tx_input_box.to, |account| {
            account.contract_path.clone().unwrap_or_else(|| {
                panic!(
                    "Recipient account is not a smart contract {}",
                    address_hex(&tx_context.tx_input_box.to)
                )
            })
        })
}
