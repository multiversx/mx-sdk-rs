use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::{LayoutFn, StorageContext};

pub struct FieldUninitializedData<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    root_context: C,
    sub_key: &'static str,
    _phantom_layout: PhantomData<L>,
}

impl<C, L> FieldUninitializedData<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    pub fn new(root_context: C, sub_key: &'static str) -> Self {
        FieldUninitializedData {
            root_context,
            sub_key,
            _phantom_layout: PhantomData,
        }
    }
}

pub enum FieldData<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    New(FieldUninitializedData<C, L>),
    Loaded(L::StorageOutput),
}

impl<C, L> FieldData<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    pub fn is_loaded(&self) -> bool {
        matches!(self, FieldData::Loaded(_))
    }
}

pub struct Field<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    pub cell: UnsafeCell<FieldData<C, L>>,
}

impl<C, L> Field<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    pub fn new_lazy(root_context: C, sub_key: &'static str) -> Self {
        Field {
            cell: UnsafeCell::new(FieldData::New(FieldUninitializedData::new(
                root_context,
                sub_key,
            ))),
        }
    }
}

impl<C, L> Field<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    fn force_load(&self) {
        let data_ptr = self.cell.get();
        unsafe {
            let data = core::ptr::read(data_ptr);
            if let FieldData::New(init_data) = data {
                let context = init_data.root_context.subcontext(init_data.sub_key);
                let mapper = L::build_storage(context);
                core::ptr::write(data_ptr, FieldData::Loaded(mapper));
            } else {
                core::mem::forget(data);
            }
        }
    }

    fn mapper_ref(&self) -> &L::StorageOutput {
        let data_ptr = self.cell.get();
        unsafe {
            if let FieldData::Loaded(mapper) = &*data_ptr {
                mapper
            } else {
                panic!("failed initialization")
            }
        }
    }

    fn mapper_ref_mut(&mut self) -> &mut L::StorageOutput {
        let data_ptr = self.cell.get();
        unsafe {
            if let FieldData::Loaded(mapper) = &mut *data_ptr {
                mapper
            } else {
                panic!("failed initialization")
            }
        }
    }
}

impl<C, L> Deref for Field<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    type Target = L::StorageOutput;

    fn deref(&self) -> &Self::Target {
        self.force_load();
        self.mapper_ref()
    }
}

impl<C, L> DerefMut for Field<C, L>
where
    C: StorageContext,
    L: LayoutFn<C>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.force_load();
        self.mapper_ref_mut()
    }
}
