use core::marker::PhantomData;

use super::super::{
    Layout, LayoutFn, LayoutWithAbi, StorageContext, StorageContextRead, StorageContextWrite,
};

pub struct VecStorage<ItemLayout, C = Layout>
where
    C: StorageContext,
    ItemLayout: LayoutWithAbi,
{
    context: C,
    pub len: usize,
    _phantom_item: PhantomData<ItemLayout>,
}

impl<C, ItemLayout> LayoutWithAbi for VecStorage<ItemLayout, C>
where
    C: StorageContext,
    ItemLayout: LayoutWithAbi,
{
}

impl<C, ItemLayout> LayoutFn<C> for VecStorage<ItemLayout, Layout>
where
    C: StorageContext,
    ItemLayout: LayoutFn<C>,
{
    type StorageOutput = VecStorage<ItemLayout, C>;

    fn build_storage(context: C) -> Self::StorageOutput {
        let len_context = context.downcast_read().subcontext(".len");
        let len = decode_len(&len_context.read_raw());
        VecStorage {
            context,
            len,
            _phantom_item: PhantomData,
        }
    }
}

impl<C, ItemLayout> VecStorage<ItemLayout, C>
where
    C: StorageContextRead,
    ItemLayout: LayoutFn<C>,
{
    pub fn length_context(&self) -> C {
        self.context.subcontext(".len")
    }

    pub fn item_context(&self, index: usize) -> C {
        assert!(index < self.len);
        let item_sub_key = format!(".item.{:04}", index);
        self.context.subcontext(&item_sub_key)
    }

    pub fn get(&self, index: usize) -> ItemLayout::StorageOutput {
        let item_context = self.item_context(index);
        ItemLayout::build_storage(item_context)
    }
}

impl<C, ItemLayout> VecStorage<ItemLayout, C>
where
    C: StorageContextWrite,
    ItemLayout: LayoutFn<C>,
{
    pub fn push(&mut self) -> ItemLayout::StorageOutput {
        let index = self.len;
        self.len += 1;
        let item_context = self.item_context(index);
        ItemLayout::build_storage(item_context)
    }
}

impl<C, ItemLayout> Drop for VecStorage<ItemLayout, C>
where
    C: StorageContext,
    ItemLayout: LayoutWithAbi,
{
    fn drop(&mut self) {
        if let Some(write_access) = self.context.try_downcast_write() {
            write_access
                .subcontext(".len")
                .write_raw(encode_len(self.len));
        }
    }
}

fn decode_len(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.parse().unwrap()
    }
}

fn encode_len(len: usize) -> String {
    if len == 0 {
        String::new()
    } else {
        len.to_string()
    }
}
