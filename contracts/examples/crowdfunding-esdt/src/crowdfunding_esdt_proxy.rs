////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct CrowdfundingProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for CrowdfundingProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = CrowdfundingProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        CrowdfundingProxyMethods { wrapped_tx: tx }
    }
}

pub struct CrowdfundingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, Gas> CrowdfundingProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<BigUint<Env::Api>>,
        Arg1: CodecInto<u64>,
        Arg2: CodecInto<EgldOrEsdtTokenIdentifier<Env::Api>>,
    >(
        self,
        target: Arg0,
        deadline: Arg1,
        token_identifier: Arg2,
    ) -> Tx<Env, From, (), (), Gas, DeployCall<Env, ()>, OriginalResultMarker<()>> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&target)
            .argument(&deadline)
            .argument(&token_identifier)
            .original_result()
    }
}
impl<Env, From, To, Gas> CrowdfundingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn fund(
        self,
    ) -> Tx<Env, From, To, (), Gas, FunctionCall<Env::Api>, OriginalResultMarker<()>> {
        self.wrapped_tx
            .raw_call()
            .function_name("fund")
            .original_result()
    }

    pub fn status(
        self,
    ) -> Tx<Env, From, To, (), Gas, FunctionCall<Env::Api>, OriginalResultMarker<Status>> {
        self.wrapped_tx
            .raw_call()
            .function_name("status")
            .original_result()
    }

    pub fn get_current_funds(
        self,
    ) -> Tx<Env, From, To, (), Gas, FunctionCall<Env::Api>, OriginalResultMarker<BigUint<Env::Api>>>
    {
        self.wrapped_tx
            .raw_call()
            .function_name("getCurrentFunds")
            .original_result()
    }

    pub fn claim(
        self,
    ) -> Tx<Env, From, To, (), Gas, FunctionCall<Env::Api>, OriginalResultMarker<()>> {
        self.wrapped_tx
            .raw_call()
            .function_name("claim")
            .original_result()
    }

    pub fn target(
        self,
    ) -> Tx<Env, From, To, (), Gas, FunctionCall<Env::Api>, OriginalResultMarker<BigUint<Env::Api>>>
    {
        self.wrapped_tx
            .raw_call()
            .function_name("getTarget")
            .original_result()
    }

    pub fn deadline(
        self,
    ) -> Tx<Env, From, To, (), Gas, FunctionCall<Env::Api>, OriginalResultMarker<u64>> {
        self.wrapped_tx
            .raw_call()
            .function_name("getDeadline")
            .original_result()
    }

    pub fn deposit<Arg0: CodecInto<ManagedAddress<Env::Api>>>(
        self,
        donor: Arg0,
    ) -> Tx<Env, From, To, (), Gas, FunctionCall<Env::Api>, OriginalResultMarker<BigUint<Env::Api>>>
    {
        self.wrapped_tx
            .raw_call()
            .function_name("getDeposit")
            .argument(&donor)
            .original_result()
    }

    pub fn cf_token_identifier(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<EgldOrEsdtTokenIdentifier<Env::Api>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getCrowdfundingTokenIdentifier")
            .original_result()
    }
}
#[derive(TopEncode, TopDecode, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}
