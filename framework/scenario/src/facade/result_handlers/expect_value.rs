use multiversx_sc::{
    abi::TypeAbiFrom,
    codec::TopEncodeMulti,
    types::{RHListItem, RHListItemExec, TxEnv},
};

use crate::scenario_model::{BytesValue, CheckValue, TxExpect, TxResponse};

/// Verifies that transaction result matches the given value.
///
/// Can only be used in tests and interactors, not available in contracts.
pub struct ExpectValue<T>(pub T);

impl<Env, Original, T> RHListItem<Env, Original> for ExpectValue<T>
where
    Env: TxEnv,
    T: TopEncodeMulti,
    Original: TypeAbiFrom<T>,
{
    type Returns = ();
}

impl<Env, Original, T> RHListItemExec<TxResponse, Env, Original> for ExpectValue<T>
where
    Env: TxEnv<RHExpect = TxExpect>,
    T: TopEncodeMulti,
    Original: TypeAbiFrom<T>,
{
    fn item_tx_expect(&self, mut prev: TxExpect) -> TxExpect {
        let mut encoded = Vec::<Vec<u8>>::new();
        self.0.multi_encode(&mut encoded).expect("encoding error");
        let out_values = encoded
            .into_iter()
            .map(|value| CheckValue::Equal(BytesValue::from(value)))
            .collect();
        prev.out = CheckValue::Equal(out_values);
        prev
    }

    fn item_process_result(self, _: &TxResponse) -> Self::Returns {}
}
