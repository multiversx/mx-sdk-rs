use core::marker::PhantomData;

use crate::codec::Empty;

use crate::{
    api::{
        BlockchainApi, BlockchainApiImpl, CallTypeApi, StorageReadApi,
        CHANGE_OWNER_BUILTIN_FUNC_NAME, CLAIM_DEVELOPER_REWARDS_FUNC_NAME,
        ESDT_LOCAL_BURN_FUNC_NAME, ESDT_LOCAL_MINT_FUNC_NAME, ESDT_NFT_ADD_QUANTITY_FUNC_NAME,
        ESDT_NFT_ADD_URI_FUNC_NAME, ESDT_NFT_BURN_FUNC_NAME, ESDT_NFT_CREATE_FUNC_NAME,
        ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME,
    },
    codec,
    esdt::ESDTSystemSmartContractProxy,
    types::{
        BigUint, ContractCall, ContractCallNoPayment, EgldOrEsdtTokenIdentifier, EsdtTokenPayment,
        ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedType, ManagedVec, TokenIdentifier,
    },
};

use super::BlockchainWrapper;

const PERCENTAGE_TOTAL: u64 = 10_000;

use super::SendRawWrapper;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
// pub trait SendApi: Clone + Sized {

#[derive(Default)]
pub struct SendWrapper<'a, A>
where
    A: CallTypeApi<'a> + StorageReadApi + BlockchainApi<'a>,
{
    _phantom: PhantomData<A>,
}

