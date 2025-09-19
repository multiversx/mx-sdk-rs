// #![allow(unused)]

// // use std::{
// //     any::TypeId,
// //     marker::PhantomData,
// //     ops::{Deref, DerefMut},
// //     ptr::NonNull,
// // };

// use crate::api;

use crate::{api::ManagedTypeApi, types::ManagedBuffer};

pub type DynamicKey<M> = ManagedBuffer<M>;

pub trait Key<M>: 'static
where
    M: ManagedTypeApi,
{
    fn full_key(&self) -> DynamicKey<M>;

    fn key_eq<Other: Key<M>>(&self, other: &Other) -> bool {
        self.full_key() == other.full_key()
    }

    fn append_to(&self, target: &mut DynamicKey<M>) {
        target.append_bytes(b".");
        target.append(&self.full_key());
    }
}

impl<M> Key<M> for DynamicKey<M>
where
    M: ManagedTypeApi,
{
    fn full_key(&self) -> DynamicKey<M> {
        self.clone()
    }
}

pub trait ConstKey<M>: Key<M> + Default
where
    M: ManagedTypeApi,
{
    fn root_key() -> &'static str;
}

// impl<K> Key for K
// where
//     K: ConstKey,
// {
//     fn full_key(&self) -> DynamicKey {
//         K::root_key().to_owned()
//     }

//     fn key_eq<Other: Key>(&self, other: &Other) -> bool {
//         TypeId::of::<Self>() == TypeId::of::<Other>()
//     }
// }

// pub struct StrKey(pub &'static str);

// impl Key for StrKey {
//     fn full_key(&self) -> DynamicKey {
//         self.0.to_owned()
//     }
// }

// pub trait StoragePath: Sized {
//     fn read_value_raw(&self) -> String;

//     fn maybe_write_value_raw(&mut self, value: &str);

//     unsafe fn duplicate_unchecked(&self) -> Self;

//     fn concat_key<K: Key>(self, key: K) -> Self;

//     // fn concat_key_ref<K: Key>(&self, key: K) -> Self {
//     //     unsafe { self.duplicate_unchecked().concat_key(key) }
//     // }

//     // fn concat_key_ref_mut<K: Key>(&mut self, key: K) -> Self {
//     //     unsafe { self.duplicate_unchecked().concat_key(key) }
//     // }
// }

// pub trait StoragePathMut: StoragePath {
//     fn write_value_raw(&mut self, value: &str);
// }

// pub trait StoragePathIntoRefMut<'a>: StoragePathMut {
//     type RefMut: StoragePathMut;

//     fn into_ref_mut(&'a mut self) -> Self::RefMut;
// }

// pub trait StoragePathConcat<K: Key> {
//     type Output: StoragePath;

//     fn concat(self, key: K) -> Self::Output;
// }

// // pub trait StoragePathMut {
// //     fn write_value_raw(&self, value: &str);

// //     unsafe fn duplicate_unchecked_mut(&self) -> Self;
// // }

// #[derive(Default)]
// pub struct SelfStorageRoot;

// #[derive(Default)]
// pub struct SelfStorageRootMut;

// pub struct SelfStorageRootRef<'a> {
//     _phantom: PhantomData<&'a SelfStorageRoot>,
// }

// impl<'a> SelfStorageRootRef<'a> {
//     pub unsafe fn new_ref() -> Self {
//         SelfStorageRootRef {
//             _phantom: PhantomData,
//         }
//     }

//     pub fn root_path(self, root_key: &str) -> SelfStorageRef<'a> {
//         SelfStorageRef {
//             source_ref: self,
//             key: root_key.to_owned(),
//         }
//     }
// }

// impl<'a> Deref for SelfStorageRootRef<'a> {
//     type Target = SelfStorageRoot;

//     // fn deref(&self) -> &Self::Target {
//     //     &SelfStorageRoot
//     // }
// }

// pub struct SelfStorageRootRefMut<'a> {
//     _phantom: PhantomData<&'a SelfStorageRoot>,
// }

// impl<'a> SelfStorageRootRefMut<'a> {
//     pub unsafe fn new_ref() -> Self {
//         SelfStorageRootRefMut {
//             _phantom: PhantomData,
//         }
//     }

