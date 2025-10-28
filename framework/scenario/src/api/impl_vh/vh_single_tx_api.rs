use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use multiversx_chain_vm_executor::{MemLength, MemPtr, VMHooksEarlyExit};

use multiversx_chain_vm::{
    blockchain::state::{AccountData, BlockConfig},
    host::{
        context::{BackTransfers, ManagedTypeContainer, TxFunctionName, TxInput, TxResult},
        vm_hooks::VMHooksContext,
    },
    schedule::GasSchedule,
    types::{VMAddress, VMCodeMetadata},
};

use crate::executor::debug::ContractDebugInstanceState;

const ZERO_GAS_SCHEDULE: GasSchedule = GasSchedule::zeroed();

#[derive(Default, Debug)]
pub struct SingleTxApiData {
    pub tx_input_box: Box<TxInput>,
    pub accounts: Mutex<HashMap<VMAddress, AccountData>>,
    pub managed_types: Mutex<ManagedTypeContainer>,
    pub tx_result_cell: Mutex<TxResult>,
    pub block_config: BlockConfig,
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
pub struct SingleTxApiVMHooksContext(Arc<SingleTxApiData>);

impl SingleTxApiVMHooksContext {
    pub fn with_mut_data<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut SingleTxApiData) -> R,
    {
        let data = Arc::get_mut(&mut self.0)
            .expect("could not retrieve mutable reference to SingleTxApi data");
        f(data)
    }
}

impl VMHooksContext for SingleTxApiVMHooksContext {
    unsafe fn memory_load(&self, offset: MemPtr, length: MemLength) -> Vec<u8> {
        let slice = unsafe { ContractDebugInstanceState::main_memory_load(offset, length) };
        slice.to_vec()
    }

    unsafe fn memory_store(&self, offset: MemPtr, data: &[u8]) {
        unsafe {
            ContractDebugInstanceState::main_memory_store(offset, data);
        }
    }

    fn m_types_lock(&self) -> MutexGuard<'_, ManagedTypeContainer> {
        self.0.managed_types.lock().unwrap()
    }

    fn gas_schedule(&self) -> &GasSchedule {
        &ZERO_GAS_SCHEDULE
    }

    fn use_gas(&mut self, _gas: u64) -> Result<(), VMHooksEarlyExit> {
        Ok(())
    }

    fn input_ref(&self) -> &TxInput {
        &self.0.tx_input_box
    }

    fn random_next_bytes(&self, _length: usize) -> Vec<u8> {
        panic!("cannot access the random bytes generator in the SingleTxApi")
    }

    fn result_lock(&self) -> MutexGuard<'_, TxResult> {
        self.0.tx_result_cell.lock().unwrap()
    }

    fn storage_read_any_address(&self, address: &VMAddress, key: &[u8]) -> Vec<u8> {
        self.0.with_account_mut(address, |account| {
            account.storage.get(key).cloned().unwrap_or_default()
        })
    }

    fn storage_write(&mut self, key: &[u8], value: &[u8]) -> Result<(), VMHooksEarlyExit> {
        self.0.with_account_mut(&self.0.tx_input_box.to, |account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
        Ok(())
    }

    fn get_block_config(&self) -> &BlockConfig {
        &self.0.block_config
    }

    fn back_transfers_lock(&self) -> MutexGuard<'_, BackTransfers> {
        panic!("cannot access back transfers in the SingleTxApi")
    }

    fn account_data(&self, address: &VMAddress) -> Option<AccountData> {
        Some(self.0.with_account_mut(address, |account| account.clone()))
    }

    fn account_code(&self, _address: &VMAddress) -> Vec<u8> {
        vec![]
    }

    fn perform_async_call(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the SingleTxApi")
    }

    fn perform_execute_on_dest_context(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> Result<TxResult, VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the SingleTxApi")
    }

    fn perform_execute_on_dest_context_readonly(
        &mut self,
        _to: VMAddress,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the SingleTxApi")
    }

    fn perform_deploy(
        &mut self,
        _egld_value: num_bigint::BigUint,
        _contract_code: Vec<u8>,
        _code_metadata: VMCodeMetadata,
        _args: Vec<Vec<u8>>,
    ) -> Result<(VMAddress, Vec<Vec<u8>>), VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the SingleTxApi")
    }

    fn perform_transfer_execute(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the SingleTxApi")
    }
}
