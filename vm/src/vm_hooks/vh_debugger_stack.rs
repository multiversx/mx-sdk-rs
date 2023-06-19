use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

use multiversx_sc::{
    err_msg,
    types::{Address, CodeMetadata},
};

use crate::{
    tx_execution::{deploy_contract, execute_builtin_function_or_default},
    tx_mock::{
        async_call_tx_input, AsyncCallTxData, BlockchainUpdate, TxCache, TxContext, TxFunctionName,
        TxInput, TxManagedTypes, TxPanic, TxResult,
    },
    world_mock::{check_reserved_key, AccountData, BlockInfo},
};

use super::{
    VMHooksBigInt, VMHooksBlockchain, VMHooksCallValue, VMHooksCrypto, VMHooksEndpointArgument,
    VMHooksEndpointFinish, VMHooksError, VMHooksErrorManaged, VMHooksHandler, VMHooksHandlerSource,
    VMHooksLog, VMHooksManagedBuffer, VMHooksManagedTypes, VMHooksSend, VMHooksStorageRead,
    VMHooksStorageWrite,
};

/// A simple wrapper around a managed type container RefCell.
///
/// Implements `VMHooksManagedTypes` and thus can be used as a basis of a minimal static API.
#[derive(Debug)]
pub struct TxContextWrapper(Rc<TxContext>);

impl TxContextWrapper {
    pub fn new(tx_context_rc: Rc<TxContext>) -> Self {
        TxContextWrapper(tx_context_rc)
    }
}

impl VMHooksHandlerSource for TxContextWrapper {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes> {
        self.0.m_types_borrow()
    }

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes> {
        self.0.m_types_borrow_mut()
    }

    fn input_ref(&self) -> &TxInput {
        self.0.input_ref()
    }

    fn result_borrow_mut(&self) -> RefMut<TxResult> {
        self.0.result_borrow_mut()
    }

    fn storage_read_any_address(&self, address: &Address, key: &[u8]) -> Vec<u8> {
        self.0.with_account_mut(address, |account| {
            account.storage.get(key).cloned().unwrap_or_default()
        })
    }

    fn storage_write(&self, key: &[u8], value: &[u8]) {
        check_reserved_key(key);

        self.0.with_contract_account_mut(|account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        &self.0.blockchain_ref().previous_block_info
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        &self.0.blockchain_ref().current_block_info
    }

    fn account_data(&self, address: &Address) -> AccountData {
        self.0.with_account(address, |account| account.clone())
    }

    fn account_code(&self, address: &Address) -> Vec<u8> {
        self.0
            .blockchain_cache()
            .with_account(address, |account| account.contract_path.clone())
            .unwrap_or_else(|| panic!("Account is not a smart contract, it has no code"))
    }

    fn perform_async_call(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> ! {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        // the cell is no longer needed, since we end in a panic
        let mut tx_result = self.0.extract_result();
        tx_result.all_calls.push(async_call_data.clone());
        tx_result.pending_calls.async_call = Some(async_call_data);
        std::panic::panic_any(tx_result)
    }

    fn perform_execute_on_dest_context(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        // let tx_input = self.prepare_execute_on_dest_context_input(to, egld_value, func_name, args);
        let tx_input = async_call_tx_input(&async_call_data);
        let tx_cache = TxCache::new(self.0.blockchain_cache_rc());
        let (tx_result, blockchain_updates) =
            execute_builtin_function_or_default(tx_input, tx_cache);

        if tx_result.result_status == 0 {
            self.sync_call_post_processing(tx_result, blockchain_updates)
        } else {
            // also kill current execution
            std::panic::panic_any(TxPanic {
                status: tx_result.result_status,
                message: tx_result.result_message,
            })
        }
    }

    fn perform_deploy(
        &self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        _code_metadata: CodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> (Address, Vec<Vec<u8>>) {
        let contract_address = &self.input_ref().to;
        let tx_hash = self.tx_hash();
        let tx_input = TxInput {
            from: contract_address.clone(),
            to: Address::zero(),
            egld_value,
            esdt_values: Vec::new(),
            func_name: TxFunctionName::EMPTY,
            args,
            gas_limit: 1000,
            gas_price: 0,
            tx_hash,
            ..Default::default()
        };

        let tx_cache = TxCache::new(self.0.blockchain_cache_rc());
        tx_cache.increase_acount_nonce(contract_address);
        let (tx_result, new_address, blockchain_updates) =
            deploy_contract(tx_input, contract_code, tx_cache);

        if tx_result.result_status == 0 {
            (
                new_address,
                self.sync_call_post_processing(tx_result, blockchain_updates),
            )
        } else {
            // also kill current execution
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT.to_string(),
            })
        }
    }

    fn perform_transfer_execute(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        let tx_input = async_call_tx_input(&async_call_data);
        let tx_cache = TxCache::new(self.0.blockchain_cache_rc());
        let (tx_result, blockchain_updates) =
            execute_builtin_function_or_default(tx_input, tx_cache);

        if tx_result.result_status == 0 {
            self.0.result_borrow_mut().all_calls.push(async_call_data);

            let _ = self.sync_call_post_processing(tx_result, blockchain_updates);
        } else {
            // also kill current execution
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT.to_string(),
            })
        }
    }
}

impl TxContextWrapper {
    fn create_async_call_data(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> AsyncCallTxData {
        let contract_address = &self.0.input_ref().to;
        let tx_hash = self.tx_hash();
        AsyncCallTxData {
            from: contract_address.clone(),
            to,
            call_value: egld_value,
            endpoint_name: func_name,
            arguments,
            tx_hash,
        }
    }

    fn sync_call_post_processing(
        &self,
        tx_result: TxResult,
        blockchain_updates: BlockchainUpdate,
    ) -> Vec<Vec<u8>> {
        self.0.blockchain_cache().commit_updates(blockchain_updates);

        self.0.result_borrow_mut().merge_after_sync_call(&tx_result);

        tx_result.result_values
    }
}

impl VMHooksBigInt for TxContextWrapper {}
impl VMHooksManagedBuffer for TxContextWrapper {}
impl VMHooksManagedTypes for TxContextWrapper {}

impl VMHooksCallValue for TxContextWrapper {}
impl VMHooksEndpointArgument for TxContextWrapper {}
impl VMHooksEndpointFinish for TxContextWrapper {}
impl VMHooksError for TxContextWrapper {}
impl VMHooksErrorManaged for TxContextWrapper {}
impl VMHooksStorageRead for TxContextWrapper {}
impl VMHooksStorageWrite for TxContextWrapper {}
impl VMHooksCrypto for TxContextWrapper {}
impl VMHooksBlockchain for TxContextWrapper {}
impl VMHooksLog for TxContextWrapper {}
impl VMHooksSend for TxContextWrapper {}

impl VMHooksHandler for TxContextWrapper {}
