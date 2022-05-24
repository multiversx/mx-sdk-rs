use crate::{
    api::{BlockchainApi, CallTypeApi, StorageReadApi},
    esdt::ESDTSystemSmartContractProxy,
    types::{
        BigUint, ContractCall, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedVec, TokenIdentifier,
    },
};

use super::SendRawWrapper;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
// pub trait SendApi: Clone + Sized {

#[derive(Default)]
pub struct SendWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    send_raw_wrapper: SendRawWrapper<A>,
}

impl<A> SendWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    pub(crate) fn new() -> Self {
        SendWrapper {
            send_raw_wrapper: SendRawWrapper::new(),
        }
    }

    pub fn esdt_system_sc_proxy(&self) -> ESDTSystemSmartContractProxy<A> {
        self.send_raw_wrapper.esdt_system_sc_proxy()
    }

    pub fn contract_call<R>(
        &self,
        to: ManagedAddress<A>,
        endpoint_name: ManagedBuffer<A>,
    ) -> ContractCall<A, R> {
        self.send_raw_wrapper.contract_call(to, endpoint_name)
    }

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, amount: &BigUint<A>, data: D)
    where
        D: Into<ManagedBuffer<A>>,
    {
        self.send_raw_wrapper.direct_egld(to, amount, data)
    }

    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce
    pub fn direct<D>(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        data: D,
    ) where
        D: Into<ManagedBuffer<A>>,
    {
        self.send_raw_wrapper.direct(to, token, nonce, amount, data);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn direct_with_gas_limit<D>(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<A>],
    ) where
        D: Into<ManagedBuffer<A>>,
    {
        self.send_raw_wrapper.direct_with_gas_limit(
            to,
            token,
            nonce,
            amount,
            gas,
            endpoint_name,
            arguments,
        )
    }

    pub fn direct_multi<D>(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        data: D,
    ) where
        D: Into<ManagedBuffer<A>>,
    {
        self.send_raw_wrapper.direct_multi(to, payments, data)
    }

    /// Performs a simple ESDT/NFT transfer, but via async call.  
    /// As with any async call, this immediately terminates the execution of the current call.  
    /// So only use as the last call in your endpoint.  
    /// If you want to perform multiple transfers, use `self.send().transfer_multiple_esdt_via_async_call()` instead.  
    /// Note that EGLD can NOT be transfered with this function.  
    pub fn transfer_esdt_via_async_call<D>(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        data: D,
    ) -> !
    where
        D: Into<ManagedBuffer<A>>,
    {
        self.send_raw_wrapper
            .transfer_esdt_via_async_call(to, token, nonce, amount, data)
    }

    pub fn transfer_multiple_esdt_via_async_call<D>(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        data: D,
    ) -> !
    where
        D: Into<ManagedBuffer<A>>,
    {
        self.send_raw_wrapper
            .transfer_multiple_esdt_via_async_call(to, payments, data)
    }

    /// Sends a synchronous call to change a smart contract address.
    pub fn change_owner_address(
        &self,
        child_sc_address: ManagedAddress<A>,
        new_owner: &ManagedAddress<A>,
    ) -> ContractCall<A, ()> {
        self.send_raw_wrapper
            .change_owner_address(child_sc_address, new_owner)
    }

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    pub fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        self.send_raw_wrapper
            .call_local_esdt_built_in_function(gas, endpoint_name, arg_buffer)
    }

    /// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
    /// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
    /// or this will fail with "action is not allowed"
    /// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
    /// This function cannot be used for NFTs.
    pub fn esdt_local_mint(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
        self.send_raw_wrapper.esdt_local_mint(token, nonce, amount)
    }

    /// Allows synchronous burning of ESDT/SFT/NFT (depending on nonce). Execution is resumed afterwards.
    /// Note that the SC must have the ESDTLocalBurn or ESDTNftBurn roles set,
    /// or this will fail with "action is not allowed"
    pub fn esdt_local_burn(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
        self.send_raw_wrapper.esdt_local_burn(token, nonce, amount)
    }

    /// Creates a new NFT token of a certain type (determined by `token_identifier`).  
    /// `attributes` can be any serializable custom struct.  
    /// This is a built-in function, so the smart contract execution is resumed after.
    /// Must have ESDTNftCreate role set, or this will fail with "action is not allowed".
    /// Returns the nonce of the newly created NFT.
    #[allow(clippy::too_many_arguments)]
    pub fn esdt_nft_create<T: elrond_codec::TopEncode>(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
        royalties: &BigUint<A>,
        hash: &ManagedBuffer<A>,
        attributes: &T,
        uris: &ManagedVec<A, ManagedBuffer<A>>,
    ) -> u64 {
        self.send_raw_wrapper
            .esdt_nft_create(token, amount, name, royalties, hash, attributes, uris)
    }

    #[inline]
    pub fn esdt_nft_create_compact<T: elrond_codec::TopEncode>(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        attributes: &T,
    ) -> u64 {
        self.send_raw_wrapper
            .esdt_nft_create_compact(token, amount, attributes)
    }

    pub fn esdt_nft_create_compact_named<T: elrond_codec::TopEncode>(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
        attributes: &T,
    ) -> u64 {
        self.send_raw_wrapper
            .esdt_nft_create_compact_named(token, amount, name, attributes)
    }

    /// Creates an NFT on behalf of the caller. This will set the "creator" field to the caller's address
    /// NOT activated on devnet/mainnet yet.
    #[allow(clippy::too_many_arguments)]
    pub fn esdt_nft_create_as_caller<T: elrond_codec::TopEncode>(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
        royalties: &BigUint<A>,
        hash: &ManagedBuffer<A>,
        attributes: &T,
        uris: &ManagedVec<A, ManagedBuffer<A>>,
    ) -> u64 {
        self.send_raw_wrapper
            .esdt_nft_create_as_caller(token, amount, name, royalties, hash, attributes, uris)
    }

    /// Sends thr NFTs to the buyer address and calculates and sends the required royalties to the NFT creator.
    /// Returns the payment amount left after sending royalties.
    #[allow(clippy::too_many_arguments)]
    pub fn sell_nft(
        &self,
        nft_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        nft_amount: &BigUint<A>,
        buyer: &ManagedAddress<A>,
        payment_token: &TokenIdentifier<A>,
        payment_nonce: u64,
        payment_amount: &BigUint<A>,
    ) -> BigUint<A> {
        self.send_raw_wrapper.sell_nft(
            nft_id,
            nft_nonce,
            nft_amount,
            buyer,
            payment_token,
            payment_nonce,
            payment_amount,
        )
    }

    pub fn nft_add_uri(
        &self,
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_uri: ManagedBuffer<A>,
    ) {
        self.send_raw_wrapper
            .nft_add_uri(token_id, nft_nonce, new_uri)
    }

    pub fn nft_add_multiple_uri(
        &self,
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_uris: &ManagedVec<A, ManagedBuffer<A>>,
    ) {
        self.send_raw_wrapper
            .nft_add_multiple_uri(token_id, nft_nonce, new_uris)
    }

    pub fn nft_update_attributes<T: elrond_codec::TopEncode>(
        &self,
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_attributes: &T,
    ) {
        self.send_raw_wrapper
            .nft_update_attributes(token_id, nft_nonce, new_attributes)
    }
}
