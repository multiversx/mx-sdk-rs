use super::super::{
    Layout, LayoutFn, LayoutWithAbi, StorageContext, StorageContextRead, StorageContextWrite,
};

pub struct SingleValueStorage<C = Layout>
where
    C: StorageContext,
{
    context: C,
}

impl LayoutWithAbi for SingleValueStorage {}

impl<C> LayoutFn<C> for SingleValueStorage<Layout>
where
    C: StorageContext,
{
    type StorageOutput = SingleValueStorage<C>;

    fn build_storage(context: C) -> Self::StorageOutput {
        SingleValueStorage { context }
    }
}

impl<C> SingleValueStorage<C>
where
    C: StorageContextRead,
{
    pub fn get(&self) -> String {
        self.context.read_raw()
    }
}

impl<C> SingleValueStorage<C>
where
    C: StorageContextWrite,
{
    pub fn set(&mut self, value: String) {
        self.context.write_raw(value);
    }
}
