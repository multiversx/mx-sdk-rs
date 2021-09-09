use crate::{
    api::{BlockchainApi, ManagedTypeApi, SendApi, StorageReadApi},
    esdt::ESDTSystemSmartContractProxy,
    types::{
        managed_vec_from_slice_of_boxed_bytes, ArgBuffer, BigUint, CodeMetadata, ContractCall,
        EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedFrom,
        ManagedInto, ManagedVec, TokenIdentifier, Vec,
    },
    HexCallDataSerializer,
};
use elrond_codec::{TopDecode, TopEncode};

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
pub const ESDT_NFT_TRANSFER_STRING: &[u8] = b"ESDTNFTTransfer";
pub const ESDT_MULTI_TRANSFER_STRING: &[u8] = b"MultiESDTNFTTransfer";

const PERCENTAGE_TOTAL: u64 = 10_000;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
// pub trait SendApi: Clone + Sized {

pub struct SendHelper<A>
where
    A: SendApi + ManagedTypeApi + StorageReadApi + BlockchainApi,
{
    pub(crate) api: A,
}

impl<A> SendHelper<A>
where
    A: SendApi + ManagedTypeApi + StorageReadApi + BlockchainApi,
{
    fn type_manager(&self) -> A {
        self.api.clone()
    }

    fn blockchain(&self) -> A {
        self.api.clone()
    }

    pub(crate) fn new(api: A) -> Self {
        SendHelper { api }
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

    // pub fn error_api(&self) -> A::ErrorApi;

    // /// Required by some of the methods.
    // pub fn blockchain(&self) -> A::BlockchainApi;

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, amount: &BigUint<A>, data: D)
    where
        D: ManagedInto<A, ManagedBuffer<A>>,
    {
        self.api.direct_egld(to, amount, data)
    }

    /// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
    pub fn direct_egld_execute(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        self.api
            .direct_egld_execute(to, amount, gas_limit, endpoint_name, arg_buffer)
    }

    /// Sends ESDT to an address and executes like an async call, but without callback.
    pub fn direct_esdt_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        self.api
            .direct_esdt_execute(to, token, amount, gas_limit, endpoint_name, arg_buffer)
    }

    /// Sends ESDT NFT to an address and executes like an async call, but without callback.
    #[allow(clippy::too_many_arguments)]
    pub fn direct_esdt_nft_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        self.api.direct_esdt_nft_execute(
            to,
            token,
            nonce,
            amount,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
    }

    pub fn direct_multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        self.api.direct_multi_esdt_transfer_execute(
            to,
            payments,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
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
            let _ = self.direct_esdt_execute(
                to,
                token,
                amount,
                0,
                &data.managed_into(self.type_manager()),
                &ManagedArgBuffer::new_empty(self.type_manager()),
            );
        } else {
            let _ = self.direct_esdt_nft_execute(
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

    /// Sends an asynchronous call to another contract.
    /// Calling this method immediately terminates tx execution.
    /// Using it directly is generally discouraged.
    ///
    /// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
    /// Use a `HexCallDataSerializer` to prepare this field.
    pub fn async_call_raw(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ! {
        self.api
            .async_call_raw(to, amount, endpoint_name, arg_buffer)
    }

    // /// Sends an asynchronous call to another contract, with either EGLD or ESDT value.
    // /// The `token` argument decides which one it will be.
    // /// Calling this method immediately terminates tx execution.
    // pub fn async_call(&self, async_call: AsyncCall<A>) -> ! {
    //     self.async_call_raw(
    //         &async_call.to,
    //         &async_call.egld_payment,
    //         async_call.hex_data.as_slice(),
    //     )
    // }

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

            self.async_call_raw(
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

            self.async_call_raw(
                &self.blockchain().get_sc_address(),
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
            // TODO: check that `!token_name.is_egld()` or let Arwen throw the error?
            arg_buffer.push_arg(payment.token_name);
            arg_buffer.push_arg(payment.token_nonce);
            arg_buffer.push_arg(payment.amount);
        }
        let data_buf: ManagedBuffer<A> = data.managed_into(self.type_manager());
        if !data_buf.is_empty() {
            arg_buffer.push_arg_raw(data_buf);
        }

        self.async_call_raw(
            &self.blockchain().get_sc_address(),
            &BigUint::zero(self.type_manager()),
            &ManagedBuffer::new_from_bytes(self.type_manager(), ESDT_MULTI_TRANSFER_STRING),
            &arg_buffer,
        );
    }

    /// Deploys a new contract in the same shard.
    /// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
    /// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    pub fn deploy_contract(
        &self,
        gas: u64,
        amount: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Option<ManagedAddress<A>> {
        self.api
            .deploy_contract(gas, amount, code, code_metadata, arg_buffer)
    }

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    pub fn deploy_from_source_contract(
        &self,
        gas: u64,
        amount: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Option<ManagedAddress<A>> {
        self.api.deploy_from_source_contract(
            gas,
            amount,
            source_contract_address,
            code_metadata,
            arg_buffer,
        )
    }

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    pub fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        amount: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        self.api
            .upgrade_contract(sc_address, gas, amount, code, code_metadata, arg_buffer)
    }

    pub fn change_owner_address(
        &self,
        child_sc_address: &ManagedAddress<A>,
        new_owner: &ManagedAddress<A>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.type_manager());
        arg_buffer.push_arg(new_owner);

        let _ = self.execute_on_dest_context_raw(
            self.blockchain().get_gas_left(),
            child_sc_address,
            &BigUint::zero(self.type_manager()),
            &ManagedBuffer::new_from_bytes(self.type_manager(), b"ChangeOwnerAddress"),
            &arg_buffer,
        );
    }

    /// Same shard, in-line execution of another contract.
    pub fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        self.api
            .execute_on_dest_context_raw(gas, address, value, endpoint_name, arg_buffer)
    }

    /// Same shard, in-line execution of another contract.
    /// Allows the contract to specify which result range to extract as sync call result.
    /// This is a workaround to handle nested sync calls.
    /// Please do not use this method unless there is absolutely no other option.
    /// Will be eliminated after some future Arwen hook redesign.
    /// `range_closure` takes the number of results before, the number of results after,
    /// and is expected to return the start index (inclusive) and end index (exclusive).
    pub fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
        range_closure: F,
    ) -> ManagedVec<A, ManagedBuffer<A>>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        self.api.execute_on_dest_context_raw_custom_result_range(
            gas,
            address,
            value,
            endpoint_name,
            arg_buffer,
            range_closure,
        )
    }

    pub fn execute_on_dest_context_by_caller_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        self.api.execute_on_dest_context_by_caller_raw(
            gas,
            address,
            value,
            endpoint_name,
            arg_buffer,
        )
    }

    pub fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        self.api
            .execute_on_same_context_raw(gas, address, value, endpoint_name, arg_buffer)
    }

    pub fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        self.api
            .execute_on_dest_context_readonly_raw(gas, address, endpoint_name, arg_buffer)
    }

    /// Used to store data between async call and callback.
    pub fn storage_store_tx_hash_key(&self, data: &ManagedBuffer<A>) {
        self.api.storage_store_tx_hash_key(data)
    }

    /// Used to store data between async call and callback.
    pub fn storage_load_tx_hash_key(&self) -> ManagedBuffer<A> {
        self.api.storage_load_tx_hash_key()
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
            self.blockchain().get_gas_left(),
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
            self.blockchain().get_gas_left(),
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
            self.blockchain().get_gas_left(),
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
        let nft_token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            nft_id,
            nft_nonce,
        );
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
