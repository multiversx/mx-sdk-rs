multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageRawApiFeatures {
    #[endpoint]
    fn storage_read_raw(&self, storage_key: ManagedBuffer) -> ManagedBuffer {
        self.storage_raw().read(storage_key)
    }

    #[endpoint]
    fn storage_write_raw(&self, storage_key: ManagedBuffer, value: ManagedBuffer) {
        self.storage_raw().write(storage_key, &value);
    }

    #[endpoint]
    fn storage_read_from_address(
        &self,
        address: ManagedAddress,
        storage_key: ManagedBuffer,
    ) -> ManagedBuffer {
        self.storage_raw().read_from_address(&address, storage_key)
    }
}
