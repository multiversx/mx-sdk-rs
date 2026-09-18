use multiversx_sc_codec::TopEncodeMulti;

use crate::types::{TxPayment, TxPaymentEgldOnly, TxResultHandler};

use super::{
    DeployCall, FunctionCall, Tx, TxCodeSource, TxEnv, TxFrom, TxGas, TxTo, TxTypedCall,
    TxTypedDeploy, TxTypedUpgrade, UpgradeCall,
};

impl<Env, From, To, Payment, Gas, RH> multiversx_sc_abi::ApplyArgument
    for Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    fn apply_argument<A: TopEncodeMulti>(self, arg: &A) -> Self {
        self.argument(arg)
    }
}

impl<Env, From, To, Payment, Gas, CodeSource, RH> multiversx_sc_abi::ApplyArgument
    for Tx<Env, From, To, Payment, Gas, DeployCall<Env, CodeSource>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    CodeSource: TxCodeSource<Env>,
    RH: TxResultHandler<Env>,
{
    fn apply_argument<A: TopEncodeMulti>(self, arg: &A) -> Self {
        self.argument(arg)
    }
}

impl<Env, From, To, Payment, Gas, CodeSource, RH> multiversx_sc_abi::ApplyArgument
    for Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, CodeSource>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentEgldOnly<Env>,
    Gas: TxGas<Env>,
    CodeSource: TxCodeSource<Env>,
    RH: TxResultHandler<Env>,
{
    fn apply_argument<A: TopEncodeMulti>(self, arg: &A) -> Self {
        self.argument(arg)
    }
}

impl<Env, From, To, Gas, P, O> multiversx_sc_abi::IntoCall<P, O>
    for Tx<Env, From, To, (), Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    P: TxPayment<Env>,
{
    type Out = TxTypedCall<Env, From, To, P, Gas, O>;

    fn into_call(self, payment: P, function_name: &str) -> Self::Out {
        self.payment(payment)
            .raw_call(function_name)
            .original_result()
    }
}

impl<Env, From, Gas, P, O> multiversx_sc_abi::IntoDeploy<P, O>
    for Tx<Env, From, (), (), Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
    P: TxPaymentEgldOnly<Env>,
{
    type Out = TxTypedDeploy<Env, From, P, Gas, O>;

    fn into_deploy(self, payment: P) -> Self::Out {
        self.payment(payment).raw_deploy().original_result()
    }
}

impl<Env, From, To, Gas, P, O> multiversx_sc_abi::IntoUpgrade<P, O>
    for Tx<Env, From, To, (), Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    P: TxPaymentEgldOnly<Env>,
{
    type Out = TxTypedUpgrade<Env, From, To, P, Gas, O>;

    fn into_upgrade(self, payment: P) -> Self::Out {
        self.payment(payment).raw_upgrade().original_result()
    }
}
