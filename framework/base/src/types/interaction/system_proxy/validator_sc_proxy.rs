use multiversx_chain_core::types::{BLSKey, BLSSignature};
use multiversx_sc_codec::multi_types::{MultiValue2, MultiValueVec};

use crate::types::{
    BigUint, EgldPayment, ProxyArg, Tx, TxEnv, TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
};

/// Proxy for the Validator system smart contract.
pub struct ValidatorSCProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for ValidatorSCProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = ValidatorSCProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        ValidatorSCProxyMethods { wrapped_tx: tx }
    }
}

/// Method container of the Validator system smart contract proxy.
pub struct ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, To, Gas> ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// amount required for staking is 2500 EGLD per BLS key
    pub fn stake<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<MultiValueVec<MultiValue2<BLSKey, BLSSignature>>>,
    >(
        self,
        max_nodes_to_run: Arg0,
        bls_keys_signatures: Arg1,
        amount: BigUint<Env::Api>,
    ) -> TxTypedCall<Env, From, To, EgldPayment<<Env as TxEnv>::Api>, Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .argument(&max_nodes_to_run)
            .argument(&bls_keys_signatures)
            .egld(amount)
            .original_result()
    }
}
