use core::marker::PhantomData;

use crate::codec::Empty;

use crate::types::{ManagedRef, SystemSCAddress};
use crate::{
    api::{BlockchainApi, CallTypeApi, StorageReadApi},
    codec,
    types::{
        system_proxy, BigUint, ContractCallNoPayment, ESDTSystemSCAddress,
        EgldOrEsdtTokenIdentifier, EsdtTokenPayment, FunctionCall, GasLeft, ManagedAddress,
        ManagedArgBuffer, ManagedBuffer, ManagedType, ManagedVec, NotPayable, OriginalResultMarker,
        ReturnsRawResult, ReturnsResult, ToSelf, TokenIdentifier, Tx, TxScEnv,
    },
};

use super::BlockchainWrapper;

const PERCENTAGE_TOTAL: u64 = 10_000;

use super::SendRawWrapper;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
// pub trait SendApi: Clone + Sized {

#[derive(Default)]
pub struct SendWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    _phantom: PhantomData<A>,
}

impl<A> SendWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    pub fn new() -> Self {
        SendWrapper {
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn send_raw_wrapper(&self) -> SendRawWrapper<A> {
        SendRawWrapper::new()
    }

    /// Backwards compatibility, synonymous to `esdt_system_sc_tx`, which is the more appropriate name now.
    pub fn esdt_system_sc_proxy(
        &self,
    ) -> system_proxy::ESDTSystemSCProxyMethods<TxScEnv<A>, (), ESDTSystemSCAddress, ()> {
        self.esdt_system_sc_tx()
    }

    /// Prepares a proxy object to call the ESDT system SC.
    /// It has the destination address set, as well as the contract type (as specified in the proxy).
    pub fn esdt_system_sc_tx(
        &self,
    ) -> system_proxy::ESDTSystemSCProxyMethods<TxScEnv<A>, (), ESDTSystemSCAddress, ()> {
        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(system_proxy::ESDTSystemSCProxy)
    }

    /// Prepares a proxy object to call the system SC.
    /// It has the destination address set, as well as the contract type (as specified in the proxy).
    pub fn system_sc_tx(
        &self,
    ) -> system_proxy::SystemSCProxyMethods<TxScEnv<A>, (), SystemSCAddress, ()> {
        Tx::new_tx_from_sc()
            .to(SystemSCAddress)
            .typed(system_proxy::SystemSCProxy)
    }

    /// Convenient way to quickly instance a minimal contract call (with no EGLD, no arguments, etc.)
    ///
    /// You can further configure this contract call by chaining methods to it.
    #[inline]
    pub fn contract_call<R>(
        &self,
        to: ManagedAddress<A>,
        endpoint_name: impl Into<ManagedBuffer<A>>,
    ) -> ContractCallNoPayment<A, R> {
        ContractCallNoPayment::new(to, endpoint_name)
    }

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    #[inline]
    pub fn direct_egld(&self, to: &ManagedAddress<A>, amount: &BigUint<A>) {
        Tx::new_tx_from_sc().to(to).egld(amount).transfer();
    }

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    ///
    /// If the amount is 0, it returns without error.
    pub fn direct_non_zero_egld(&self, to: &ManagedAddress<A>, amount: &BigUint<A>) {
        Tx::new_tx_from_sc()
            .to(to)
            .egld(amount)
            .transfer_if_not_empty();
    }

    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce
    #[inline]
    pub fn direct(
        &self,
        to: &ManagedAddress<A>,
        token: &EgldOrEsdtTokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
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
        to: &ManagedAddress<A>,
        token: &EgldOrEsdtTokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
    ) {
        self.direct_non_zero_with_gas_limit(to, token, nonce, amount, 0, Empty, &[]);
    }

    /// Sends a single ESDT transfer, and calls an endpoint at the destination.
    ///
    /// Avoid if possible, use a contract call with ESDT transfer instead, and call `.transfer_execute()` on it.
    #[allow(clippy::too_many_arguments)]
    pub fn direct_esdt_with_gas_limit<D>(
        &self,
        to: &ManagedAddress<A>,
        token_identifier: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<A>],
    ) where
        D: Into<ManagedBuffer<A>>,
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
        to: &ManagedAddress<A>,
        token_identifier: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<A>],
    ) where
        D: Into<ManagedBuffer<A>>,
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
    pub fn direct_esdt(
        &self,
        to: &ManagedAddress<A>,
        token_identifier: &TokenIdentifier<A>,
        token_nonce: u64,
        amount: &BigUint<A>,
    ) {
        Tx::new_tx_from_sc()
            .to(to)
            .single_esdt(token_identifier, token_nonce, amount)
            .transfer();
    }

    /// Sends a single ESDT transfer to target address.
    ///
    /// If the amount is 0, it returns without error.
    pub fn direct_non_zero_esdt_payment(
        &self,
        to: &ManagedAddress<A>,
        payment: &EsdtTokenPayment<A>,
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
        to: &ManagedAddress<A>,
        token: &EgldOrEsdtTokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<A>],
    ) where
        D: Into<ManagedBuffer<A>>,
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
        to: &ManagedAddress<A>,
        token: &EgldOrEsdtTokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<A>],
    ) where
        D: Into<ManagedBuffer<A>>,
    {
        if amount == &0 {
            return;
        }
        self.direct_with_gas_limit(to, token, nonce, amount, gas, endpoint_name, arguments);
    }

    /// Sends multiple ESDT tokens to a target address.
    pub fn direct_multi(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
    ) {
        Tx::new_tx_from_sc().to(to).payment(payments).transfer();
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
        to: ManagedAddress<A>,
        token: TokenIdentifier<A>,
        nonce: u64,
        amount: BigUint<A>,
    ) -> ! {
        Tx::new_tx_from_sc()
            .to(to)
            .esdt((token, nonce, amount))
            .async_call_and_exit()
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
        to: ManagedAddress<A>,
        token: TokenIdentifier<A>,
        nonce: u64,
        amount: BigUint<A>,
    ) {
        if amount == 0 {
            return;
        }
        self.transfer_esdt_via_async_call(to, token, nonce, amount)
    }

    /// Sends multiple ESDT tokens to a target address, via an async call.
    pub fn transfer_multiple_esdt_via_async_call(
        &self,
        to: ManagedAddress<A>,
        payments: ManagedVec<A, EsdtTokenPayment<A>>,
    ) -> ! {
        Tx::new_tx_from_sc()
            .to(to)
            .payment(payments)
            .async_call_and_exit()
    }

    /// Creates a call to the `ClaimDeveloperRewards` builtin function.
    #[allow(clippy::type_complexity)]
    pub fn claim_developer_rewards(
        &self,
        child_sc_address: ManagedAddress<A>,
    ) -> Tx<
        TxScEnv<A>,
        (),
        ManagedAddress<A>,
        NotPayable,
        (),
        FunctionCall<A>,
        OriginalResultMarker<()>,
    > {
        Tx::new_tx_from_sc()
            .to(child_sc_address)
            .typed(system_proxy::UserBuiltinProxy)
            .claim_developer_rewards()
    }

    /// Creates a call to the `ChangeOwnerAddress` builtin function.
    #[allow(clippy::type_complexity)]
    pub fn change_owner_address(
        &self,
        child_sc_address: ManagedAddress<A>,
        new_owner: &ManagedAddress<A>,
    ) -> Tx<
        TxScEnv<A>,
        (),
        ManagedAddress<A>,
        NotPayable,
        (),
        FunctionCall<A>,
        OriginalResultMarker<()>,
    > {
        Tx::new_tx_from_sc()
            .to(child_sc_address)
            .typed(system_proxy::UserBuiltinProxy)
            .change_owner_address(new_owner)
    }

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    pub fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        endpoint_name: ManagedBuffer<A>,
        arg_buffer: ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(gas)
            .raw_call(endpoint_name)
            .arguments_raw(arg_buffer)
            .returns(ReturnsRawResult)
            .sync_call()
    }

    /// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
    /// or this will fail with "action is not allowed".
    ///
    /// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
    ///
    /// This function cannot be used for NFTs.
    pub fn esdt_local_mint(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_local_mint(token, nonce, amount)
            .sync_call()
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
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
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
    pub fn esdt_local_burn(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_local_burn(token, nonce, amount)
            .sync_call()
    }

    /// Allows synchronous burning of ESDT/SFT/NFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the ESDTLocalBurn or ESDTNftBurn roles set,
    /// or this will fail with "action is not allowed".
    ///
    /// If the amount is 0, it returns without error.
    pub fn esdt_non_zero_local_burn(
        &self,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
    ) {
        if amount == &0 {
            return;
        }
        self.esdt_local_burn(token, nonce, amount);
    }

    /// Allows burning of multiple ESDT tokens at once.
    ///
    /// Will execute a synchronous call to the appropriate burn builtin function for each.
    pub fn esdt_local_burn_multi(&self, payments: &ManagedVec<A, EsdtTokenPayment<A>>) {
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
    pub fn esdt_non_zero_local_burn_multi(&self, payments: &ManagedVec<A, EsdtTokenPayment<A>>) {
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
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
        royalties: &BigUint<A>,
        hash: &ManagedBuffer<A>,
        attributes: &T,
        uris: &ManagedVec<A, ManagedBuffer<A>>,
    ) -> u64 {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_nft_create(token, amount, name, royalties, hash, attributes, uris)
            .returns(ReturnsResult)
            .sync_call()
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
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
        royalties: &BigUint<A>,
        hash: &ManagedBuffer<A>,
        attributes: &T,
        uris: &ManagedVec<A, ManagedBuffer<A>>,
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
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        attributes: &T,
    ) -> u64 {
        self.esdt_nft_create_compact_named(token, amount, &ManagedBuffer::new(), attributes)
    }

    /// Quick way of creating a new NFT token instance, with custom name.
    ///
    /// Returns the new NFT nonce.
    pub fn esdt_nft_create_compact_named<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
        attributes: &T,
    ) -> u64 {
        let big_zero = BigUint::zero();
        let empty_buffer = ManagedBuffer::new();

        // sneakily reuses the same handle
        let empty_vec = unsafe { ManagedRef::wrap_handle(empty_buffer.get_handle()) };

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
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
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
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
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
        nft_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        nft_amount: &BigUint<A>,
        buyer: &ManagedAddress<A>,
        payment_token: &EgldOrEsdtTokenIdentifier<A>,
        payment_nonce: u64,
        payment_amount: &BigUint<A>,
    ) -> BigUint<A> {
        let nft_token_data = BlockchainWrapper::<A>::new().get_esdt_token_data(
            &BlockchainWrapper::<A>::new().get_sc_address(),
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
        nft_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        nft_amount: &BigUint<A>,
        buyer: &ManagedAddress<A>,
        payment_token: &EgldOrEsdtTokenIdentifier<A>,
        payment_nonce: u64,
        payment_amount: &BigUint<A>,
    ) -> BigUint<A> {
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
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_uri: ManagedBuffer<A>,
    ) {
        self.nft_add_multiple_uri(token_id, nft_nonce, &ManagedVec::from_single_item(new_uri));
    }

    /// Adds a multiple URIs to an NFT, via a synchronous builtin function call.
    pub fn nft_add_multiple_uri(
        &self,
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_uris: &ManagedVec<A, ManagedBuffer<A>>,
    ) {
        if new_uris.is_empty() {
            return;
        }

        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .nft_add_multiple_uri(token_id, nft_nonce, new_uris)
            .sync_call()
    }

    /// Changes attributes of an NFT, via a synchronous builtin function call.
    pub fn nft_update_attributes<T: codec::TopEncode>(
        &self,
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_attributes: &T,
    ) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .nft_update_attributes(token_id, nft_nonce, new_attributes)
            .sync_call()
    }

    /// Modifies royalties for a specific token.
    pub fn esdt_modify_royalties(
        &self,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
        new_royalty: u64,
    ) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_modify_royalties(token_id, nonce, new_royalty)
            .sync_call()
    }

    /// Sets new uris for a specific token.
    pub fn esdt_nft_set_new_uris(
        &self,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
        uris: &ManagedVec<A, ManagedBuffer<A>>,
    ) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_nft_set_new_uris(token_id, nonce, uris)
            .sync_call()
    }

    /// Changes the creator of a specific token into the caller.
    pub fn esdt_nft_modify_creator(&self, token_id: &TokenIdentifier<A>, nonce: u64) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_nft_modify_creator(token_id, nonce)
            .sync_call()
    }

    /// Recreates an ESDT token with the newly specified attributes.
    #[allow(clippy::too_many_arguments)]
    pub fn esdt_metadata_recreate<T: codec::TopEncode>(
        &self,
        token_id: TokenIdentifier<A>,
        nonce: u64,
        name: ManagedBuffer<A>,
        royalties: u64,
        hash: ManagedBuffer<A>,
        new_attributes: &T,
        uris: ManagedVec<A, ManagedBuffer<A>>,
    ) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_metadata_recreate(token_id, nonce, name, royalties, hash, new_attributes, uris)
            .sync_call()
    }

    /// Updates an ESDT token with the newly specified attributes.
    #[allow(clippy::too_many_arguments)]
    pub fn esdt_metadata_update<T: codec::TopEncode>(
        &self,
        token_id: TokenIdentifier<A>,
        nonce: u64,
        name: ManagedBuffer<A>,
        royalties: u64,
        hash: ManagedBuffer<A>,
        new_attributes: &T,
        uris: ManagedVec<A, ManagedBuffer<A>>,
    ) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_metadata_update(token_id, nonce, name, royalties, hash, new_attributes, uris)
            .sync_call()
    }
}
