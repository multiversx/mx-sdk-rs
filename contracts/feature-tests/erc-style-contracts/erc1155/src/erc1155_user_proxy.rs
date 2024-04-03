#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct Erc1155UserProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for Erc1155UserProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = Erc1155UserProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        Erc1155UserProxyMethods { wrapped_tx: tx }
    }
}

pub struct Erc1155UserProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, To, Gas> Erc1155UserProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn on_erc1155_received<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<BigUint<Env::Api>>,
        Arg3: CodecInto<BigUint<Env::Api>>,
        Arg4: CodecInto<ManagedBuffer<Env::Api>>,
    >(
        self,
        operator: Arg0,
        from: Arg1,
        type_id: Arg2,
        value: Arg3,
        data: Arg4,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("onERC1155Received")
            .argument(&operator)
            .argument(&from)
            .argument(&type_id)
            .argument(&value)
            .argument(&data)
            .original_result()
    }

    pub fn on_erc1155_batch_received<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<Vec<BigUint<Env::Api>>>,
        Arg3: CodecInto<Vec<BigUint<Env::Api>>>,
        Arg4: CodecInto<ManagedBuffer<Env::Api>>,
    >(
        self,
        operator: Arg0,
        from: Arg1,
        type_ids: Arg2,
        values: Arg3,
        data: Arg4,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("onERC1155BatchReceived")
            .argument(&operator)
            .argument(&from)
            .argument(&type_ids)
            .argument(&values)
            .argument(&data)
            .original_result()
    }
}
