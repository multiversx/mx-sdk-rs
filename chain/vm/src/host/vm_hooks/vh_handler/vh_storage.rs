use multiversx_chain_vm_executor::VMHooksError;

use crate::{
    host::vm_hooks::VMHooksHandlerSource,
    types::{RawHandle, VMAddress},
};

use super::VMHooksManagedTypes;

pub trait VMHooksStorageRead: VMHooksHandlerSource {
    fn storage_load_managed_buffer_raw(
        &mut self,
        key_handle: RawHandle,
        dest: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.storage_load)?;

        let value = self.storage_read(self.m_types_lock().mb_get(key_handle));
        self.m_types_lock().mb_set(dest, value);

        Ok(())
    }

    fn storage_load_from_address(
        &mut self,
        address_handle: RawHandle,
        key_handle: RawHandle,
        dest: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.storage_load)?;

        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let value = self.storage_read_any_address(&address, self.m_types_lock().mb_get(key_handle));
        self.m_types_lock().mb_set(dest, value);

        Ok(())
    }
}

pub trait VMHooksStorageWrite: VMHooksHandlerSource + VMHooksManagedTypes {
    fn storage_store_managed_buffer_raw(
        &mut self,
        key_handle: RawHandle,
        value_handle: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.storage_store)?;

        let types = self.m_types_lock();
        let key_bytes = types.mb_get_owned(key_handle);
        let value_bytes = types.mb_get_owned(value_handle);
        std::mem::drop(types);
        self.storage_write(&key_bytes, &value_bytes);

        Ok(())
    }
}
