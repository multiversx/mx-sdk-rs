use super::super::{Layout, LayoutFn, StorageContext, StorageContextRead, StorageContextWrite};

use super::LayoutWithAbi;

pub struct BorrowedStorage<C = Layout>
where
    C: StorageContext,
{
    context: C,
    pub value: String,
}

impl LayoutWithAbi for BorrowedStorage {}

impl<C> LayoutFn<C> for BorrowedStorage
where
    C: StorageContext,
{
    type StorageOutput = BorrowedStorage<C>;

    fn build_storage(context: C) -> Self::StorageOutput {
        let value = context.downcast_read().read_raw();
        BorrowedStorage { context, value }
    }
}

impl<C> BorrowedStorage<C>
where
    C: StorageContextRead,
{
    pub fn get(&self) -> &String {
        &self.value
    }
}

impl<C> BorrowedStorage<C>
where
    C: StorageContextWrite,
{
    pub fn set(&mut self, value: String) {
        self.value = value;
    }
}

impl<C> Drop for BorrowedStorage<C>
where
    C: StorageContext,
{
    fn drop(&mut self) {
        if let Some(write_access) = self.context.try_downcast_write() {
            write_access.write_raw(self.value.clone());
        }
    }
}
