elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait StorageApiFeatures {
    #[endpoint]
    fn storage_read_raw(&self, storage_key: ManagedBuffer) -> ManagedBuffer {
        self.storage_raw().read(&storage_key.into())
    }

    #[endpoint]
    fn storage_write_raw(&self, storage_key: ManagedBuffer, value: ManagedBuffer) {
        self.storage_raw().write(&storage_key.into(), &value);
    }

    #[endpoint]
    fn storage_read_from_address(
        &self,
        address: ManagedAddress,
        storage_key: ManagedBuffer,
    ) -> ManagedBuffer {
        self.storage_raw()
            .read_from_address(&address, &storage_key.into())
    }
}
