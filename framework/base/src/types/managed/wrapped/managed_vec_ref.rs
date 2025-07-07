use crate::types::ManagedVecItem;
use core::{borrow::Borrow, fmt::Debug, marker::PhantomData, mem::ManuallyDrop, ops::Deref};

pub struct ManagedVecRef<'a, T>
where
    T: ManagedVecItem,
{
    _phantom: PhantomData<&'a T>, // needed for the lifetime, even though T is present
    item: ManuallyDrop<T>,
}

impl<T> ManagedVecRef<'_, T>
where
    T: ManagedVecItem,
{
    /// Creates a new ManagedVecRef instance.
    ///
    /// ## Safety
    ///
    /// The ManagedVecRef object might not drop its content, effectively leading to a leak.
    pub unsafe fn new(item: T) -> Self {
        ManagedVecRef {
            _phantom: PhantomData,
            item: ManuallyDrop::new(item),
        }
    }
}

impl<T> Drop for ManagedVecRef<'_, T>
where
    T: ManagedVecItem,
{
    fn drop(&mut self) {
        // TODO: improve
        unsafe {
            ManuallyDrop::drop(&mut self.item);
        }
    }
}

impl<T> Deref for ManagedVecRef<'_, T>
where
    T: ManagedVecItem,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> Borrow<T> for ManagedVecRef<'_, T>
where
    T: ManagedVecItem,
{
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T> Debug for ManagedVecRef<'_, T>
where
    T: ManagedVecItem + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.item.deref().fmt(f)
    }
}

impl<T1, T2> PartialEq<ManagedVecRef<'_, T2>> for ManagedVecRef<'_, T1>
where
    T1: ManagedVecItem + PartialEq<T2>,
    T2: ManagedVecItem,
{
    fn eq(&self, other: &ManagedVecRef<'_, T2>) -> bool {
        self.deref().eq(other.deref())
    }
}
