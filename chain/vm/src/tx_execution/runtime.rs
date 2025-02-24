use std::{
    cell::RefCell,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex, Weak},
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
    pub override_executor: Option<Box<dyn Executor + Send + Sync>>,
    pub stack: RefCell<Stack>,
    pub current_context_cell: RefCell<Option<TxContextRef>>,
}

#[derive(Clone)]
pub struct RuntimeRef(pub Arc<Runtime>);

#[derive(Clone)]
pub struct RuntimeWeakRef(pub Weak<Runtime>);

impl Runtime {
    pub fn new(vm_ref: BlockchainVMRef) -> Self {
        Runtime {
            vm_ref,
            override_executor: None,
            stack: Default::default(),
            current_context_cell: RefCell::new(None),
        }
    }

    pub fn executor(&self) -> &(dyn Executor + Send + Sync) {
        self.override_executor
            .as_ref()
            .unwrap_or(&self.vm_ref.executor)
            .deref()
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
    pub fn new(vm_ref: BlockchainVMRef) -> Self {
        RuntimeRef(Arc::new(Runtime::new(vm_ref)))
    }

    pub fn downgrade(&self) -> RuntimeWeakRef {
        RuntimeWeakRef(Arc::downgrade(&self.0))
    }

    pub fn get_mut(&mut self) -> &mut Runtime {
        Arc::get_mut(&mut self.0).expect(
            "RuntimeRef cannot grant mutable access, because more than one strong reference exists",
        )
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

fn instance_call(instance: &dyn Instance, func_name: &str) {
    instance.call(func_name).expect("execution error");
}

impl RuntimeRef {
    /// TODO: shorten to just execute when cleaning up
    pub fn execute_in_runtime(
        &self,
        tx_input: TxInput,
        state: &mut BlockchainStateRef,
    ) -> (TxResult, BlockchainUpdate) {
        self.execute_lambda_in_runtime(tx_input, state, instance_call)
    }

    pub fn execute_lambda_in_runtime<F>(
        &self,
        tx_input: TxInput,
        state: &mut BlockchainStateRef,
        call_lambda: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(&dyn Instance, &str),
    {
        let func_name = tx_input.func_name.clone();
        let tx_cache = TxCache::new(state.get_arc());
        let tx_context = TxContext::new(self.clone(), tx_input, tx_cache);
        let contract_code = get_contract_identifier(&tx_context);
        let tx_context_ref = TxContextRef::new(Arc::new(tx_context));

        self.set_current_context(Some(tx_context_ref.clone()));

        let instance = self
            .executor()
            .new_instance(contract_code.as_slice(), &COMPILATION_OPTIONS)
            .expect("error instantiating executor instance");
        let instance_ref = Rc::new(instance);

        self.stack_push(StackItem {
            instance_ref: instance_ref.clone(),
            tx_context_ref: tx_context_ref.clone(),
        });

        call_lambda(&**instance_ref, func_name.as_str());

        std::mem::drop(instance_ref);

        let stack_item = self.stack_pop();
        std::mem::drop(stack_item);
        self.set_current_context(self.top_tx_context_ref());

        let tx_context = Arc::into_inner(tx_context_ref.0)
            .expect("cannot extract final TxContext from stack because of lingering references");

        tx_context.into_results()
    }
}
