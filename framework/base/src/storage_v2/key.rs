use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use alloc::borrow::ToOwned;

use crate::{
    api::{
        ErrorApi, ManagedTypeApi, StorageReadApi, StorageReadApiImpl, StorageWriteApi,
        StorageWriteApiImpl,
    },
    storage::StorageKey,
    types::{ManagedBuffer, ManagedType},
};

#[derive(Default, Clone)]
pub struct DynamicKey<A>(StorageKey<A>)
where
    A: ManagedTypeApi + ErrorApi + 'static;

impl<A> PartialEq for DynamicKey<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<A> Deref for DynamicKey<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    type Target = StorageKey<A>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> DerefMut for DynamicKey<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(dead_code)]
pub trait Key<A>: 'static
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn full_key(&self) -> DynamicKey<A>;

    fn key_eq<Other: Key<A>>(&self, other: &Other) -> bool {
        self.full_key().eq(&other.full_key())
    }

    fn append_to(&self, target: &mut DynamicKey<A>) {
        target.append_bytes(b".");
        target.append_managed_buffer(&self.full_key().buffer);
    }
}

impl<A> Key<A> for DynamicKey<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn full_key(&self) -> DynamicKey<A> {
        self.clone()
    }
}

#[allow(dead_code)]
pub trait ConstKey<A>: Key<A> + Default
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn root_key() -> &'static ManagedBuffer<A>;
}

// impl<A, K> Key<A> for K
// where
//     K: ConstKey<A>,
//     A: ManagedTypeApi + ErrorApi + 'static,
// {
//     fn full_key(&self) -> DynamicKey<A> {
//         K::root_key().to_owned()
//     }

//     fn key_eq<Other: Key<A>>(&self, other: &Other) -> bool {
//         false
//         // TypeId::of::<Self>() == TypeId::of::<Other>()
//     }
// }

#[allow(dead_code)]
pub struct StrKey(&'static str);

impl Deref for StrKey {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> Key<A> for StrKey
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn full_key(&self) -> DynamicKey<A> {
        DynamicKey(StorageKey::new(self.as_bytes()))
    }
}

#[allow(dead_code)]
pub trait StoragePath<A>: Sized
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn read_value_raw(&self) -> StorageKey<A>;

    fn maybe_write_value_raw(&mut self, value: &ManagedBuffer<A>);

    unsafe fn duplicate_unchecked(&self) -> Self;

    fn concat_key<K: Key<A>>(self, key: K) -> Self;

    // fn concat_key_ref<K: Key>(&self, key: K) -> Self {
    //     unsafe { self.duplicate_unchecked().concat_key(key) }
    // }

    // fn concat_key_ref_mut<K: Key>(&mut self, key: K) -> Self {
    //     unsafe { self.duplicate_unchecked().concat_key(key) }
    // }
}

#[allow(dead_code)]
pub trait StoragePathMut<A>: StoragePath<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn write_value_raw(&mut self, value: &ManagedBuffer<A>);
}

#[allow(dead_code)]
pub trait StoragePathIntoRefMut<'a, A>: StoragePathMut<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    type RefMut: StoragePathMut<A>;

    fn as_ref_mut(&'a mut self) -> Self::RefMut;
}

#[allow(dead_code)]
pub trait StoragePathConcat<A, K>
where
    K: Key<A>,
    A: ManagedTypeApi + ErrorApi + 'static,
{
    type Output: StoragePath<A>;

    fn concat(self, key: K) -> Self::Output;
}

// pub trait StoragePathMut {
//     fn write_value_raw(&self, value: &str);

//     unsafe fn duplicate_unchecked_mut(&self) -> Self;
// }

#[derive(Default)]
pub struct SelfStorageRoot;

#[allow(dead_code)]
#[derive(Default)]
pub struct SelfStorageRootMut;

#[allow(dead_code)]
pub struct SelfStorageRootRef<'a, A> {
    _phantom: PhantomData<(&'a SelfStorageRoot, A)>,
}

#[allow(dead_code)]
impl<'a, A> SelfStorageRootRef<'a, A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    pub unsafe fn new_ref() -> Self {
        SelfStorageRootRef {
            _phantom: PhantomData,
        }
    }

    pub fn root_path(self, root_key: &DynamicKey<A>) -> SelfStorageRef<'a, A> {
        SelfStorageRef {
            source_ref: self,
            key: root_key.to_owned(),
        }
    }
}

// impl<'a> Deref for SelfStorageRootRef<'a> {
//     type Target = SelfStorageRoot;

//     // fn deref(&self) -> &Self::Target {
//     //     &SelfStorageRoot
//     // }
// }

pub struct SelfStorageRootRefMut<'a, A> {
    _phantom: PhantomData<(&'a SelfStorageRoot, A)>,
}

#[allow(dead_code)]
impl<'a, A> SelfStorageRootRefMut<'a, A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    pub unsafe fn new_ref() -> Self {
        SelfStorageRootRefMut {
            _phantom: PhantomData,
        }
    }

    pub fn root_path(self, root_key: &DynamicKey<A>) -> SelfStorageRefMut<'a, A> {
        SelfStorageRefMut {
            source_ref: self,
            key: root_key.to_owned(),
        }
    }
}

