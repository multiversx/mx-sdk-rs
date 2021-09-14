use crate::{
    api::{BlockchainApi, ManagedTypeApi, SendApi, StorageReadApi},
    esdt::ESDTSystemSmartContractProxy,
    types::{
        BigUint, ContractCall, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedInto, ManagedVec, TokenIdentifier,
    },
};
use elrond_codec::TopDecode;

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
pub const ESDT_NFT_TRANSFER_STRING: &[u8] = b"ESDTNFTTransfer";
pub const ESDT_MULTI_TRANSFER_STRING: &[u8] = b"MultiESDTNFTTransfer";
pub const CHANGE_ADDRESS_BUILTIN_FUNC_NAME: &[u8] = b"ChangeOwnerAddress";

const PERCENTAGE_TOTAL: u64 = 10_000;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
// pub trait SendApi: Clone + Sized {

pub struct SendWrapper<A>
where
    A: SendApi + ManagedTypeApi + StorageReadApi + BlockchainApi,
{
    pub(crate) api: A,
}

impl<A> SendWrapper<A>
where
    A: SendApi + ManagedTypeApi + StorageReadApi + BlockchainApi,
{
    fn type_manager(&self) -> A {
        self.api.clone()
    }

    pub(crate) fn new(api: A) -> Self {
        SendWrapper { api }
    }

    pub fn esdt_system_sc_proxy(&self) -> ESDTSystemSmartContractProxy<A> {
        ESDTSystemSmartContractProxy::new_proxy_obj(self.api.clone())
    }

    pub fn contract_call<R>(
        &self,
        to: ManagedAddress<A>,
        endpoint_name: ManagedBuffer<A>,
    ) -> ContractCall<A, R> {
        ContractCall::new(self.api.clone(), to, endpoint_name)
    }

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, amount: &BigUint<A>, data: D)
    where
        D: ManagedInto<A, ManagedBuffer<A>>,
    {
        self.api.direct_egld(to, amount, data)
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
        D: ManagedInto<A, ManagedBuffer<A>>,
    {
        if token.is_egld() {
            self.direct_egld(to, amount, data);
        } else if nonce == 0 {
            let _ = self.api.direct_esdt_execute(
                to,
                token,
                amount,
                0,
                &data.managed_into(self.type_manager()),
                &ManagedArgBuffer::new_empty(self.type_manager()),
            );
        } else {
            let _ = self.api.direct_esdt_nft_execute(
                to,
                token,
                nonce,
                amount,
                0,
                &data.managed_into(self.type_manager()),
                &ManagedArgBuffer::new_empty(self.type_manager()),
            );
        }
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
        D: ManagedInto<A, ManagedBuffer<A>>,
    {
        let data_buf: ManagedBuffer<A> = data.managed_into(self.type_manager());
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.type_manager());
        arg_buffer.push_arg(token);
        if nonce == 0 {
            arg_buffer.push_arg(amount);
            if !data_buf.is_empty() {
                arg_buffer.push_arg_raw(data_buf);
            }

            self.api.async_call_raw(
                to,
                &BigUint::zero(self.type_manager()),
                &ManagedBuffer::new_from_bytes(self.type_manager(), ESDT_TRANSFER_STRING),
                &arg_buffer,
            )
        } else {
            arg_buffer.push_arg(nonce);
            arg_buffer.push_arg(amount);
            arg_buffer.push_arg(to);
            if !data_buf.is_empty() {
                arg_buffer.push_arg_raw(data_buf);
            }

            self.api.async_call_raw(
                &self.api.get_sc_address(),
                &BigUint::zero(self.type_manager()),
                &ManagedBuffer::new_from_bytes(self.type_manager(), ESDT_NFT_TRANSFER_STRING),
                &arg_buffer,
            )
        }
    }

    pub fn transfer_multiple_esdt_via_async_call<D>(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        data: D,
    ) -> !
    where
        D: ManagedInto<A, ManagedBuffer<A>>,
    {
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.type_manager());
        arg_buffer.push_arg(to);
        arg_buffer.push_arg(payments.len());

        for payment in payments.into_iter() {
            // TODO: check that `!token_identifier.is_egld()` or let Arwen throw the error?
            arg_buffer.push_arg(payment.token_identifier);
            arg_buffer.push_arg(payment.token_nonce);
            arg_buffer.push_arg(payment.amount);
        }
        let data_buf: ManagedBuffer<A> = data.managed_into(self.type_manager());
        if !data_buf.is_empty() {
            arg_buffer.push_arg_raw(data_buf);
        }

        self.api.async_call_raw(
            &self.api.get_sc_address(),
            &BigUint::zero(self.type_manager()),
            &ManagedBuffer::new_from_bytes(self.type_manager(), ESDT_MULTI_TRANSFER_STRING),
            &arg_buffer,
        );
    }

    /// Sends a synchronous call to change a smart contract address.
    /// Only works in the same shard.
    pub fn change_owner_address(
        &self,
        child_sc_address: ManagedAddress<A>,
        new_owner: &ManagedAddress<A>,
    ) -> ContractCall<A, ()> {
        let mut contract_call = ContractCall::new(
            self.api.clone(),
            child_sc_address,
            ManagedBuffer::new_from_bytes(self.type_manager(), CHANGE_ADDRESS_BUILTIN_FUNC_NAME),
        );
        contract_call.push_endpoint_arg(&new_owner);
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
        self.api
            .call_local_esdt_built_in_function(gas, endpoint_name, arg_buffer)
    }

    /// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
    /// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
    /// or this will fail with "action is not allowed"
    /// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
    /// This function cannot be used for NFTs.
    pub fn esdt_local_mint(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.type_manager());
        let func_name: &[u8];

        arg_buffer.push_arg(token);

        if nonce == 0 {
            func_name = b"ESDTLocalMint";
        } else {
            func_name = b"ESDTNFTAddQuantity";
            arg_buffer.push_arg(nonce);
        }

        arg_buffer.push_arg(amount);

        let _ = self.call_local_esdt_built_in_function(
            self.api.get_gas_left(),
            &ManagedBuffer::new_from_bytes(self.type_manager(), func_name),
            &arg_buffer,
        );
    }

    /// Allows synchronous burning of ESDT/SFT/NFT (depending on nonce). Execution is resumed afterwards.
    /// Note that the SC must have the ESDTLocalBurn or ESDTNftBurn roles set,
    /// or this will fail with "action is not allowed"
    pub fn esdt_local_burn(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.type_manager());
        let func_name: &[u8];

        arg_buffer.push_arg(token);
        if nonce == 0 {
            func_name = b"ESDTLocalBurn";
        } else {
            func_name = b"ESDTNFTBurn";
            arg_buffer.push_arg(&nonce);
        }

        arg_buffer.push_arg(amount);

        let _ = self.call_local_esdt_built_in_function(
            self.api.get_gas_left(),
            &ManagedBuffer::new_from_bytes(self.type_manager(), func_name),
            &arg_buffer,
        );
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
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.type_manager());
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(name);
        arg_buffer.push_arg(royalties);
        arg_buffer.push_arg(hash);
        arg_buffer.push_arg(attributes);

        // The API function has the last argument as variadic,
        // so we top-encode each and send as separate argument
        for uri in uris {
            arg_buffer.push_arg(uri);
        }

        let output = self.call_local_esdt_built_in_function(
            self.api.get_gas_left(),
            &ManagedBuffer::new_from_bytes(self.type_manager(), b"ESDTNFTCreate"),
            &arg_buffer,
        );

        if let Some(first_result_bytes) = output.get(0) {
            u64::top_decode(&first_result_bytes).unwrap_or_default()
        } else {
            0
        }
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
        let nft_token_data =
            self.api
                .get_esdt_token_data(&self.api.get_sc_address(), nft_id, nft_nonce);
        let royalties_amount = payment_amount.clone() * nft_token_data.royalties / PERCENTAGE_TOTAL;

        self.direct(buyer, nft_id, nft_nonce, nft_amount, &[]);

        if royalties_amount > 0u32 {
            self.direct(
                &nft_token_data.creator,
                payment_token,
                payment_nonce,
                &royalties_amount,
                &[],
            );

            payment_amount.clone() - royalties_amount
        } else {
            payment_amount.clone()
        }
    }
}
