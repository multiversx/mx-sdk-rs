////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct DigitalCashProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for DigitalCashProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = DigitalCashProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        DigitalCashProxyMethods { wrapped_tx: tx }
    }
}

pub struct DigitalCashProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> DigitalCashProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<BigUint<Env::Api>>,
        Arg1: CodecInto<EgldOrEsdtTokenIdentifier<Env::Api>>,
    >(
        self,
        fee: Arg0,
        token: Arg1,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&fee)
            .argument(&token)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> DigitalCashProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn whitelist_fee_token<
        Arg0: CodecInto<BigUint<Env::Api>>,
        Arg1: CodecInto<EgldOrEsdtTokenIdentifier<Env::Api>>,
    >(
        self,
        fee: Arg0,
        token: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("whitelistFeeToken")
            .argument(&fee)
            .argument(&token)
            .original_result()
    }

    pub fn blacklist_fee_token<
        Arg0: CodecInto<EgldOrEsdtTokenIdentifier<Env::Api>>,
    >(
        self,
        token: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("blacklistFeeToken")
            .argument(&token)
            .original_result()
    }

    pub fn claim_fees(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("claimFees")
            .original_result()
    }

    pub fn get_amount<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: CodecInto<u64>,
    >(
        self,
        address: Arg0,
        token: Arg1,
        nonce: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call()
            .function_name("getAmount")
            .argument(&address)
            .argument(&token)
            .argument(&nonce)
            .original_result()
    }

    pub fn pay_fee_and_fund_esdt<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<u64>,
    >(
        self,
        address: Arg0,
        valability: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("payFeeAndFundESDT")
            .argument(&address)
            .argument(&valability)
            .original_result()
    }

    pub fn pay_fee_and_fund_egld<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<u64>,
    >(
        self,
        address: Arg0,
        valability: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("payFeeAndFundEGLD")
            .argument(&address)
            .argument(&valability)
            .original_result()
    }

    pub fn fund<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<u64>,
    >(
        self,
        address: Arg0,
        valability: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("fund")
            .argument(&address)
            .argument(&valability)
            .original_result()
    }

    pub fn deposit_fees<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("depositFees")
            .argument(&address)
            .original_result()
    }

    pub fn withdraw<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("withdraw")
            .argument(&address)
            .original_result()
    }

    pub fn claim<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedByteArray<Env::Api, 64usize>>,
    >(
        self,
        address: Arg0,
        signature: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("claim")
            .argument(&address)
            .argument(&signature)
            .original_result()
    }

    pub fn forward<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<ManagedByteArray<Env::Api, 64usize>>,
    >(
        self,
        address: Arg0,
        forward_address: Arg1,
        signature: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("forward")
            .argument(&address)
            .argument(&forward_address)
            .argument(&signature)
            .original_result()
    }

    pub fn deposit<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        donor: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, DepositInfo<Env::Api>> {
        self.wrapped_tx
            .raw_call()
            .function_name("deposit")
            .argument(&donor)
            .original_result()
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DepositInfo<Api>
where
    Api: ManagedTypeApi,
{
    pub depositor_address: ManagedAddress<Api>,
    pub esdt_funds: ManagedVec<Api, EsdtTokenPayment<Api>>,
    pub egld_funds: BigUint<Api>,
    pub valability: u64,
    pub expiration_round: u64,
    pub fees: Fee<Api>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Fee<Api>
where
    Api: ManagedTypeApi,
{
    pub num_token_to_transfer: usize,
    pub value: EgldOrEsdtTokenPayment<Api>,
}