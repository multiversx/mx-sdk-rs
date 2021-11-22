use std::{lazy::OnceCell, ops::Deref, rc::Rc};

use elrond_wasm::api::ManagedTypeApi;

use crate::tx_mock::{TxContext, TxInput, TxResult};

use super::{BlockchainUpdate, TxCache};

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Debug)]
pub struct TxContextRef;

#[derive(Clone)]
pub type DebugApi = OnceCell<TxContextStack>;

#[derive(Debug)]
pub struct TxContextStack(Vec<TxContext>);

impl Deref for TxContextStack {
    type Target = Vec<TxContext>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait StaticStack {
    fn first(self) -> &'static TxContext;
    fn push(self, element: TxContext);
    fn pop(self) -> TxContext;
}

impl StaticStack for OnceCell<TxContextStack> {
    fn first(self) -> &'static TxContext {
        self.get().unwrap().first().unwrap()
    }
    fn push(self, element: TxContext) {
        self.get().unwrap().push(element)
    }
    fn pop(self) -> TxContext {
        self.get().unwrap().pop()
    }
}

impl ManagedTypeApi for DebugApi {}
impl TryStaticCast for DebugApi {}

pub static API_INSTANCE: OnceCell<TxContextStack> = OnceCell::new();

impl Clone for TxContextRef {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl TxContextRef {
    pub fn new(tx_input: TxInput, tx_cache: TxCache) -> TxContext {
        TxContext::new(tx_input, tx_cache)
    }

    pub fn dummy() -> TxContext {
        TxContext::dummy()
    }

    pub fn into_blockchain_updates() -> BlockchainUpdate {
        let tx_context = API_INSTANCE.first();
        let tx_cache = Rc::try_unwrap(tx_context.tx_cache).unwrap();
        tx_cache.into_blockchain_updates()
    }

    /// Consumes the current API and returns the contained output.
    /// Should be called at the end of a tx execution.
    /// Will fail if any other references to the tx context survive, this must be the last.
    pub fn into_tx_result(self) -> TxResult {
        // TODO: investigate if we can also destroy the Rc
        // can be done if we can make sure that no more references exist at this point
        // let tx_context = Rc::try_unwrap(self.0).unwrap();
        API_INSTANCE
            .first()
            .tx_result_cell
            .replace(TxResult::default())
    }
}
