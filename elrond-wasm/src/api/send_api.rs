use elrond_codec::{TopDecode, TopEncode};

use super::{BlockchainApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi};
use crate::{
    types::{
        Address, ArgBuffer, AsyncCall, BigUint, BoxedBytes, CodeMetadata, EsdtTokenPayment,
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
    fn direct_egld(&self, to: &Address, amount: &BigUint<Self::ProxyTypeManager>, data: &[u8]);

    /// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
    fn direct_egld_execute(
        &self,
        to: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT to an address and executes like an async call, but without callback.
    fn direct_esdt_execute(
        &self,
        to: &Address,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT NFT to an address and executes like an async call, but without callback.
    #[allow(clippy::too_many_arguments)]
    fn direct_esdt_nft_execute(
        &self,
        to: &Address,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    fn direct_multi_esdt_transfer_execute(
        &self,
        to: &Address,
        tokens: &[EsdtTokenPayment<Self::ProxyTypeManager>],
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    /// Sends either EGLD, ESDT or NFT to the target address,
    /// depending on the token identifier and nonce
    fn direct(
        &self,
        to: &Address,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: &[u8],
    ) {
        if token.is_egld() {
            self.direct_egld(to, amount, data);
        } else if nonce == 0 {
            let _ = self.direct_esdt_execute(to, token, amount, 0, data, &ArgBuffer::new());
        } else {
            let _ =
                self.direct_esdt_nft_execute(to, token, nonce, amount, 0, data, &ArgBuffer::new());
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
        to: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: &[u8],
    ) -> !;

    /// Sends an asynchronous call to another contract, with either EGLD or ESDT value.
    /// The `token` argument decides which one it will be.
    /// Calling this method immediately terminates tx execution.
    fn async_call(&self, async_call: AsyncCall<Self>) -> ! {
        self.async_call_raw(
            &async_call.to,
            &async_call.egld_payment,
            async_call.hex_data.as_slice(),
        )
    }

    /// Performs a simple ESDT/NFT transfer, but via async call.  
    /// As with any async call, this immediately terminates the execution of the current call.  
    /// So only use as the last call in your endpoint.  
    /// If you want to perform multiple transfers, use `self.send().transfer_multiple_esdt_via_async_call()` instead.  
    /// Note that EGLD can NOT be transfered with this function.  
    fn transfer_esdt_via_async_call(
        &self,
        to: &Address,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: &[u8],
    ) -> ! {
        if nonce == 0 {
            let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
            serializer.push_argument_bytes(token.to_esdt_identifier().as_slice());
            serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
            if !data.is_empty() {
                serializer.push_argument_bytes(data);
            }

            self.async_call_raw(
                to,
                &BigUint::zero(self.type_manager()),
                serializer.as_slice(),
            )
        } else {
            let mut serializer = HexCallDataSerializer::new(ESDT_NFT_TRANSFER_STRING);
            serializer.push_argument_bytes(token.to_esdt_identifier().as_slice());
            serializer.push_argument_bytes(&nonce.to_be_bytes()[..]);
            serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
            serializer.push_argument_bytes(to.as_bytes());
            if !data.is_empty() {
                serializer.push_argument_bytes(data);
            }

            self.async_call_raw(
                &self.blockchain().get_sc_address(),
                &BigUint::zero(self.type_manager()),
                serializer.as_slice(),
            );
        }
    }

    fn transfer_multiple_esdt_via_async_call(
        &self,
        to: &Address,
        tokens: &[EsdtTokenPayment<Self::ProxyTypeManager>],
        data: &[u8],
    ) -> ! {
        let mut serializer = HexCallDataSerializer::new(ESDT_MULTI_TRANSFER_STRING);
        serializer.push_argument_bytes(to.as_bytes());
        serializer.push_argument_bytes(&tokens.len().to_be_bytes()[..]);

        for token in tokens {
            serializer.push_argument_bytes(token.token_name.to_esdt_identifier().as_slice());
            serializer.push_argument_bytes(&token.token_nonce.to_be_bytes()[..]);
            serializer.push_argument_bytes(token.amount.to_bytes_be().as_slice());
        }

        if !data.is_empty() {
            serializer.push_argument_bytes(data);
        }

        self.async_call_raw(
            &self.blockchain().get_sc_address(),
            &BigUint::zero(self.type_manager()),
            serializer.as_slice(),
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
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> Option<Address>;

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    fn deploy_from_source_contract(
        &self,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        source_contract_address: &Address,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> Option<Address>;

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    fn upgrade_contract(
        &self,
        sc_address: &Address,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    );

    fn change_owner_address(&self, child_sc_address: &Address, new_owner: &Address) {
        let mut arg_buffer = ArgBuffer::new();
        arg_buffer.push_argument_bytes(new_owner.as_bytes());

        let _ = self.execute_on_dest_context_raw(
            self.blockchain().get_gas_left(),
            child_sc_address,
            &BigUint::zero(self.type_manager()),
            b"ChangeOwnerAddress",
            &arg_buffer,
        );
    }

    /// Same shard, in-line execution of another contract.
    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &Address,
        value: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes>;

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
        address: &Address,
        value: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
        range_closure: F,
    ) -> Vec<BoxedBytes>
    where
        F: FnOnce(usize, usize) -> (usize, usize);

    fn execute_on_dest_context_by_caller_raw(
        &self,
        gas: u64,
        address: &Address,
        value: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes>;

    fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &Address,
        value: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    );

    /// Used to store data between async call and callback.
    fn storage_store_tx_hash_key(&self, data: &[u8]);

    /// Used to store data between async call and callback.
    fn storage_load_tx_hash_key(&self) -> BoxedBytes;

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes>;

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
        let mut arg_buffer = ArgBuffer::new();
        let func_name: &[u8];

        arg_buffer.push_argument_bytes(token.to_esdt_identifier().as_slice());

        if nonce == 0 {
            func_name = b"ESDTLocalMint";
        } else {
            func_name = b"ESDTNFTAddQuantity";
            arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
        }

        arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

        let _ = self.call_local_esdt_built_in_function(
            self.blockchain().get_gas_left(),
            func_name,
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
        let mut arg_buffer = ArgBuffer::new();
        let func_name: &[u8];

        arg_buffer.push_argument_bytes(token.to_esdt_identifier().as_slice());

        if nonce == 0 {
            func_name = b"ESDTLocalBurn";
        } else {
            func_name = b"ESDTNFTBurn";
            arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
        }

        arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

        let _ = self.call_local_esdt_built_in_function(
            self.blockchain().get_gas_left(),
            func_name,
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
        name: &BoxedBytes,
        royalties: &BigUint<Self::ProxyTypeManager>,
        hash: &BoxedBytes,
        attributes: &T,
        uris: &[BoxedBytes],
    ) -> u64 {
        let mut arg_buffer = ArgBuffer::new();
        arg_buffer.push_argument_bytes(token.to_esdt_identifier().as_slice());
        arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());
        arg_buffer.push_argument_bytes(name.as_slice());
        arg_buffer.push_argument_bytes(royalties.to_bytes_be().as_slice());
        arg_buffer.push_argument_bytes(hash.as_slice());

        let mut top_encoded_attributes = Vec::new();
        let _ = attributes.top_encode(&mut top_encoded_attributes);
        arg_buffer.push_argument_bytes(top_encoded_attributes.as_slice());

        // The API function has the last argument as variadic,
        // so we top-encode each and send as separate argument
        for uri in uris {
            let mut top_encoded_uri = Vec::new();
            let _ = uri.top_encode(&mut top_encoded_uri);

            arg_buffer.push_argument_bytes(top_encoded_uri.as_slice());
        }

        let output = self.call_local_esdt_built_in_function(
            self.blockchain().get_gas_left(),
            b"ESDTNFTCreate",
            &arg_buffer,
        );

        u64::top_decode(output[0].as_slice()).unwrap_or_default()
    }

    /// Sends thr NFTs to the buyer address and calculates and sends the required royalties to the NFT creator.
    /// Returns the payment amount left after sending royalties.
    #[allow(clippy::too_many_arguments)]
    fn sell_nft(
        &self,
        nft_id: &TokenIdentifier<Self::ProxyTypeManager>,
        nft_nonce: u64,
        nft_amount: &BigUint<Self::ProxyTypeManager>,
        buyer: &Address,
        payment_token: &TokenIdentifier<Self::ProxyTypeManager>,
        payment_nonce: u64,
        payment_amount: &BigUint<Self::ProxyTypeManager>,
    ) -> BigUint<Self::ProxyTypeManager> {
        let nft_token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address_managed(),
            nft_id,
            nft_nonce,
        );
        let royalties_amount = payment_amount.clone() * nft_token_data.royalties / PERCENTAGE_TOTAL;

        self.direct(buyer, nft_id, nft_nonce, nft_amount, &[]);

        if royalties_amount > 0u32 {
            self.direct(
                &nft_token_data.creator.to_address(),
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
