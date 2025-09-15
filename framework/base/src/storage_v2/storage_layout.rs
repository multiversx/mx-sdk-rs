use super::{Layout, StorageContext};

pub trait LayoutWithAbi {
    fn abi() -> String {
        "stay tuned".to_owned()
    }
}

/// (Layout, Context) -> Storage
pub trait LayoutFn<C>: LayoutWithAbi
where
    C: StorageContext,
{
    type StorageOutput;

    fn build_storage(context: C) -> Self::StorageOutput;
}
