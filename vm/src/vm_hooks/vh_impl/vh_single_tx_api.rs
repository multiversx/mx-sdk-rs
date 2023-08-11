use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{
    tx_mock::{TxFunctionName, TxInput, TxManagedTypes, TxResult},
    types::{VMAddress, VMCodeMetadata},
    vm_hooks::{
        VMHooksBigFloat, VMHooksBigInt, VMHooksBlockchain, VMHooksCallValue, VMHooksCrypto,
        VMHooksEndpointArgument, VMHooksEndpointFinish, VMHooksError, VMHooksErrorManaged,
        VMHooksHandler, VMHooksHandlerSource, VMHooksLog, VMHooksManagedBuffer, VMHooksManagedMap,
        VMHooksManagedTypes, VMHooksSend, VMHooksStorageRead, VMHooksStorageWrite,
    },
    world_mock::{AccountData, BlockInfo},
};

#[derive(Default, Debug)]
pub struct SingleTxApiData {
    pub tx_input_box: Box<TxInput>,
    pub accounts: Mutex<HashMap<VMAddress, AccountData>>,
    pub managed_types: Mutex<TxManagedTypes>,
    pub tx_result_cell: Mutex<TxResult>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
}

impl SingleTxApiData {
    pub fn with_account_mut<R, F>(&self, address: &VMAddress, f: F) -> R
    where
        F: FnOnce(&mut AccountData) -> R,
    {
        let mut accounts = self.accounts.lock().unwrap();
        let account = accounts
            .entry(address.clone())
            .or_insert(AccountData::new_empty(address.clone()));
        f(account)
    }
}

#[derive(Default, Debug, Clone)]
pub struct SingleTxApiVMHooksHandler(Arc<SingleTxApiData>);

impl SingleTxApiVMHooksHandler {
    pub fn with_mut_data<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut SingleTxApiData) -> R,
    {
        let data = Arc::get_mut(&mut self.0)
            .expect("could not retrieve mutable reference to SingleTxApi data");
        f(data)
    }
}

impl VMHooksHandlerSource for SingleTxApiVMHooksHandler {
    fn m_types_lock(&self) -> MutexGuard<TxManagedTypes> {
        self.0.managed_types.lock().unwrap()
    }

    fn halt_with_error(&self, status: u64, message: &str) -> ! {
        panic!("VM error occured, status: {status}, message: {message}")
    }

    fn input_ref(&self) -> &TxInput {
        &self.0.tx_input_box
    }

    fn random_next_bytes(&self, _length: usize) -> Vec<u8> {
        panic!("cannot access the random bytes generator in the SingleTxApi")
    }

    fn result_lock(&self) -> MutexGuard<TxResult> {
        self.0.tx_result_cell.lock().unwrap()
    }

    fn storage_read_any_address(&self, address: &VMAddress, key: &[u8]) -> Vec<u8> {
        self.0.with_account_mut(address, |account| {
            account.storage.get(key).cloned().unwrap_or_default()
        })
    }

    fn storage_write(&self, key: &[u8], value: &[u8]) {
        self.0.with_account_mut(&self.0.tx_input_box.to, |account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        &self.0.previous_block_info
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        &self.0.current_block_info
    }

    fn account_data(&self, address: &VMAddress) -> AccountData {
        self.0.with_account_mut(address, |account| account.clone())
    }

    fn account_code(&self, _address: &VMAddress) -> Vec<u8> {
        vec![]
    }

    fn perform_async_call(
        &self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> ! {
        panic!("cannot launch contract calls in the SingleTxApi")
    }

    fn perform_execute_on_dest_context(
        &self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        panic!("cannot launch contract calls in the SingleTxApi")
    }

    fn perform_deploy(
        &self,
        _egld_value: num_bigint::BigUint,
        _contract_code: Vec<u8>,
        _code_metadata: VMCodeMetadata,
        _args: Vec<Vec<u8>>,
    ) -> (VMAddress, Vec<Vec<u8>>) {
        panic!("cannot launch contract calls in the SingleTxApi")
    }

    fn perform_transfer_execute(
        &self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) {
        panic!("cannot launch contract calls in the SingleTxApi")
    }
}

impl VMHooksBigInt for SingleTxApiVMHooksHandler {}
impl VMHooksManagedBuffer for SingleTxApiVMHooksHandler {}
impl VMHooksManagedMap for SingleTxApiVMHooksHandler {}
impl VMHooksBigFloat for SingleTxApiVMHooksHandler {}
impl VMHooksManagedTypes for SingleTxApiVMHooksHandler {}

impl VMHooksCallValue for SingleTxApiVMHooksHandler {}
impl VMHooksEndpointArgument for SingleTxApiVMHooksHandler {}
impl VMHooksEndpointFinish for SingleTxApiVMHooksHandler {}
impl VMHooksError for SingleTxApiVMHooksHandler {}
impl VMHooksErrorManaged for SingleTxApiVMHooksHandler {}
impl VMHooksStorageRead for SingleTxApiVMHooksHandler {}
impl VMHooksStorageWrite for SingleTxApiVMHooksHandler {}
impl VMHooksCrypto for SingleTxApiVMHooksHandler {}
impl VMHooksBlockchain for SingleTxApiVMHooksHandler {}
impl VMHooksLog for SingleTxApiVMHooksHandler {}
impl VMHooksSend for SingleTxApiVMHooksHandler {}

impl VMHooksHandler for SingleTxApiVMHooksHandler {}
