use std::marker::PhantomData;

use elrond_wasm::types::ContractCall;

use crate::{
    mandos_system::model::{AddressValue, BytesValue, TxExpect, TxQuery},
    DebugApi,
};

use super::{process_contract_call, ScQueryStep};

#[derive(Debug)]
pub struct TypedScQuery<OriginalResult> {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxQuery>,
    pub expect: Option<TxExpect>,
    _return_type: PhantomData<OriginalResult>,
}

pub trait IntoVMQuery<OriginalResult> {
    fn into_vm_query(self) -> TypedScQuery<OriginalResult>;
}

impl<OriginalResult> IntoVMQuery<OriginalResult> for ContractCall<DebugApi, OriginalResult> {
    fn into_vm_query(self) -> TypedScQuery<OriginalResult> {
        TypedScQuery::<OriginalResult>::default().set_contract_call(self)
    }
}

impl<OriginalResult> Default for TypedScQuery<OriginalResult> {
    fn default() -> Self {
        Self {
            tx_id: Default::default(),
            comment: Default::default(),
            tx: Default::default(),
            expect: Default::default(),
            _return_type: PhantomData,
        }
    }
}

impl<OriginalResult> From<TypedScQuery<OriginalResult>> for ScQueryStep {
    fn from(typed: TypedScQuery<OriginalResult>) -> Self {
        Self {
            tx_id: typed.tx_id,
            comment: typed.comment,
            tx: typed.tx,
            expect: typed.expect,
        }
    }
}

impl<OriginalResult> TypedScQuery<OriginalResult> {
    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    fn set_contract_call(mut self, contract_call: ContractCall<DebugApi, OriginalResult>) -> Self {
        let (to_str, function, mandos_args) = process_contract_call(contract_call);
        self = self.to(to_str.as_str());
        self = self.function(function.as_str());
        for arg in mandos_args {
            self = self.argument(arg.as_str());
        }
        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn argument<A>(mut self, expr: A) -> Self
    where
        BytesValue: From<A>,
    {
        self.tx.arguments.push(BytesValue::from(expr));
        self
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.to = AddressValue::from(address);
        self
    }

    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }
}
