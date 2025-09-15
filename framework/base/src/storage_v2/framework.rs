use crate::{api, key::*};

use super::{
    root_field::RootFieldMut, Layout, LayoutFn, LayoutWithAbi, SelfRead, SelfWrite, StorageContext,
};

#[macro_export]
macro_rules! storage {
    ($key:ty, $method: ident, $method_mut: ident, $layout:ty) => {
        fn $method(&self) -> RootField<'_, $layout, $key> {
            RootField::new()
        }

        fn $method_mut(&mut self) -> RootFieldMut<'_, $layout, $key> {
            RootFieldMut::new()
        }
    };
}

#[macro_export]
macro_rules! const_key {
    ($key_type:ident, $const_str:expr) => {
        #[derive(Default)]
        pub struct $key_type;

        impl ConstKey for $key_type {
            fn root_key() -> &'static str {
                $const_str
            }
        }
    };
}

pub trait ContractBase {
    fn self_write_two(&mut self) -> (SelfWrite<'_>, SelfWrite<'_>) {
        (
            SelfWrite::new("a".to_owned()),
            SelfWrite::new("a".to_owned()),
        )
    }

    fn split_dont_call<'a, K1, L1, K2, L2>(
        &'a mut self,
        _fn1: fn(&'a mut Self) -> RootFieldMut<'a, L1, K1>,
        _fn2: fn(&'a mut Self) -> RootFieldMut<'a, L2, K2>,
    ) -> (RootFieldMut<'a, L1, K1>, RootFieldMut<'a, L2, K2>)
    where
        K1: ConstKey,
        L1: LayoutFn<SelfWrite<'a>>,
        K2: ConstKey,
        L2: LayoutFn<SelfWrite<'a>>,
    {
        (RootFieldMut::new(), RootFieldMut::new())
    }

    fn split_call<'a, K1, L1, K2, L2>(
        &'a mut self,
        fn1: fn(&'a mut Self) -> RootFieldMut<'a, L1, K1>,
        fn2: fn(&'a mut Self) -> RootFieldMut<'a, L2, K2>,
    ) -> (RootFieldMut<'a, L1, K1>, RootFieldMut<'a, L2, K2>)
    where
        K1: ConstKey,
        L1: LayoutFn<SelfWrite<'a>>,
        K2: ConstKey,
        L2: LayoutFn<SelfWrite<'a>>,
    {
        let also_self = unsafe { core::mem::transmute_copy::<&mut Self, &mut Self>(&self) };
        let rf1 = fn1(also_self);
        let rf2 = fn2(self);

        (rf1, rf2)
    }

    fn split_dont_call_3<'a, K1, L1, K2, L2, K3, L3>(
        &'a mut self,
        _fn1: fn(&'a mut Self) -> RootFieldMut<'a, L1, K1>,
        _fn2: fn(&'a mut Self) -> RootFieldMut<'a, L2, K2>,
        _fn3: fn(&'a mut Self) -> RootFieldMut<'a, L3, K3>,
    ) -> (
        RootFieldMut<'a, L1, K1>,
        RootFieldMut<'a, L2, K2>,
        RootFieldMut<'a, L3, K3>,
    )
    where
        K1: ConstKey,
        L1: LayoutFn<SelfWrite<'a>>,
        K2: ConstKey,
        L2: LayoutFn<SelfWrite<'a>>,
        K3: ConstKey,
        L3: LayoutFn<SelfWrite<'a>>,
    {
        (
            RootFieldMut::new(),
            RootFieldMut::new(),
            RootFieldMut::new(),
        )
    }
}