//     pub fn root_path(self, root_key: &str) -> SelfStorageRefMut<'a> {
//         SelfStorageRefMut {
//             source_ref: self,
//             key: root_key.to_owned(),
//         }
//     }
// }

// // impl<'a> Deref for SelfStorageRootRefMut<'a> {
// //     type Target = SelfStorageRoot;

// //     fn deref(&self) -> &Self::Target {
// //         &SelfStorageRoot
// //     }
// // }

// // impl<'a> DerefMut for SelfStorageRootRefMut<'a> {
// //     fn deref_mut(&mut self) -> &mut Self::Target {
// //         let ptr = NonNull::<SelfStorageRoot>::dangling();
// //         unsafe { &mut *ptr.as_ptr() }
// //     }
// // }

// pub struct SelfStorageRef<'a> {
//     pub source_ref: SelfStorageRootRef<'a>,
//     pub key: DynamicKey,
// }

// impl<'a> StoragePath for SelfStorageRef<'a> {
//     fn read_value_raw(&self) -> String {
//         api::get(&self.key)
//     }

//     fn maybe_write_value_raw(&mut self, value: &str) {
//         // readonly
//     }

//     unsafe fn duplicate_unchecked(&self) -> Self {
//         SelfStorageRef {
//             source_ref: SelfStorageRootRef::new_ref(),
//             key: self.key.clone(),
//         }
//     }

//     fn concat_key<K: Key>(mut self, key: K) -> Self {
//         key.append_to(&mut self.key);
//         self
//     }
// }

// // impl<'a, K: Key> StoragePathConcat<K> for SelfStorageRootRef<'a> {
// //     type Output = SelfStorageRef<'a>;

// //     fn concat(self, key: K) -> Self::Output {
// //         SelfStorageRef {
// //             source_ref: self,
// //             key: key.full_key(),
// //         }
// //     }
// // }

// pub struct SelfStorageRefMut<'a> {
//     pub source_ref: SelfStorageRootRefMut<'a>,
//     pub key: DynamicKey,
// }

// impl<'a> StoragePath for SelfStorageRefMut<'a> {
//     fn read_value_raw(&self) -> String {
//         api::get(&self.key)
//     }

//     fn maybe_write_value_raw(&mut self, value: &str) {
//         api::set(self.key.clone(), value.to_owned());
//     }

//     unsafe fn duplicate_unchecked(&self) -> Self {
//         SelfStorageRefMut {
//             source_ref: SelfStorageRootRefMut::new_ref(),
//             key: self.key.clone(),
//         }
//     }

//     fn concat_key<K: Key>(mut self, key: K) -> Self {
//         key.append_to(&mut self.key);
//         self
//     }
// }

// impl<'a> StoragePathMut for SelfStorageRefMut<'a> {
//     fn write_value_raw(&mut self, value: &str) {
//         api::set(self.key.clone(), value.to_owned());
//     }
// }

// impl<'a> StoragePathIntoRefMut<'a> for SelfStorageRefMut<'a> {
//     type RefMut = SelfStorageRefMut<'a>;

//     fn into_ref_mut(&'a mut self) -> Self::RefMut {
//         unsafe { self.duplicate_unchecked() }
//     }
// }

// impl<'a, K: Key> StoragePathConcat<K> for SelfStorageRootRefMut<'a> {
//     type Output = SelfStorageRefMut<'a>;

//     fn concat(self, key: K) -> Self::Output {
//         SelfStorageRefMut {
//             source_ref: self,
//             key: key.full_key(),
//         }
//     }
// }

// pub trait StorageSource: Default + 'static {
//     fn can_write() -> bool;
// }

// impl StorageSource for SelfStorageRoot {
//     fn can_write() -> bool {
//         // println!("SelfStorage false");
//         false
//     }
// }
// impl StorageSource for SelfStorageRootMut {
//     fn can_write() -> bool {
//         // println!("SelfStorageMut true");
//         true
//     }
// }

// pub fn path_lifetimes(root: SelfStorageRootRefMut<'_>) {
//     let mut path1 = root.root_path("key1");
//     let path1a = path1.into_ref_mut().concat_key("key1a".to_owned());
//     // let path1b = path1.into_ref_mut().concat_key("key1a".to_owned());
//     // let path1a = path1.concat_key("key1a".to_owned());
// }
