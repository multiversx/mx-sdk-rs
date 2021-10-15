use crate::{
    async_data::AsyncCallTxData,
    tx_mock::{SendBalance, TxPanic},
    DebugApi,
};
use elrond_wasm::{
    api::{BlockchainApi, SendApi, StorageReadApi, StorageWriteApi},
    types::{
        BigUint, BoxedBytes, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer,
        ManagedBuffer, ManagedInto, ManagedVec, TokenIdentifier,
    },
    HexCallDataSerializer,
};

impl DebugApi {
    // fn get_available_egld_balance(&self) -> num_bigint::BigUint {
    //     self.with_contract_account(|account| account.egld_balance.clone())
    // }

    // fn get_available_esdt_balance(
    //     &self,
    //     token_identifier: &[u8],
    //     nonce: u64,
    // ) -> num_bigint::BigUint {
    //     // start with the pre-existing balance
    //     let mut available_balance = self
    //         .get_contract_account()
    //         .esdt
    //         .get_esdt_balance(token_identifier, nonce);

    //     // // add amount received (if the same token)
    //     // for esdt_value in self.input_ref().esdt_values.iter() {
    //     //     if esdt_value.token_identifier == token_identifier && esdt_value.nonce == nonce {
    //     //         available_balance += &esdt_value.value;
    //     //     }
    //     // }

    //     // let tx_output = self.output_borrow();

    //     // // already sent
    //     // for send_balance in &tx_output.send_balance_list {
    //     //     if send_balance.token_identifier.as_slice() == token_identifier {
    //     //         available_balance -= &send_balance.amount;
    //     //     }
    //     // }

    //     available_balance
    // }
}

impl SendApi for DebugApi {
    fn direct_egld<D>(&self, to: &ManagedAddress<Self>, amount: &BigUint<Self>, _data: D)
    where
        D: ManagedInto<Self, ManagedBuffer<Self>>,
    {
        let amount_value = self.big_uint_value(amount);
        // if amount_value > self.get_available_egld_balance() {
        //     std::panic::panic_any(TxPanic {
        //         status: 10,
        //         message: b"failed transfer (insufficient funds)".to_vec(),
        //     });
        // }

        let contract_address = &self.input_ref().to;
        self.blockchain_cache()
            .subtract_egld_balance(contract_address, &amount_value)
            .unwrap();

        let recipient = &to.to_address();
        self.blockchain_cache()
            .increase_egld_balance(recipient, &amount_value);

        // let mut tx_result = self.result_borrow_mut();
        // tx_output.send_balance_list.push(SendBalance {
        //     recipient,
        //     token_identifier: BoxedBytes::empty(),
        //     amount: amount_value,
        //     nonce: 0u64,
        // });
    }

    fn direct_egld_execute(
        &self,
        _to: &ManagedAddress<Self>,
        _amount: &BigUint<Self>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        panic!("direct_egld_execute not yet implemented")
    }

    fn direct_esdt_execute(
        &self,
        to: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        amount: &BigUint<Self>,
        _gas: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        // let amount_value = self.big_uint_value(amount);
        // if amount_value
        //     > self.get_available_esdt_balance(token.to_esdt_identifier().as_slice(), 0u64)
        // {
        //     std::panic::panic_any(TxPanic {
        //         status: 10,
        //         message: b"insufficient funds".to_vec(),
        //     });
        // }

        // let recipient = to.to_address();
        // let token_identifier = token.to_esdt_identifier();
        // let mut tx_result = self.result_borrow_mut();
        // tx_output.send_balance_list.push(SendBalance {
        //     recipient,
        //     token_identifier,
        //     amount: amount_value,
        //     nonce: 0u64,
        // });
        // Ok(())

        panic!("direct_esdt_execute not yet implemented")
    }

    fn direct_esdt_nft_execute(
        &self,
        _to: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
        _nonce: u64,
        _amount: &BigUint<Self>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        panic!("direct_esdt_nft_execute not implemented yet");
    }

    fn direct_multi_esdt_transfer_execute(
        &self,
        _to: &ManagedAddress<Self>,
        _payments: &ManagedVec<Self, EsdtTokenPayment<Self>>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        panic!("direct_multi_esdt_transfer_execute not implemented yet");
    }

    fn async_call_raw(
        &self,
        to: &ManagedAddress<Self>,
        amount: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ! {
        // let amount_value = self.big_uint_value(amount);
        // let recipient = to.to_address();
        // let call_data =
        //     HexCallDataSerializer::from_managed_arg_buffer(endpoint_name, arg_buffer).into_vec();
        // let tx_hash = self.get_tx_hash_legacy();
        // // the cell is no longer needed, since we end in a panic
        // let mut tx_output = self.consume_output();
        // tx_output.async_call = Some(AsyncCallTxData {
        //     to: recipient,
        //     call_value: amount_value,
        //     call_data,
        //     tx_hash,
        // });
        // std::panic::panic_any(tx_output)
        panic!("async_call_raw not implemented yet");
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self>,
        _code: &ManagedBuffer<Self>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> (ManagedAddress<Self>, ManagedVec<Self, ManagedBuffer<Self>>) {
        panic!("deploy_contract not yet implemented")
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self>,
        _source_contract_address: &ManagedAddress<Self>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> (ManagedAddress<Self>, ManagedVec<Self, ManagedBuffer<Self>>) {
        panic!("deploy_from_source_contract not yet implemented")
    }

    fn upgrade_contract(
        &self,
        _sc_address: &ManagedAddress<Self>,
        _gas: u64,
        _amount: &BigUint<Self>,
        _code: &ManagedBuffer<Self>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) {
        panic!("upgrade_contract not yet implemented")
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("execute_on_dest_context_raw not implemented yet!");
    }

    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
        _range_closure: F,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        panic!("execute_on_dest_context_raw_custom_result_range not implemented yet!");
    }

    fn execute_on_dest_context_by_caller_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("execute_on_dest_context_by_caller_raw not implemented yet!");
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("execute_on_same_context_raw not implemented yet!");
    }

    fn execute_on_dest_context_readonly_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("execute_on_dest_context_readonly_raw not implemented yet!");
    }

    fn storage_store_tx_hash_key(&self, data: &ManagedBuffer<Self>) {
        let tx_hash = self.get_tx_hash_legacy();
        self.storage_store_slice_u8(tx_hash.as_bytes(), data.to_boxed_bytes().as_slice());
    }

    fn storage_load_tx_hash_key(&self) -> ManagedBuffer<Self> {
        let tx_hash = self.get_tx_hash_legacy();
        let bytes = self.storage_load_boxed_bytes(tx_hash.as_bytes());
        ManagedBuffer::new_from_bytes(self.clone(), bytes.as_slice())
    }

    fn call_local_esdt_built_in_function(
        &self,
        _gas: u64,
        _function_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("call_local_esdt_built_in_function not implemented yet!");
    }
}
