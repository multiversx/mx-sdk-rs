use crate::async_data::AsyncCallTxData;
use crate::{SendBalance, TxContext, TxOutput, TxPanic};
use elrond_wasm::api::{BlockchainApi, SendApi, StorageReadApi, StorageWriteApi};
use elrond_wasm::types::{
    Address, ArgBuffer, BigUint, BoxedBytes, CodeMetadata, EsdtTokenPayment, TokenIdentifier,
};
// use num_bigint::BigUint;
use num_traits::Zero;

impl TxContext {
    fn get_available_egld_balance(&self) -> num_bigint::BigUint {
        // start with the pre-existing balance
        let mut available_balance = self.blockchain_info_box.contract_balance.clone();

        // add amount received
        available_balance += &self.tx_input_box.call_value;

        // already sent
        let tx_output = self.tx_output_cell.borrow();
        for send_balance in &tx_output.send_balance_list {
            available_balance -= &send_balance.amount;
        }

        available_balance
    }

    fn get_available_esdt_balance(&self, token_name: &[u8]) -> num_bigint::BigUint {
        // start with the pre-existing balance
        let mut available_balance = self
            .blockchain_info_box
            .contract_esdt
            .get(token_name)
            .unwrap_or(&num_bigint::BigUint::zero())
            .clone();

        // add amount received (if the same token)
        if self.tx_input_box.esdt_token_identifier == token_name {
            available_balance += &self.tx_input_box.esdt_value;
        }
        let tx_output = self.tx_output_cell.borrow();

        // already sent
        for send_balance in &tx_output.send_balance_list {
            if send_balance.token_name.as_slice() == token_name {
                available_balance -= &send_balance.amount;
            }
        }

        available_balance
    }
}

impl SendApi for TxContext {
    type ProxyTypeManager = Self;
    type ProxyStorage = Self;
    type ErrorApi = Self;
    type BlockchainApi = Self;

    fn type_manager(&self) -> Self::ProxyTypeManager {
        self.clone()
    }

    fn error_api(&self) -> Self::ErrorApi {
        self.clone()
    }

    fn blockchain(&self) -> Self::BlockchainApi {
        self.clone()
    }

    fn direct_egld(&self, to: &Address, amount: &BigUint<Self::ProxyTypeManager>, _data: &[u8]) {
        let amount_value = self.big_uint_value(amount);
        if amount_value > self.get_available_egld_balance() {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"failed transfer (insufficient funds)".to_vec(),
            });
        }

        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output.send_balance_list.push(SendBalance {
            recipient: to.clone(),
            token_name: BoxedBytes::empty(),
            amount: amount_value,
        });
    }

    fn direct_egld_execute(
        &self,
        _to: &Address,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _gas_limit: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        panic!("direct_egld_execute not yet implemented")
    }

    fn direct_esdt_execute(
        &self,
        to: &Address,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        _gas: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        let amount_value = self.big_uint_value(amount);
        if amount_value > self.get_available_esdt_balance(token.to_esdt_identifier().as_slice()) {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"insufficient funds".to_vec(),
            });
        }

        let token_name = token.to_esdt_identifier();
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output.send_balance_list.push(SendBalance {
            recipient: to.clone(),
            token_name,
            amount: amount_value,
        });
        Ok(())
    }

    fn direct_esdt_nft_execute(
        &self,
        _to: &Address,
        _token: &TokenIdentifier<Self::ProxyTypeManager>,
        _nonce: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _gas_limit: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        panic!("direct_esdt_nft_execute not implemented yet");
    }

    fn direct_multi_esdt_transfer_execute(
        &self,
        _to: &Address,
        _tokens: &[EsdtTokenPayment<Self::ProxyTypeManager>],
        _gas_limit: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        panic!("direct_multi_esdt_transfer_execute not implemented yet");
    }

    fn async_call_raw(
        &self,
        to: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: &[u8],
    ) -> ! {
        let amount_value = self.big_uint_value(amount);
        // the cell is no longer needed, since we end in a panic
        let mut tx_output = self.tx_output_cell.replace(TxOutput::default());
        tx_output.async_call = Some(AsyncCallTxData {
            to: to.clone(),
            call_value: amount_value,
            call_data: data.to_vec(),
            tx_hash: self.get_tx_hash(),
        });
        std::panic::panic_any(tx_output)
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> Option<Address> {
        panic!("deploy_contract not yet implemented")
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _source_contract_address: &Address,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> Option<Address> {
        panic!("deploy_from_source_contract not yet implemented")
    }

    fn upgrade_contract(
        &self,
        _sc_address: &Address,
        _gas: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) {
        panic!("upgrade_contract not yet implemented")
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUint<Self::ProxyTypeManager>,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        panic!("execute_on_dest_context_raw not implemented yet!");
    }

    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUint<Self::ProxyTypeManager>,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
        _range_closure: F,
    ) -> Vec<BoxedBytes>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        panic!("execute_on_dest_context_raw_custom_result_range not implemented yet!");
    }

    fn execute_on_dest_context_by_caller_raw(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUint<Self::ProxyTypeManager>,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        panic!("execute_on_dest_context_by_caller_raw not implemented yet!");
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUint<Self::ProxyTypeManager>,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) {
        panic!("execute_on_same_context_raw not implemented yet!");
    }

    fn storage_store_tx_hash_key(&self, data: &[u8]) {
        let tx_hash = self.get_tx_hash();
        self.storage_store_slice_u8(tx_hash.as_bytes(), data);
    }

    fn storage_load_tx_hash_key(&self) -> BoxedBytes {
        let tx_hash = self.get_tx_hash();
        self.storage_load_boxed_bytes(tx_hash.as_bytes())
    }

    fn call_local_esdt_built_in_function(
        &self,
        _gas: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        panic!("call_local_esdt_built_in_function not implemented yet!");
    }
}