impl<'a, A> SendWrapper<'a, A>
where
    A: CallTypeApi<'a> + StorageReadApi + BlockchainApi<'a>,
{
    pub fn new() -> Self {
        SendWrapper {
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn send_raw_wrapper(&self) -> SendRawWrapper<'a, A> {
        SendRawWrapper::new()
    }

    /// A proxy for calling the system smart contract.
    ///
    /// Use the methods of this proxy to launch contract calls to the system SC.
    #[inline]
    pub fn esdt_system_sc_proxy(&self) -> ESDTSystemSmartContractProxy<'a, A> {
        ESDTSystemSmartContractProxy::new_proxy_obj()
    }

    /// Convenient way to quickly instance a minimal contract call (with no EGLD, no arguments, etc.)
    ///
    /// You can further configure this contract call by chaining methods to it.
    #[inline]
    pub fn contract_call<R>(
        &self,
        to: ManagedAddress<'a, A>,
        endpoint_name: impl Into<ManagedBuffer<'a, A>>,
    ) -> ContractCallNoPayment<'a, A, R> {
        ContractCallNoPayment::new(to, endpoint_name)
    }

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    #[inline]
    pub fn direct_egld(&self, to: &ManagedAddress<'a, A>, amount: &BigUint<'a, A>) {
        self.send_raw_wrapper().direct_egld(to, amount, Empty)
    }

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    ///
    /// If the amount is 0, it returns without error.
    pub fn direct_non_zero_egld(&self, to: &ManagedAddress<'a, A>, amount: &BigUint<'a, A>) {
        if amount == &0 {
            return;
        }

        self.direct_egld(to, amount)
    }

    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce
    #[inline]
    pub fn direct(
        &self,
        to: &ManagedAddress<'a, A>,
        token: &EgldOrEsdtTokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
    ) {
        self.direct_with_gas_limit(to, token, nonce, amount, 0, Empty, &[]);
    }

    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce.
    ///
    /// If the amount is 0, it returns without error.
    #[inline]
    pub fn direct_non_zero(
        &self,
        to: &ManagedAddress<'a, A>,
        token: &EgldOrEsdtTokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
    ) {
        self.direct_non_zero_with_gas_limit(to, token, nonce, amount, 0, Empty, &[]);
    }

    /// Sends a single ESDT transfer, and calls an endpoint at the destination.
    ///
    /// Avoid if possible, use a contract call with ESDT transfer instead, and call `.transfer_execute()` on it.
    #[allow(clippy::too_many_arguments)]
    pub fn direct_esdt_with_gas_limit<D>(
        &self,
        to: &ManagedAddress<'a, A>,
        token_identifier: &TokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<'a, A>],
    ) where
        D: Into<ManagedBuffer<'a, A>>,
    {
        if nonce == 0 {
            let _ = self.send_raw_wrapper().transfer_esdt_execute(
                to,
                token_identifier,
                amount,
                gas,
                &endpoint_name.into(),
                &arguments.into(),
            );
        } else {
            let _ = self.send_raw_wrapper().transfer_esdt_nft_execute(
                to,
                token_identifier,
                nonce,
                amount,
                gas,
                &endpoint_name.into(),
                &arguments.into(),
            );
        }
    }

    /// Sends a single ESDT transfer, and calls an endpoint at the destination.
    ///
    /// If the amount is 0, it returns without error.
    ///
    /// Avoid if possible, use a contract call with ESDT transfer instead, and call `.transfer_execute()` on it.
    #[allow(clippy::too_many_arguments)]
    pub fn direct_non_zero_esdt_with_gas_limit<D>(
        &self,
        to: &ManagedAddress<'a, A>,
        token_identifier: &TokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<'a, A>],
    ) where
        D: Into<ManagedBuffer<'a, A>>,
    {
        if amount == &0 {
            return;
        }
        self.direct_esdt_with_gas_limit(
            to,
            token_identifier,
            nonce,
            amount,
            gas,
            endpoint_name,
            arguments,
        );
    }

    /// Sends a single ESDT transfer to target address.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn direct_esdt(
        &self,
        to: &ManagedAddress<'a, A>,
        token_identifier: &TokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
    ) {
        self.direct_esdt_with_gas_limit(to, token_identifier, nonce, amount, 0, Empty, &[]);
    }

    /// Sends a single ESDT transfer to target address.
    ///
    /// If the amount is 0, it returns without error.
    pub fn direct_non_zero_esdt_payment(
        &self,
        to: &ManagedAddress<'a, A>,
        payment: &EsdtTokenPayment<'a, A>,
    ) {
        if payment.amount == 0 {
            return;
        }

        self.direct_esdt(
            to,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce.
    /// Also and calls an endpoint at the destination.
    ///
    /// Avoid if possible, use a contract call with ESDT transfer instead, and call `.transfer_execute()` on it.
    #[allow(clippy::too_many_arguments)]
    pub fn direct_with_gas_limit<D>(
        &self,
        to: &ManagedAddress<'a, A>,
        token: &EgldOrEsdtTokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<'a, A>],
    ) where
        D: Into<ManagedBuffer<'a, A>>,
    {
        if let Some(esdt_token_identifier) = token.as_esdt_option() {
            self.direct_esdt_with_gas_limit(
                to,
                &esdt_token_identifier,
                nonce,
                amount,
                gas,
                endpoint_name,
                arguments,
            );
        } else {
            let _ = self.send_raw_wrapper().direct_egld_execute(
                to,
                amount,
                gas,
                &endpoint_name.into(),
                &arguments.into(),
            );
        }
    }
    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce.
    /// Also and calls an endpoint at the destination.
    ///
    /// If the amount is 0, it returns without error.
    ///
    /// Avoid if possible, use a contract call with ESDT transfer instead, and call `.transfer_execute()` on it.
    #[allow(clippy::too_many_arguments)]
    pub fn direct_non_zero_with_gas_limit<D>(
        &self,
        to: &ManagedAddress<'a, A>,
        token: &EgldOrEsdtTokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<'a, A>],
    ) where
        D: Into<ManagedBuffer<'a, A>>,
    {
        if amount == &0 {
            return;
        }
        self.direct_with_gas_limit(to, token, nonce, amount, gas, endpoint_name, arguments);
    }

    /// Sends multiple ESDT tokens to a target address.
    pub fn direct_multi(
        &self,
        to: &ManagedAddress<'a, A>,
        payments: &ManagedVec<'a, A, EsdtTokenPayment<'a, A>>,
    ) {
        let _ = self.send_raw_wrapper().multi_esdt_transfer_execute(
            to,
            payments,
            0,
            &ManagedBuffer::new(),
            &ManagedArgBuffer::new(),
        );
    }

    /// Performs a simple ESDT/NFT transfer, but via async call.  
    ///
    /// As with any async call, this immediately terminates the execution of the current call,
    /// so only use as the last call in your endpoint.
    ///
    /// If you want to perform multiple transfers, use `self.send().transfer_multiple_esdt_via_async_call()` instead.
    ///
    /// Note that EGLD can NOT be transfered with this function.  
    pub fn transfer_esdt_via_async_call(
        &self,
        to: ManagedAddress<'a, A>,
        token: TokenIdentifier<'a, A>,
        nonce: u64,
        amount: BigUint<'a, A>,
    ) -> ! {
        ContractCallNoPayment::<'a, A, ()>::new(to, ManagedBuffer::new())
            .with_esdt_transfer((token, nonce, amount))
            .async_call()
            .call_and_exit_ignore_callback()
    }

    /// Performs a simple ESDT/NFT transfer, but via async call.  
    ///
    /// As with any async call, this immediately terminates the execution of the current call,
    /// so only use as the last call in your endpoint.
    ///
    /// If you want to perform multiple transfers, use `self.send().transfer_multiple_esdt_via_async_call()` instead.  
    /// Note that EGLD can NOT be transfered with this function.
    ///
    /// If the amount is 0, it returns without error.
    pub fn transfer_esdt_non_zero_via_async_call(
        &self,
        to: ManagedAddress<'a, A>,
        token: TokenIdentifier<'a, A>,
        nonce: u64,
        amount: BigUint<'a, A>,
    ) {
        if amount == 0 {
            return;
        }
        ContractCallNoPayment::<'a, A, ()>::new(to, ManagedBuffer::new())
            .with_esdt_transfer((token, nonce, amount))
            .async_call()
            .call_and_exit_ignore_callback()
    }

    /// Sends multiple ESDT tokens to a target address, via an async call.
    pub fn transfer_multiple_esdt_via_async_call(
        &self,
        to: ManagedAddress<'a, A>,
        payments: ManagedVec<'a, A, EsdtTokenPayment<'a, A>>,
    ) -> ! {
        ContractCallNoPayment::<'a, A, ()>::new(to, ManagedBuffer::new())
            .with_multi_token_transfer(payments)
            .async_call()
            .call_and_exit_ignore_callback()
    }

    /// Creates a call to the `ClaimDeveloperRewards` builtin function.
    ///
    /// In itself, this does nothing. You need to then call turn the contract call into an async call.
    pub fn claim_developer_rewards(
        &self,
        child_sc_address: ManagedAddress<'a, A>,
    ) -> ContractCallNoPayment<'a, A, ()> {
        ContractCallNoPayment::new(child_sc_address, CLAIM_DEVELOPER_REWARDS_FUNC_NAME)
    }

    /// Creates a call to the `ChangeOwnerAddress` builtin function.
    ///
    /// In itself, this does nothing. You need to then call turn the contract call into an async call.
    pub fn change_owner_address(
        &self,
        child_sc_address: ManagedAddress<'a, A>,
        new_owner: &ManagedAddress<'a, A>,
    ) -> ContractCallNoPayment<'a, A, ()> {
        self.contract_call(child_sc_address, CHANGE_OWNER_BUILTIN_FUNC_NAME)
            .argument(&new_owner)
    }

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    pub fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        endpoint_name: &ManagedBuffer<'a, A>,
        arg_buffer: &ManagedArgBuffer<'a, A>,
    ) -> ManagedVec<'a, A, ManagedBuffer<'a, A>> {
        self.send_raw_wrapper()
            .call_local_esdt_built_in_function(gas, endpoint_name, arg_buffer)
    }

    /// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
    /// or this will fail with "action is not allowed".
    ///
    /// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
    ///
    /// This function cannot be used for NFTs.
    pub fn esdt_local_mint(&self, token: &TokenIdentifier<'a, A>, nonce: u64, amount: &BigUint<'a, A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name: &str;

        arg_buffer.push_arg(token);

        if nonce == 0 {
            func_name = ESDT_LOCAL_MINT_FUNC_NAME;
        } else {
            func_name = ESDT_NFT_ADD_QUANTITY_FUNC_NAME;
            arg_buffer.push_arg(nonce);
        }

        arg_buffer.push_arg(amount);

        let _ = self.call_local_esdt_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            &ManagedBuffer::from(func_name),
            &arg_buffer,
        );
    }

    /// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
    /// or this will fail with "action is not allowed".
    ///
    /// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
    /// This function cannot be used for NFTs.
    ///
    /// If the amount is 0, it returns without error.
    pub fn esdt_non_zero_local_mint(
        &self,
        token: &TokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
    ) {
        if amount == &0 {
            return;
        }
        self.esdt_local_mint(token, nonce, amount);
    }

    /// Allows synchronous burning of ESDT/SFT/NFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the ESDTLocalBurn or ESDTNftBurn roles set,
    /// or this will fail with "action is not allowed".
    pub fn esdt_local_burn(&self, token: &TokenIdentifier<'a, A>, nonce: u64, amount: &BigUint<'a, A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name: &str;

        arg_buffer.push_arg(token);
        if nonce == 0 {
            func_name = ESDT_LOCAL_BURN_FUNC_NAME;
        } else {
            func_name = ESDT_NFT_BURN_FUNC_NAME;
            arg_buffer.push_arg(nonce);
        }

        arg_buffer.push_arg(amount);

        let _ = self.call_local_esdt_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            &ManagedBuffer::from(func_name),
            &arg_buffer,
        );
    }

    /// Allows synchronous burning of ESDT/SFT/NFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the ESDTLocalBurn or ESDTNftBurn roles set,
    /// or this will fail with "action is not allowed".
    ///
    /// If the amount is 0, it returns without error.
    pub fn esdt_non_zero_local_burn(
        &self,
        token: &TokenIdentifier<'a, A>,
        nonce: u64,
        amount: &BigUint<'a, A>,
    ) {
        if amount == &0 {
            return;
        }
        self.esdt_local_burn(token, nonce, amount);
    }

    /// Allows burning of multiple ESDT tokens at once.
    ///
    /// Will execute a synchronous call to the appropriate burn builtin function for each.
    pub fn esdt_local_burn_multi(&self, payments: &ManagedVec<'a, A, EsdtTokenPayment<'a, A>>) {
        for payment in payments {
            self.esdt_local_burn(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );
        }
    }

    /// Allows burning of multiple ESDT tokens at once.
    ///
    /// Will execute a synchronous call to the appropriate burn builtin function for each.
    ///
    /// If any of the token amounts is 0 skips that token without throwing error.
    pub fn esdt_non_zero_local_burn_multi(&self, payments: &ManagedVec<'a, A, EsdtTokenPayment<'a, A>>) {
        for payment in payments {
            self.esdt_non_zero_local_burn(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );
        }
    }

    /// Creates a new NFT token of a certain type (determined by `token_identifier`).
    /// `attributes` can be any serializable custom struct.  
    ///
    /// This is a synchronous built-in function call, so the smart contract execution is resumed afterwards.
    ///
    /// Must have ESDTNftCreate role set, or this will fail with "action is not allowed".
    ///
    /// Returns the nonce of the newly created NFT.
    #[allow(clippy::too_many_arguments)]
    pub fn esdt_nft_create<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<'a, A>,
        amount: &BigUint<'a, A>,
        name: &ManagedBuffer<'a, A>,
        royalties: &BigUint<'a, A>,
        hash: &ManagedBuffer<'a, A>,
        attributes: &T,
        uris: &ManagedVec<'a, A, ManagedBuffer<'a, A>>,
    ) -> u64 {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(name);
        arg_buffer.push_arg(royalties);
        arg_buffer.push_arg(hash);
        arg_buffer.push_arg(attributes);

        if uris.is_empty() {
            // at least one URI is required, so we push an empty one
            arg_buffer.push_arg(codec::Empty);
        } else {
            // The API function has the last argument as variadic,
            // so we top-encode each and send as separate argument
            for uri in uris {
                arg_buffer.push_arg(uri);
            }
        }

        let output = self.call_local_esdt_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            &ManagedBuffer::from(ESDT_NFT_CREATE_FUNC_NAME),
            &arg_buffer,
        );

        if let Some(first_result_bytes) = output.try_get(0) {
            first_result_bytes.parse_as_u64().unwrap_or_default()
        } else {
            0
        }
    }

    /// Creates a new NFT token of a certain type (determined by `token_identifier`).
    ///
    /// `attributes` can be any serializable custom struct.
    ///
    /// This is a built-in function, so the smart contract execution is resumed after.
    /// Must have ESDTNftCreate role set, or this will fail with "action is not allowed".
    ///
    /// Returns the nonce of the newly created NFT.
    ///
    /// If the amount is 0, it returns without error.
    #[allow(clippy::too_many_arguments)]
    pub fn esdt_non_zero_nft_create<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<'a, A>,
        amount: &BigUint<'a, A>,
        name: &ManagedBuffer<'a, A>,
        royalties: &BigUint<'a, A>,
        hash: &ManagedBuffer<'a, A>,
        attributes: &T,
        uris: &ManagedVec<'a, A, ManagedBuffer<'a, A>>,
    ) -> u64 {
        if amount == &0 {
            0
        } else {
            self.esdt_nft_create(token, amount, name, royalties, hash, attributes, uris)
        }
    }

    /// Quick way of creating a new NFT token instance.
    ///
    /// Returns the new NFT nonce.
    #[inline]
    pub fn esdt_nft_create_compact<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<'a, A>,
        amount: &BigUint<'a, A>,
        attributes: &T,
    ) -> u64 {
        self.esdt_nft_create_compact_named(token, amount, &ManagedBuffer::new(), attributes)
    }

    /// Quick way of creating a new NFT token instance, with custom name.
    ///
    /// Returns the new NFT nonce.
    pub fn esdt_nft_create_compact_named<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<'a, A>,
        amount: &BigUint<'a, A>,
        name: &ManagedBuffer<'a, A>,
        attributes: &T,
    ) -> u64 {
        let big_zero = BigUint::zero();
        let empty_buffer = ManagedBuffer::new();
        let empty_vec = ManagedVec::from_handle(empty_buffer.get_handle());

        self.esdt_nft_create(
            token,
            amount,
            name,
            &big_zero,
            &empty_buffer,
            attributes,
            &empty_vec,
        )
    }

    /// Quick way of creating a new NFT token instance.
    ///
    /// Returns the new NFT nonce.
    ///
    /// If the amount is 0, it returns without error.
    #[inline]
    pub fn esdt_non_zero_nft_create_compact<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<'a, A>,
        amount: &BigUint<'a, A>,
        attributes: &T,
    ) -> u64 {
        self.esdt_non_zero_nft_create_compact_named(
            token,
            amount,
            &ManagedBuffer::new(),
            attributes,
        )
    }

    /// Quick way of creating a new NFT token instance, with custom name.
    ///
    /// Returns the new NFT nonce.
    ///
    /// If the amount is 0, it returns without error.
    pub fn esdt_non_zero_nft_create_compact_named<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<'a, A>,
        amount: &BigUint<'a, A>,
        name: &ManagedBuffer<'a, A>,
        attributes: &T,
    ) -> u64 {
        if amount == &0 {
            0
        } else {
            self.esdt_nft_create_compact_named(token, amount, name, attributes)
        }
    }

    /// Sends the NFTs to the buyer address and calculates and sends the required royalties to the NFT creator.
    ///
    /// Returns the payment amount left after sending royalties.
    #[allow(clippy::too_many_arguments)]
    pub fn sell_nft(
        &self,
        nft_id: &TokenIdentifier<'a, A>,
        nft_nonce: u64,
        nft_amount: &BigUint<'a, A>,
        buyer: &ManagedAddress<'a, A>,
        payment_token: &EgldOrEsdtTokenIdentifier<'a, A>,
        payment_nonce: u64,
        payment_amount: &BigUint<'a, A>,
    ) -> BigUint<'a, A> {
        let nft_token_data = BlockchainWrapper::<'a, A>::new().get_esdt_token_data(
            &BlockchainWrapper::<'a, A>::new().get_sc_address(),
            nft_id,
            nft_nonce,
        );
        let royalties_amount = payment_amount.clone() * nft_token_data.royalties / PERCENTAGE_TOTAL;

        let _ = self.send_raw_wrapper().transfer_esdt_nft_execute(
            buyer,
            nft_id,
            nft_nonce,
            nft_amount,
            0,
            &ManagedBuffer::new(),
            &ManagedArgBuffer::new(),
        );

        if royalties_amount > 0u32 {
            self.direct(
                &nft_token_data.creator,
                payment_token,
                payment_nonce,
                &royalties_amount,
            );

            payment_amount.clone() - royalties_amount
        } else {
            payment_amount.clone()
        }
    }

    /// Sends the NFTs to the buyer address and calculates and sends the required royalties to the NFT creator.
    ///
    /// Returns the payment amount left after sending royalties.
    ///
    /// If the nft_amount or the payment_amount is 0 returns without error
    #[allow(clippy::too_many_arguments)]
    pub fn sell_nft_non_zero(
        &self,
        nft_id: &TokenIdentifier<'a, A>,
        nft_nonce: u64,
        nft_amount: &BigUint<'a, A>,
        buyer: &ManagedAddress<'a, A>,
        payment_token: &EgldOrEsdtTokenIdentifier<'a, A>,
        payment_nonce: u64,
        payment_amount: &BigUint<'a, A>,
    ) -> BigUint<'a, A> {
        if nft_amount == &0 || payment_amount == &0 {
            payment_amount.clone()
        } else {
            self.sell_nft(
                nft_id,
                nft_nonce,
                nft_amount,
                buyer,
                payment_token,
                payment_nonce,
                payment_amount,
            )
        }
    }

    /// Adds a new URI to an NFT, via a synchronous builtin function call.
    pub fn nft_add_uri(
        &self,
        token_id: &TokenIdentifier<'a, A>,
        nft_nonce: u64,
        new_uri: ManagedBuffer<'a, A>,
    ) {
        self.nft_add_multiple_uri(token_id, nft_nonce, &ManagedVec::from_single_item(new_uri));
    }

    /// Adds a multiple URIs to an NFT, via a synchronous builtin function call.
    pub fn nft_add_multiple_uri(
        &self,
        token_id: &TokenIdentifier<'a, A>,
        nft_nonce: u64,
        new_uris: &ManagedVec<'a, A, ManagedBuffer<'a, A>>,
    ) {
        if new_uris.is_empty() {
            return;
        }

        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(token_id);
        arg_buffer.push_arg(nft_nonce);

        for uri in new_uris {
            arg_buffer.push_arg(uri);
        }

        let _ = self.call_local_esdt_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            &ManagedBuffer::from(ESDT_NFT_ADD_URI_FUNC_NAME),
            &arg_buffer,
        );
    }

    /// Changes attributes of an NFT, via a synchronous builtin function call.
    pub fn nft_update_attributes<T: codec::TopEncode>(
        &self,
        token_id: &TokenIdentifier<'a, A>,
        nft_nonce: u64,
        new_attributes: &T,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(token_id);
        arg_buffer.push_arg(nft_nonce);
        arg_buffer.push_arg(new_attributes);

        let _ = self.call_local_esdt_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            &ManagedBuffer::from(ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME),
            &arg_buffer,
        );
    }
}
