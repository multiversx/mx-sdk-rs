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
    pub(crate) fn new() -> Self {
        SendWrapper {
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn send_raw_wrapper(&self) -> SendRawWrapper<A> {
        SendRawWrapper::new()
    }

    #[inline]
    pub fn esdt_system_sc_proxy(&self) -> ESDTSystemSmartContractProxy<A> {
        ESDTSystemSmartContractProxy::new_proxy_obj()
    }

    #[inline]
    pub fn contract_call<R>(
        &self,
        to: ManagedAddress<A>,
        endpoint_name: ManagedBuffer<A>,
    ) -> ContractCallNoPayment<A, R> {
        ContractCallNoPayment::new(to, endpoint_name)
    }

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    #[inline]
    pub fn direct_egld(&self, to: &ManagedAddress<A>, amount: &BigUint<A>) {
        self.send_raw_wrapper().direct_egld(to, amount, Empty)
    }

    pub fn direct_non_zero_egld(&self, to: &ManagedAddress<A>, amount: &BigUint<A>) {
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
        to: &ManagedAddress<A>,
        token: &EgldOrEsdtTokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
    ) {
        self.direct_with_gas_limit(to, token, nonce, amount, 0, Empty, &[]);
    }

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

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn direct_esdt(
        &self,
        to: &ManagedAddress<A>,
        token_identifier: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
    ) {
        self.direct_esdt_with_gas_limit(to, token_identifier, nonce, amount, 0, Empty, &[]);
    }

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

    pub fn direct_multi(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
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
    /// As with any async call, this immediately terminates the execution of the current call.  
    /// So only use as the last call in your endpoint.  
    /// If you want to perform multiple transfers, use `self.send().transfer_multiple_esdt_via_async_call()` instead.  
    /// Note that EGLD can NOT be transfered with this function.  
    pub fn transfer_esdt_via_async_call(
        &self,
        to: ManagedAddress<A>,
        token: TokenIdentifier<A>,
        nonce: u64,
        amount: BigUint<A>,
    ) -> ! {
        ContractCallNoPayment::<A, ()>::new(to, ManagedBuffer::new())
            .with_esdt_transfer((token, nonce, amount))
            .async_call()
            .call_and_exit_ignore_callback()
    }

    pub fn transfer_multiple_esdt_via_async_call(
        &self,
        to: ManagedAddress<A>,
        payments: ManagedVec<A, EsdtTokenPayment<A>>,
    ) -> ! {
        ContractCallNoPayment::<A, ()>::new(to, ManagedBuffer::new())
            .with_multi_token_transfer(payments)
            .async_call()
            .call_and_exit_ignore_callback()
    }

    pub fn claim_developer_rewards(
        &self,
        child_sc_address: ManagedAddress<A>,
    ) -> ContractCallNoPayment<A, ()> {
        ContractCallNoPayment::new(child_sc_address, CLAIM_DEVELOPER_REWARDS_FUNC_NAME)
    }

    /// Sends a synchronous call to change a smart contract address.
    pub fn change_owner_address(
        &self,
        child_sc_address: ManagedAddress<A>,
        new_owner: &ManagedAddress<A>,
    ) -> ContractCallNoPayment<A, ()> {
        let mut contract_call =
            ContractCallNoPayment::new(child_sc_address, CHANGE_OWNER_BUILTIN_FUNC_NAME);
        contract_call.proxy_arg(&new_owner);
        contract_call
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
        self.send_raw_wrapper()
            .call_local_esdt_built_in_function(gas, endpoint_name, arg_buffer)
    }

    /// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
    /// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
    /// or this will fail with "action is not allowed"
    /// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
    /// This function cannot be used for NFTs.
    pub fn esdt_local_mint(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
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

    /// Allows synchronous burning of ESDT/SFT/NFT (depending on nonce). Execution is resumed afterwards.
    /// Note that the SC must have the ESDTLocalBurn or ESDTNftBurn roles set,
    /// or this will fail with "action is not allowed"
    pub fn esdt_local_burn(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
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

    /// Creates a new NFT token of a certain type (determined by `token_identifier`).  
    /// `attributes` can be any serializable custom struct.  
    /// This is a built-in function, so the smart contract execution is resumed after.
    /// Must have ESDTNftCreate role set, or this will fail with "action is not allowed".
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

    #[inline]
    pub fn esdt_nft_create_compact<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        attributes: &T,
    ) -> u64 {
        self.esdt_nft_create_compact_named(token, amount, &ManagedBuffer::new(), attributes)
    }

    pub fn esdt_nft_create_compact_named<T: codec::TopEncode>(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        name: &ManagedBuffer<A>,
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

    /// Sends the NFTs to the buyer address and calculates and sends the required royalties to the NFT creator.
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
        let nft_token_data = A::blockchain_api_impl().load_esdt_token_data::<A>(
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

    pub fn nft_add_uri(
        &self,
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_uri: ManagedBuffer<A>,
    ) {
        self.nft_add_multiple_uri(token_id, nft_nonce, &ManagedVec::from_single_item(new_uri));
    }

    pub fn nft_add_multiple_uri(
        &self,
        token_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        new_uris: &ManagedVec<A, ManagedBuffer<A>>,
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

    pub fn nft_update_attributes<T: codec::TopEncode>(
        &self,
        token_id: &TokenIdentifier<A>,
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
