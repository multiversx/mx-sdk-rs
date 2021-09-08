use elrond_codec::{TopDecode, TopEncode};

use super::{BlockchainApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi};
use crate::{
    types::{
        managed_vec_from_slice_of_boxed_bytes, ArgBuffer, BigUint, CodeMetadata, EsdtTokenPayment,
        ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedFrom, ManagedInto, ManagedVec,
        TokenIdentifier, Vec,
    },
    HexCallDataSerializer,
};

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
pub const ESDT_NFT_TRANSFER_STRING: &[u8] = b"ESDTNFTTransfer";
pub const ESDT_MULTI_TRANSFER_STRING: &[u8] = b"MultiESDTNFTTransfer";

const PERCENTAGE_TOTAL: u64 = 10_000;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
pub trait SendApi: Clone + Sized {
    type ProxyTypeManager: ManagedTypeApi + ErrorApi + 'static;

    /// Not used by `SendApi`, but forwarded to the proxy traits.
    type ProxyStorage: StorageReadApi
        + StorageWriteApi
        + ManagedTypeApi
        + ErrorApi
        + Clone
        + 'static;

    type ErrorApi: ErrorApi + ManagedTypeApi + Clone + 'static;

    type BlockchainApi: BlockchainApi<Storage = Self::ProxyStorage, TypeManager = Self::ProxyTypeManager>
        + Clone
        + 'static;

    fn type_manager(&self) -> Self::ProxyTypeManager;

    fn error_api(&self) -> Self::ErrorApi;

    /// Required by some of the methods.
    fn blockchain(&self) -> Self::BlockchainApi;

    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    fn direct_egld<D>(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: D,
    ) where
        D: ManagedInto<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>;

    /// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
    fn direct_egld_execute(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT to an address and executes like an async call, but without callback.
    fn direct_esdt_execute(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT NFT to an address and executes like an async call, but without callback.
    #[allow(clippy::too_many_arguments)]
    fn direct_esdt_nft_execute(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]>;

    fn direct_multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        payments: &ManagedVec<Self::ProxyTypeManager, EsdtTokenPayment<Self::ProxyTypeManager>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]>;

    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce
    fn direct<D>(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: D,
    ) where
        D: ManagedInto<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
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
    fn async_call_raw(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> !;

    /// Performs a simple ESDT/NFT transfer, but via async call.  
    /// As with any async call, this immediately terminates the execution of the current call.  
    /// So only use as the last call in your endpoint.  
    /// If you want to perform multiple transfers, use `self.send().transfer_multiple_esdt_via_async_call()` instead.  
    /// Note that EGLD can NOT be transfered with this function.  
    fn transfer_esdt_via_async_call<D>(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: D,
    ) -> !
    where
        D: ManagedInto<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
    {
        let data_buf: ManagedBuffer<Self::ProxyTypeManager> =
            data.managed_into(self.type_manager());
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

    fn transfer_multiple_esdt_via_async_call<D>(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        payments: &ManagedVec<Self::ProxyTypeManager, EsdtTokenPayment<Self::ProxyTypeManager>>,
        data: D,
    ) -> !
    where
        D: ManagedInto<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
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
        let data_buf: ManagedBuffer<Self::ProxyTypeManager> =
            data.managed_into(self.type_manager());
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
    fn deploy_contract(
        &self,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        code: &ManagedBuffer<Self::ProxyTypeManager>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> (
        ManagedAddress<Self::ProxyTypeManager>,
        ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
    );

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    fn deploy_from_source_contract(
        &self,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        source_contract_address: &ManagedAddress<Self::ProxyTypeManager>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> (
        ManagedAddress<Self::ProxyTypeManager>,
        ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
    );

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<Self::ProxyTypeManager>,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        code: &ManagedBuffer<Self::ProxyTypeManager>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>;

    fn change_owner_address(
        &self,
        child_sc_address: &ManagedAddress<Self::ProxyTypeManager>,
        new_owner: &ManagedAddress<Self::ProxyTypeManager>,
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
    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self::ProxyTypeManager>,
        value: &BigUint<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>;

    /// Same shard, in-line execution of another contract.
    /// Allows the contract to specify which result range to extract as sync call result.
    /// This is a workaround to handle nested sync calls.
    /// Please do not use this method unless there is absolutely no other option.
    /// Will be eliminated after some future Arwen hook redesign.
    /// `range_closure` takes the number of results before, the number of results after,
    /// and is expected to return the start index (inclusive) and end index (exclusive).
    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        gas: u64,
        address: &ManagedAddress<Self::ProxyTypeManager>,
        value: &BigUint<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
        range_closure: F,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>
    where
        F: FnOnce(usize, usize) -> (usize, usize);

    fn execute_on_dest_context_by_caller_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self::ProxyTypeManager>,
        value: &BigUint<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>;

    fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self::ProxyTypeManager>,
        value: &BigUint<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>;

    fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>;

    /// Used to store data between async call and callback.
    fn storage_store_tx_hash_key(&self, data: &ManagedBuffer<Self::ProxyTypeManager>);

    /// Used to store data between async call and callback.
    fn storage_load_tx_hash_key(&self) -> ManagedBuffer<Self::ProxyTypeManager>;

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        function_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>;

    /// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
    /// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
    /// or this will fail with "action is not allowed"
    /// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
    /// This function cannot be used for NFTs.
    fn esdt_local_mint(
        &self,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
    ) {
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
    fn esdt_local_burn(
        &self,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
    ) {
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
    fn esdt_nft_create<T: elrond_codec::TopEncode>(
        &self,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        name: &ManagedBuffer<Self::ProxyTypeManager>,
        royalties: &BigUint<Self::ProxyTypeManager>,
        hash: &ManagedBuffer<Self::ProxyTypeManager>,
        attributes: &T,
        uris: &ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
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
    fn sell_nft(
        &self,
        nft_id: &TokenIdentifier<Self::ProxyTypeManager>,
        nft_nonce: u64,
        nft_amount: &BigUint<Self::ProxyTypeManager>,
        buyer: &ManagedAddress<Self::ProxyTypeManager>,
        payment_token: &TokenIdentifier<Self::ProxyTypeManager>,
        payment_nonce: u64,
        payment_amount: &BigUint<Self::ProxyTypeManager>,
    ) -> BigUint<Self::ProxyTypeManager> {
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
