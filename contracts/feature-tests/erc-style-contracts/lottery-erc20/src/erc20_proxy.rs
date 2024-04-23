// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct SimpleErc20TokenProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for SimpleErc20TokenProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = SimpleErc20TokenProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        SimpleErc20TokenProxyMethods { wrapped_tx: tx }
    }
}

pub struct SimpleErc20TokenProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> SimpleErc20TokenProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    /// Constructor, is called immediately after the contract is created 
    /// Will set the fixed global token supply and give all the supply to the creator. 
    pub fn init<
        Arg0: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        total_supply: Arg0,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&total_supply)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> SimpleErc20TokenProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Total number of tokens in existence. 
    pub fn total_supply(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("totalSupply")
            .original_result()
    }

    /// Gets the balance of the specified address. 
    ///  
    /// Arguments: 
    ///  
    /// * `address` The address to query the the balance of 
    ///  
    pub fn token_balance<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("balanceOf")
            .argument(&address)
            .original_result()
    }

    /// The amount of tokens that an owner allowed to a spender. 
    ///  
    /// Arguments: 
    ///  
    /// * `owner` The address that owns the funds. 
    /// * `spender` The address that will spend the funds. 
    ///  
    pub fn allowance<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        owner: Arg0,
        spender: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("allowance")
            .argument(&owner)
            .argument(&spender)
            .original_result()
    }

    /// Transfer token to a specified address from sender. 
    ///  
    /// Arguments: 
    ///  
    /// * `to` The address to transfer to. 
    ///  
    pub fn transfer<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        amount: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("transfer")
            .argument(&to)
            .argument(&amount)
            .original_result()
    }

    /// Use allowance to transfer funds between two accounts. 
    ///  
    /// Arguments: 
    ///  
    /// * `sender` The address to transfer from. 
    /// * `recipient` The address to transfer to. 
    /// * `amount` the amount of tokens to be transferred. 
    ///  
    pub fn transfer_from<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        sender: Arg0,
        recipient: Arg1,
        amount: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("transferFrom")
            .argument(&sender)
            .argument(&recipient)
            .argument(&amount)
            .original_result()
    }

    /// Approve the given address to spend the specified amount of tokens on behalf of the sender. 
    /// It overwrites any previously existing allowance from sender to beneficiary. 
    ///  
    /// Arguments: 
    ///  
    /// * `spender` The address that will spend the funds. 
    /// * `amount` The amount of tokens to be spent. 
    ///  
    pub fn approve<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        spender: Arg0,
        amount: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("approve")
            .argument(&spender)
            .argument(&amount)
            .original_result()
    }
}
