#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

use crate::{Kitty, KittyGenes};

pub struct KittyGeneticAlgProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for KittyGeneticAlgProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = KittyGeneticAlgProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        KittyGeneticAlgProxyMethods { wrapped_tx: tx }
    }
}

pub struct KittyGeneticAlgProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, To, Gas> KittyGeneticAlgProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn generate_kitty_genes<
        Arg0: CodecInto<Kitty>,
        Arg1: CodecInto<Kitty>,
    >(
        self,
        matron: Arg0,
        sire: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, KittyGenes> {
        self.wrapped_tx
            .raw_call()
            .function_name("generateKittyGenes")
            .argument(&matron)
            .argument(&sire)
            .original_result()
    }
}
