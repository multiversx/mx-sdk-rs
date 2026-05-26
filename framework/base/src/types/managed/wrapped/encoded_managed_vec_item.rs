use super::ManagedVecItem;
use core::{borrow::Borrow, cmp::Ordering, marker::PhantomData};

pub struct EncodedManagedVecItem<T: ManagedVecItem>
where
    T: ManagedVecItem,
{
    pub encoded: T::PAYLOAD,
    _phantom: PhantomData<T>,
}

impl<T> EncodedManagedVecItem<T>
where
    T: ManagedVecItem,
{
    pub(crate) fn decode(&self) -> T::Ref<'_> {
        unsafe { T::borrow_from_payload(&self.encoded) }
    }
}

impl<T> PartialEq for EncodedManagedVecItem<T>
where
    T: PartialEq + ManagedVecItem,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let self_ref = self.decode();
        let other_ref = other.decode();
        self_ref.borrow().eq(other_ref.borrow())
    }
}

impl<T> Eq for EncodedManagedVecItem<T> where T: Eq + ManagedVecItem {}

impl<T> PartialOrd for EncodedManagedVecItem<T>
where
    T: PartialOrd + ManagedVecItem,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.decode().borrow().partial_cmp(other.decode().borrow())
    }
}

impl<T> Ord for EncodedManagedVecItem<T>
where
    T: Ord + ManagedVecItem,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.decode().borrow().cmp(other.decode().borrow())
    }
}
