use crate::{
    contract_base::{SendRawWrapper, TransferExecuteFailed},
    types::{BigUint, EsdtTokenPaymentRefs, ManagedAddress, ManagedVec, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for EsdtTokenPaymentRefs<'_, Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.amount == &0u32
    }

    fn perform_transfer_execute_fallible(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        // TODO: some clones could be avoided?
        let mut payments = ManagedVec::new();
        payments.push(self.to_owned_payment().into_multi_egld_or_esdt_payment());

        // using multi-transfer (instead of single ESDT/NFT), because it is the only one that is fallible
        SendRawWrapper::<Env::Api>::new().multi_egld_or_esdt_transfer_execute_fallible(
            to,
            &payments,
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        )
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
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            if self.token_nonce == 0 {
                let fc_conv = fc.convert_to_single_transfer_fungible_call(self);
                f(to_addr, &*BigUint::zero_ref(), fc_conv)
            } else {
                let fc_conv = fc.convert_to_single_transfer_nft_call(to_addr, self);
                f(&from.resolve_address(env), &*BigUint::zero_ref(), fc_conv)
            }
        })
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: ManagedVec::from_single_item(
                self.to_owned_payment().into_multi_egld_or_esdt_payment(),
            ),
        }
    }
}
