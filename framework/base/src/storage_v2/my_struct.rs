use super::{
    BorrowedStorage, Field, Layout, LayoutFn, LayoutWithAbi, StorageContext, StorageContextRead,
    VecStorage,
};

// #[derive(StorageMapper)]
pub struct MyStructA<C = Layout>
where
    C: StorageContext,
{
    // #[key("myValue")]
    // pub my_value: field!(BorrowedStorage),
    pub my_value: Field<C, BorrowedStorage>,

    // pub my_value: field!(VecStorage<BorrowedStorage>),
    pub my_list: Field<C, VecStorage<BorrowedStorage>>,

    pub vec_of_vec: Field<C, VecStorage<VecStorage<BorrowedStorage>>>,

    pub vec_of_b: Field<C, VecStorage<MyStructB>>,
}

impl LayoutWithAbi for MyStructA<Layout> {
    fn abi() -> String {
        "
MyStructA {
    my_value   -> BorrowedStorage
    my_list    -> VecStorage<BorrowedStorage>
    vec_of_vec -> VecStorage<VecStorage<BorrowedStorage>>
    vec_of_b   -> VecStorage<MyStructB>
}
        "
        .to_owned()
    }
}

impl<C> LayoutFn<C> for MyStructA<Layout>
where
    C: StorageContext,
{
    type StorageOutput = MyStructA<C>;

    fn build_storage(context: C) -> Self::StorageOutput {
        unsafe {
            MyStructA {
                my_value: Field::new_lazy(context.unsafe_clone(), ".my-value"),
                my_list: Field::new_lazy(context.unsafe_clone(), ".my-list"),
                vec_of_vec: Field::new_lazy(context.unsafe_clone(), ".vec-of-vec"),
                vec_of_b: Field::new_lazy(context.unsafe_clone(), ".vec_of_b"),
            }
        }
    }
}

pub struct MyStructB<C = Layout>
where
    C: StorageContext,
{
    pub b1: Field<C, BorrowedStorage>,

    pub b2: Field<C, VecStorage<BorrowedStorage>>,
}

impl LayoutWithAbi for MyStructB<Layout> {}

impl<C> LayoutFn<C> for MyStructB<Layout>
where
    C: StorageContext,
{
    type StorageOutput = MyStructB<C>;

    fn build_storage(context: C) -> Self::StorageOutput {
        unsafe {
            MyStructB {
                b1: Field::new_lazy(context.unsafe_clone(), ".b1"),
                b2: Field::new_lazy(context.unsafe_clone(), ".b2"),
            }
        }
    }
}
