use crate::types::ManagedVecItem;
use core::{borrow::Borrow, fmt::Debug, marker::PhantomData, mem::ManuallyDrop, ops::Deref};

/// A reference to a type that implements ManagedVecItem.
///
/// Primarily used for preventing any mutability.
///
/// The names Ref and ManagedVecRef are interchangeable.
pub struct Ref<'a, T>
where
    T: ManagedVecItem,
{
    _phantom: PhantomData<&'a T>, // needed for the lifetime, even though T is present
    item: ManuallyDrop<T>,
}

/// The names ManagedVecRef and Ref are interchangeable.
pub type ManagedVecRef<'a, T> = Ref<'a, T>;

impl<T> Ref<'_, T>
where
    T: ManagedVecItem,
{
    /// Creates a new ManagedVecRef instance.
    ///
    /// ## Safety
    ///
    /// The ManagedVecRef object might not drop its content, effectively leading to a leak.
    pub unsafe fn new(item: T) -> Self {
        Ref {
            _phantom: PhantomData,
            item: ManuallyDrop::new(item),
        }
    }
}

impl<T> Drop for Ref<'_, T>
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

impl<T> Deref for Ref<'_, T>
where
    T: ManagedVecItem,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> Borrow<T> for Ref<'_, T>
where
    T: ManagedVecItem,
{
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T> AsRef<T> for Ref<'_, T>
where
    T: ManagedVecItem,
{
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> Debug for Ref<'_, T>
where
    T: ManagedVecItem + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.item.deref().fmt(f)
    }
}

impl<T1, T2> PartialEq<Ref<'_, T2>> for Ref<'_, T1>
where
    T1: ManagedVecItem + PartialEq<T2>,
    T2: ManagedVecItem,
{
    fn eq(&self, other: &Ref<'_, T2>) -> bool {
        self.deref().eq(other.deref())
    }
}
