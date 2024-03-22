use crate::{
    contract_base::SendRawWrapper,
    types::{
        BigUint, EsdtTokenPayment, ManagedAddress, ManagedVec, MultiEsdtPayment, TxFrom,
        TxToSpecified,
    },
};

use super::{AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.amount == 0u32
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        MultiEsdtPayment::from_single_item(self).perform_transfer_execute(env, to, gas_limit, fc);
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            if self.token_nonce == 0 {
                let fc_conv = fc.convert_to_single_transfer_fungible_call(&self);
                f(to_addr, &BigUint::zero(), &fc_conv)
            } else {
                let fc_conv = fc.convert_to_single_transfer_nft_call(to_addr, &self);
                f(&from.resolve_address(env), &BigUint::zero(), &fc_conv)
            }
        })
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: MultiEsdtPayment::from_single_item(self),
        }
    }
}
