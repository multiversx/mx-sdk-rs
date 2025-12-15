use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, ManagedAddress, Payment, Ref, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

macro_rules! impl_txpayment_for_payment_ref {
    ($ty:ty) => {
        impl<Env> TxPayment<Env> for $ty
        where
            Env: TxEnv,
        {
            #[inline]
            fn is_no_payment(&self, _env: &Env) -> bool {
                // amount is NonZeroBigUint
                false
            }

            fn perform_transfer_execute_fallible(
                self,
                env: &Env,
                to: &ManagedAddress<Env::Api>,
                gas_limit: u64,
                fc: FunctionCall<Env::Api>,
            ) -> Result<(), TransferExecuteFailed> {
                self.as_refs()
                    .perform_transfer_execute_fallible(env, to, gas_limit, fc)
            }

            fn perform_transfer_execute_legacy(
                self,
                env: &Env,
                to: &ManagedAddress<Env::Api>,
                gas_limit: u64,
                fc: FunctionCall<Env::Api>,
            ) {
                self.as_refs()
                    .perform_transfer_execute_legacy(env, to, gas_limit, fc)
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
                F: FnOnce(
                    &ManagedAddress<Env::Api>,
                    &BigUint<Env::Api>,
                    FunctionCall<Env::Api>,
                ) -> R,
            {
                self.as_refs().with_normalized(env, from, to, fc, f)
            }

            fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
                self.as_refs().into_full_payment_data(env)
            }
        }
    };
}

impl_txpayment_for_payment_ref!(&Payment<Env::Api>);
impl_txpayment_for_payment_ref!(Ref<'_, Payment<Env::Api>>);
