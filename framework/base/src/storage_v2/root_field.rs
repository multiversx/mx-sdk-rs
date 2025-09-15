use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::key::{ConstKey, DynamicKey, Key, StrKey};

use super::{
    Layout, LayoutFn, LayoutWithAbi, SelfRead, SelfWrite, StorageContext, StorageContextWrite,
};

pub struct RootField<'a, L, K>
where
    K: ConstKey,
    L: LayoutFn<SelfRead<'a>>,
{
    _key: PhantomData<K>,
    _context: PhantomData<SelfRead<'a>>,
    storage: <L as LayoutFn<SelfRead<'a>>>::StorageOutput,
}

impl<'a, L, K> RootField<'a, L, K>
where
    K: ConstKey,
    L: LayoutFn<SelfRead<'a>>,
{
    pub fn new() -> Self {
        let context = SelfRead::new(K::root_key().to_owned());
        RootField {
            _key: PhantomData,
            _context: PhantomData,
            storage: <L as LayoutFn<SelfRead>>::build_storage(context),
        }
    }
}

impl<'a, L, K> Deref for RootField<'a, L, K>
where
    K: ConstKey,
    L: LayoutFn<SelfRead<'a>>,
{
    type Target = L::StorageOutput;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

pub struct RootFieldMut<'a, L, K>
where
    K: ConstKey,
    L: LayoutFn<SelfWrite<'a>>,
{
    _key: PhantomData<K>,
    _context: PhantomData<SelfWrite<'a>>,
    storage: <L as LayoutFn<SelfWrite<'a>>>::StorageOutput,
}

impl<'a, L, K> RootFieldMut<'a, L, K>
where
    K: ConstKey,
    L: LayoutFn<SelfWrite<'a>>,
{
    pub fn new() -> Self {
        let context = SelfWrite::new(K::root_key().to_owned());
        RootFieldMut {
            _key: PhantomData,
            _context: PhantomData,
            storage: <L as LayoutFn<SelfWrite>>::build_storage(context),
        }
    }
}

impl<'a, L, K> Deref for RootFieldMut<'a, L, K>
where
    K: ConstKey,
    L: LayoutFn<SelfWrite<'a>>,
{
    type Target = L::StorageOutput;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<'a, L, K> DerefMut for RootFieldMut<'a, L, K>
where
    K: ConstKey,
    L: LayoutFn<SelfWrite<'a>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}
