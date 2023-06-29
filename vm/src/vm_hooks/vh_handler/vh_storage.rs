use crate::{
    types::{RawHandle, VMAddress},
    vm_hooks::VMHooksHandlerSource,
};

use super::VMHooksManagedTypes;

pub trait VMHooksStorageRead: VMHooksHandlerSource {
    fn storage_load_managed_buffer_raw(&self, key_handle: RawHandle, dest: RawHandle) {
        let value = self.storage_read(self.m_types_borrow().mb_get(key_handle));
        self.m_types_borrow_mut().mb_set(dest, value);
    }

    fn storage_load_from_address(
        &self,
        address_handle: RawHandle,
        key_handle: RawHandle,
        dest: RawHandle,
    ) {
        let address = VMAddress::from_slice(self.m_types_borrow().mb_get(address_handle));
        let value =
            self.storage_read_any_address(&address, self.m_types_borrow().mb_get(key_handle));
        self.m_types_borrow_mut().mb_set(dest, value);
    }
}

pub trait VMHooksStorageWrite: VMHooksHandlerSource + VMHooksManagedTypes {
    fn storage_store_managed_buffer_raw(&self, key_handle: RawHandle, value_handle: RawHandle) {
        self.storage_write(
            self.m_types_borrow().mb_get(key_handle),
            self.m_types_borrow().mb_get(value_handle),
        );
    }
}
