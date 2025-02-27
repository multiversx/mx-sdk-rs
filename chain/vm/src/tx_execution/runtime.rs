use std::{
    cell::RefCell,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Weak},
};

use multiversx_chain_vm_executor::{Executor, Instance};

use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxContext, TxContextRef, TxInput, TxResult},
    world_mock::BlockchainStateRef,
};

use super::{
    exec_contract_endpoint::{get_contract_identifier, COMPILATION_OPTIONS},
    BlockchainVMRef, Stack, StackItem,
};

pub struct Runtime {
    pub vm_ref: BlockchainVMRef,
    pub executor: Box<dyn Executor + Send + Sync>,
    pub stack: RefCell<Stack>,
    pub current_context_cell: RefCell<Option<TxContextRef>>,
}

#[derive(Clone)]
pub struct RuntimeRef(Arc<Runtime>);

#[derive(Clone)]
pub struct RuntimeWeakRef(Weak<Runtime>);

pub struct RuntimeInstanceCall<'a> {
    pub instance: &'a dyn Instance,
    pub func_name: &'a str,
}

impl Runtime {
    pub fn new(vm_ref: BlockchainVMRef, executor: Box<dyn Executor + Send + Sync>) -> Self {
        Runtime {
            vm_ref,
            executor,
            stack: Default::default(),
            current_context_cell: RefCell::new(None),
        }
    }

    fn stack_push(&self, stack_item: StackItem) {
        if let Some(top) = self.stack.borrow().peek() {
            top.on_stack_top_leave();
        }
        self.stack.borrow_mut().push(stack_item);
        if let Some(top) = self.stack.borrow().peek() {
            top.on_stack_top_enter();
        }
    }

    fn stack_pop(&self) -> StackItem {
        if let Some(top) = self.stack.borrow().peek() {
            top.on_stack_top_leave();
        }
        let popped = self.stack.borrow_mut().pop();
        if let Some(top) = self.stack.borrow().peek() {
            top.on_stack_top_enter();
        }
        popped
    }

    pub fn top_tx_context_ref(&self) -> Option<TxContextRef> {
        self.stack
            .borrow()
            .peek()
            .map(|stack_item| stack_item.tx_context_ref.clone())
    }

    pub fn current_context(&self) -> TxContextRef {
        self.current_context_cell
            .borrow()
            .clone()
            .expect("no current context")
    }

    fn set_current_context(&self, value: Option<TxContextRef>) {
        *self.current_context_cell.borrow_mut() = value;
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
    instance_call
        .instance
        .call(instance_call.func_name)
        .expect("execution error");
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

        self.set_current_context(Some(tx_context_ref.clone()));

        let instance = self
            .executor
            .new_instance(contract_code.as_slice(), &COMPILATION_OPTIONS)
            .expect("error instantiating executor instance");
        let instance_ref = Rc::new(instance);

        self.stack_push(StackItem {
            instance_ref: instance_ref.clone(),
            tx_context_ref: tx_context_ref.clone(),
        });

        call_lambda(RuntimeInstanceCall {
            instance: &**instance_ref,
            func_name: func_name.as_str(),
        });

        std::mem::drop(instance_ref);

        let stack_item = self.stack_pop();
        std::mem::drop(stack_item);
        self.set_current_context(self.top_tx_context_ref());

        Arc::into_inner(tx_context_ref.0)
            .expect("cannot extract final TxContext from stack because of lingering references")
    }
}