impl<'a, _A> Deref for SelfStorageRootRefMut<'a, _A> {
    type Target = SelfStorageRoot;

    fn deref(&self) -> &Self::Target {
        &SelfStorageRoot
    }
}

impl<'a, _A> DerefMut for SelfStorageRootRefMut<'a, _A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = NonNull::<SelfStorageRoot>::dangling();
        unsafe { &mut *ptr.as_ptr() }
    }
}

#[allow(dead_code)]
pub struct SelfStorageRef<'a, A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    pub source_ref: SelfStorageRootRef<'a, A>,
    pub key: DynamicKey<A>,
}

impl<'a, A> StoragePath<A> for SelfStorageRef<'a, A>
where
    A: StorageWriteApi + StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    fn read_value_raw(&self) -> StorageKey<A> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            A::storage_read_api_impl()
                .storage_load_managed_buffer_raw(self.key.get_handle(), result.get_handle());
            StorageKey::from(result)
        }
    }

    fn maybe_write_value_raw(&mut self, value: &ManagedBuffer<A>) {
        // read-only
        let storage_path = self.key.clone();
        A::storage_write_api_impl()
            .storage_store_managed_buffer_raw(storage_path.get_handle(), value.get_handle());
    }

    unsafe fn duplicate_unchecked(&self) -> Self {
        SelfStorageRef {
            source_ref: SelfStorageRootRef::new_ref(),
            key: self.key.clone(),
        }
    }

    fn concat_key<K: Key<A>>(mut self, key: K) -> Self {
        key.append_to(&mut self.key);
        self
    }
}

// impl<'a, K: Key> StoragePathConcat<K> for SelfStorageRootRef<'a> {
//     type Output = SelfStorageRef<'a>;

//     fn concat(self, key: K) -> Self::Output {
//         SelfStorageRef {
//             source_ref: self,
//             key: key.full_key(),
//         }
//     }
// }

#[allow(dead_code)]
pub struct SelfStorageRefMut<'a, A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    pub source_ref: SelfStorageRootRefMut<'a, A>,
    pub key: DynamicKey<A>,
}

impl<'a, A> StoragePath<A> for SelfStorageRefMut<'a, A>
where
    A: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    fn read_value_raw(&self) -> StorageKey<A> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            A::storage_read_api_impl()
                .storage_load_managed_buffer_raw(self.key.get_handle(), result.get_handle());
            StorageKey::from(result)
        }
    }

    fn maybe_write_value_raw(&mut self, value: &ManagedBuffer<A>) {
        A::storage_write_api_impl()
            .storage_store_managed_buffer_raw(self.key.get_handle(), value.get_handle());
    }

    unsafe fn duplicate_unchecked(&self) -> Self {
        SelfStorageRefMut {
            source_ref: SelfStorageRootRefMut::new_ref(),
            key: self.key.clone(),
        }
    }

    fn concat_key<K: Key<A>>(mut self, key: K) -> Self {
        key.append_to(&mut self.key);
        self
    }
}

impl<'a, A> StoragePathMut<A> for SelfStorageRefMut<'a, A>
where
    A: StorageWriteApi + StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    fn write_value_raw(&mut self, value: &ManagedBuffer<A>) {
        A::storage_write_api_impl()
            .storage_store_managed_buffer_raw(self.key.get_handle(), value.get_handle());
    }
}

impl<'a, A> StoragePathIntoRefMut<'a, A> for SelfStorageRefMut<'a, A>
where
    A: StorageWriteApi + StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    type RefMut = SelfStorageRefMut<'a, A>;

    fn as_ref_mut(&'a mut self) -> Self::RefMut {
        unsafe { self.duplicate_unchecked() }
    }
}

impl<'a, A, K> StoragePathConcat<A, K> for SelfStorageRootRefMut<'a, A>
where
    K: Key<A>,
    A: StorageWriteApi + StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    type Output = SelfStorageRefMut<'a, A>;

    fn concat(self, key: K) -> Self::Output {
        SelfStorageRefMut {
            source_ref: self,
            key: key.full_key(),
        }
    }
}

#[allow(dead_code)]
pub trait StorageSource: Default + 'static {
    fn can_write() -> bool;
}

impl StorageSource for SelfStorageRoot {
    fn can_write() -> bool {
        // println!("SelfStorage false");
        false
    }
}
impl StorageSource for SelfStorageRootMut {
    fn can_write() -> bool {
        // println!("SelfStorageMut true");
        true
    }
}

pub fn _path_lifetimes<A>(root: SelfStorageRootRefMut<'_, A>)
where
    A: StorageWriteApi + StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    let mut path1 = root.root_path(&DynamicKey(StorageKey::new(b"root")));
    let _path1a = path1.as_ref_mut().concat_key(StrKey("key1a"));
    // let path1b = path1.into_ref_mut().concat_key("key1a".to_owned());
    // let path1a = path1.concat_key("key1a".to_owned());
}
