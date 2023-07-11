use std::cell::{Ref, RefCell, RefMut};

use crate::{
    tx_mock::{TxFunctionName, TxInput, TxLog, TxManagedTypes, TxResult},
    types::{VMAddress, VMCodeMetadata},
    vm_hooks::{
        VMHooksBigFloat, VMHooksBigInt, VMHooksBlockchain, VMHooksCallValue, VMHooksCrypto,
        VMHooksEndpointArgument, VMHooksEndpointFinish, VMHooksError, VMHooksErrorManaged,
        VMHooksHandler, VMHooksHandlerSource, VMHooksLog, VMHooksManagedBuffer, VMHooksManagedMap,
        VMHooksManagedTypes, VMHooksSend, VMHooksStorageRead, VMHooksStorageWrite,
    },
    world_mock::{AccountData, BlockInfo},
};

/// A simple wrapper around a managed type container RefCell.
///
/// Implements `VMHooksManagedTypes` and thus can be used as a basis of a minimal static API.
#[derive(Debug, Default)]
pub struct TxManagedTypesCell(RefCell<TxManagedTypes>);

impl VMHooksHandlerSource for TxManagedTypesCell {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes> {
        self.0.borrow()
    }

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes> {
        self.0.borrow_mut()
    }

    fn halt_with_error(&self, status: u64, message: &str) -> ! {
        panic!("VM error occured, status: {status}, message: {message}")
    }

    fn input_ref(&self) -> &TxInput {
        panic!("cannot access tx inputs in the StaticApi")
    }

    fn random_next_bytes(&self, _length: usize) -> Vec<u8> {
        panic!("cannot access the random bytes generator in the StaticApi")
    }

    fn result_borrow_mut(&self) -> RefMut<TxResult> {
        panic!("cannot access tx results in the StaticApi")
    }

    fn push_tx_log(&self, _tx_log: TxLog) {
        panic!("cannot log events in the StaticApi")
    }

    fn storage_read_any_address(&self, _address: &VMAddress, _key: &[u8]) -> Vec<u8> {
        panic!("cannot access the storage in the StaticApi")
    }

    fn storage_write(&self, _key: &[u8], _value: &[u8]) {
        panic!("cannot access the storage in the StaticApi")
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        panic!("cannot access the block info in the StaticApi")
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        panic!("cannot access the block info in the StaticApi")
    }

    fn account_data(&self, _address: &VMAddress) -> AccountData {
        panic!("cannot access account data in the StaticApi")
    }

    fn account_code(&self, _address: &VMAddress) -> Vec<u8> {
        panic!("cannot access account data in the StaticApi")
    }

    fn perform_async_call(
        &self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> ! {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_execute_on_dest_context(
        &self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_deploy(
        &self,
        _egld_value: num_bigint::BigUint,
        _contract_code: Vec<u8>,
        _code_metadata: VMCodeMetadata,
        _args: Vec<Vec<u8>>,
    ) -> (VMAddress, Vec<Vec<u8>>) {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_transfer_execute(
        &self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) {
        panic!("cannot launch contract calls in the StaticApi")
    }
}

impl VMHooksBigInt for TxManagedTypesCell {}
impl VMHooksManagedBuffer for TxManagedTypesCell {}
impl VMHooksManagedMap for TxManagedTypesCell {}
impl VMHooksBigFloat for TxManagedTypesCell {}
impl VMHooksManagedTypes for TxManagedTypesCell {}

impl VMHooksCallValue for TxManagedTypesCell {}
impl VMHooksEndpointArgument for TxManagedTypesCell {}
impl VMHooksEndpointFinish for TxManagedTypesCell {}
impl VMHooksError for TxManagedTypesCell {}
impl VMHooksErrorManaged for TxManagedTypesCell {}
impl VMHooksStorageRead for TxManagedTypesCell {}
impl VMHooksStorageWrite for TxManagedTypesCell {}
impl VMHooksCrypto for TxManagedTypesCell {}
impl VMHooksBlockchain for TxManagedTypesCell {}
impl VMHooksLog for TxManagedTypesCell {}
impl VMHooksSend for TxManagedTypesCell {}

impl VMHooksHandler for TxManagedTypesCell {}
