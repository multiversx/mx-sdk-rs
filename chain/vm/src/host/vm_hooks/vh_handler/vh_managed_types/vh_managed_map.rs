use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{
    host::vm_hooks::{VMHooksContext, VMHooksHandler},
    types::RawHandle,
};

impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn mm_new(&mut self) -> Result<RawHandle, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().managed_map_api_cost.managed_map_new)?;
        Ok(self.context.m_types_lock().mm_new())
    }

    pub fn mm_get(
        &mut self,
        map_handle: RawHandle,
        key_handle: RawHandle,
        out_value_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().managed_map_api_cost.managed_map_get)?;
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        let value = self
            .context
            .m_types_lock()
            .mm_values_get(map_handle, key.as_slice());
        self.context.m_types_lock().mb_set(out_value_handle, value);
        Ok(())
    }

    pub fn mm_put(
        &mut self,
        map_handle: RawHandle,
        key_handle: RawHandle,
        value_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().managed_map_api_cost.managed_map_put)?;
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        let value = self.context.m_types_lock().mb_get(value_handle).to_vec();
        self.context
            .m_types_lock()
            .mm_values_insert(map_handle, key, value);
        Ok(())
    }

    pub fn mm_remove(
        &mut self,
        map_handle: RawHandle,
        key_handle: RawHandle,
        out_value_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().managed_map_api_cost.managed_map_remove)?;
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        let value = self
            .context
            .m_types_lock()
            .mm_values_remove(map_handle, key.as_slice());
        self.context.m_types_lock().mb_set(out_value_handle, value);
        Ok(())
    }

    pub fn mm_contains(
        &mut self,
        map_handle: RawHandle,
        key_handle: RawHandle,
    ) -> Result<bool, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_map_api_cost
                .managed_map_contains,
        )?;
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        Ok(self
            .context
            .m_types_lock()
            .mm_contains(map_handle, key.as_slice()))
    }

    pub fn mm_drop(&self, map_handle: RawHandle) {
        self.context.m_types_lock().mm_remove(map_handle);
    }
}
