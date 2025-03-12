use std::{
    ops::Deref,
    sync::{Arc, Mutex, Weak},
};

use multiversx_chain_vm_executor::{BreakpointValue, Executor, Instance};

use crate::{
    display_util::address_hex,
    host::context::{BlockchainUpdate, TxCache, TxContext, TxContextRef, TxInput, TxResult},
    world_mock::{BlockchainStateRef, BlockchainVMRef},
};

pub struct Runtime {
    pub vm_ref: BlockchainVMRef,
    pub executor: Box<dyn Executor + Send + Sync>,
    pub executor_context_cell: Mutex<Option<TxContextRef>>,
}

#[derive(Clone)]
pub struct RuntimeRef(Arc<Runtime>);

#[derive(Clone)]
pub struct RuntimeWeakRef(Weak<Runtime>);

pub struct RuntimeInstanceCall<'a> {
    pub instance: &'a dyn Instance,
    pub func_name: &'a str,
    pub tx_context_ref: &'a TxContextRef,
}

impl Runtime {
    pub fn new(vm_ref: BlockchainVMRef, executor: Box<dyn Executor + Send + Sync>) -> Self {
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
    pub fn new(vm_ref: BlockchainVMRef, executor: Box<dyn Executor + Send + Sync>) -> Self {
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

pub fn instance_call(instance_call: RuntimeInstanceCall<'_>) {
    if !instance_call.instance.has_function(instance_call.func_name) {
        *instance_call.tx_context_ref.result_lock() = TxResult::from_function_not_found();
        return;
    }

    let result = instance_call.instance.call(instance_call.func_name);
    if let Err(err) = result {
        let breakpoint = instance_call
            .instance
            .get_breakpoint_value()
            .expect("error retrieving instance breakpoint value");
        println!("breakpoint: {breakpoint:?}");
        if breakpoint == BreakpointValue::None {
            *instance_call.tx_context_ref.result_lock() = TxResult::from_vm_error(err);
        }
    }
}

impl RuntimeRef {
    /// TODO: shorten to just execute when cleaning up
    pub fn execute_in_runtime(
        &self,
        tx_input: TxInput,
        state: &mut BlockchainStateRef,
    ) -> (TxResult, BlockchainUpdate) {
        let tx_cache = TxCache::new(state.get_arc());
        let tx_context = TxContext::new(self.clone(), tx_input, tx_cache);
        self.execute_lambda_in_runtime(tx_context, instance_call)
    }

    pub fn execute_lambda_in_runtime<F>(
        &self,
        tx_context: TxContext,
        call_lambda: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        let tx_context = self.execute_tx_context_in_runtime(tx_context, call_lambda);
        tx_context.into_results()
    }

    pub fn execute_tx_context_in_runtime<F>(
        &self,
        tx_context: TxContext,
        call_lambda: F,
    ) -> TxContext
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        let func_name = tx_context.tx_input_box.func_name.clone();
        let contract_code = get_contract_identifier(&tx_context);
        let tx_context_ref = TxContextRef::new(Arc::new(tx_context));

        self.set_executor_context(Some(tx_context_ref.clone()));

        let instance = self
            .executor
            .new_instance(contract_code.as_slice(), &self.vm_ref.compilation_options)
            .expect("error instantiating executor instance");

        self.set_executor_context(None);

        call_lambda(RuntimeInstanceCall {
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
