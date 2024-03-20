////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(clippy::all)]

use multiversx_sc::imports::*;

pub struct CryptoZombiesProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for CryptoZombiesProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = CryptoZombiesProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        CryptoZombiesProxyMethods { wrapped_tx: tx }
    }
}

pub struct CryptoZombiesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, Gas> CryptoZombiesProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> Tx<
        Env,
        From,
        (),
        (),
        Gas,
        DeployCall<Env, ()>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_deploy()
            .original_result()
    }

}
impl<Env, From, To, Gas> CryptoZombiesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_crypto_kitties_sc_address<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("set_crypto_kitties_sc_address")
            .argument(&address)
            .original_result()
    }

    pub fn generate_random_dna(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<u64>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("generate_random_dna")
            .original_result()
    }

    pub fn create_random_zombie<
        Arg0: CodecInto<ManagedBuffer<Env::Api>>,
    >(
        self,
        name: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("create_random_zombie")
            .argument(&name)
            .original_result()
    }

    pub fn is_ready<
        Arg0: CodecInto<usize>,
    >(
        self,
        zombie_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<bool>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("is_ready")
            .argument(&zombie_id)
            .original_result()
    }

    pub fn feed_on_kitty<
        Arg0: CodecInto<usize>,
        Arg1: CodecInto<u32>,
    >(
        self,
        zombie_id: Arg0,
        kitty_id: Arg1,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("feed_on_kitty")
            .argument(&zombie_id)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn dna_digits(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<u8>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("dna_digits")
            .original_result()
    }

    pub fn zombies_count(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("zombies_count")
            .original_result()
    }

    pub fn zombies<
        Arg0: CodecInto<usize>,
    >(
        self,
        id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<Zombie<Env::Api>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("zombies")
            .argument(&id)
            .original_result()
    }

    pub fn zombie_owner<
        Arg0: CodecInto<usize>,
    >(
        self,
        id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<ManagedAddress<Env::Api>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("zombie_owner")
            .argument(&id)
            .original_result()
    }

    pub fn crypto_kitties_sc_address(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<ManagedAddress<Env::Api>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("crypto_kitties_sc_address")
            .original_result()
    }

    pub fn cooldown_time(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<u64>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("cooldown_time")
            .original_result()
    }

    pub fn owned_zombies<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        owner: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<MultiValueEncoded<Env::Api, usize>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("owned_zombies")
            .argument(&owner)
            .original_result()
    }

    pub fn level_up<
        Arg0: CodecInto<usize>,
    >(
        self,
        zombie_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("level_up")
            .argument(&zombie_id)
            .original_result()
    }

    pub fn withdraw(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("withdraw")
            .original_result()
    }

    pub fn change_name<
        Arg0: CodecInto<usize>,
        Arg1: CodecInto<ManagedBuffer<Env::Api>>,
    >(
        self,
        zombie_id: Arg0,
        name: Arg1,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("change_name")
            .argument(&zombie_id)
            .argument(&name)
            .original_result()
    }

    pub fn change_dna<
        Arg0: CodecInto<usize>,
        Arg1: CodecInto<u64>,
    >(
        self,
        zombie_id: Arg0,
        dna: Arg1,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("change_dna")
            .argument(&zombie_id)
            .argument(&dna)
            .original_result()
    }

    pub fn attack<
        Arg0: CodecInto<usize>,
        Arg1: CodecInto<usize>,
    >(
        self,
        zombie_id: Arg0,
        target_id: Arg1,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("attack")
            .argument(&zombie_id)
            .argument(&target_id)
            .original_result()
    }

}
use multiversx_sc::derive_imports::*;

#[derive(TypeAbi)]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Zombie<Api>
where
    Api: ManagedTypeApi,
{
    pub name: ManagedBuffer<Api>,
    pub dna: u64,
    pub level: u16,
    pub ready_time: u64,
    pub win_count: usize,
    pub loss_count: usize,
}

