use std::marker::PhantomData;

use crate::mandos_system::model::{AddressValue, BytesValue, TxExpect, TxQuery};

use super::ScQueryStep;

#[derive(Debug)]
pub struct TypedScQuery<OriginalResult> {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxQuery>,
    pub expect: Option<TxExpect>,
    _return_type: PhantomData<OriginalResult>,
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

impl<OriginalResult> From<ScQueryStep> for TypedScQuery<OriginalResult> {
    fn from(untyped: ScQueryStep) -> Self {
        Self {
            tx_id: untyped.tx_id,
            comment: untyped.comment,
            tx: untyped.tx,
            expect: untyped.expect,
            _return_type: PhantomData,
        }
    }
}

impl<OriginalResult> TypedScQuery<OriginalResult> {
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
