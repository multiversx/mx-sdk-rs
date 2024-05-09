use super::{ManagedVecItem, ManagedVecItemPayload};
use core::{cmp::Ordering, marker::PhantomData};

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
    pub(crate) fn decode(&self) -> T {
        T::from_byte_reader(|item_bytes| {
            item_bytes.copy_from_slice(self.encoded.payload_slice());
        })
    }
}

impl<T> PartialEq for EncodedManagedVecItem<T>
where
    T: PartialEq + ManagedVecItem,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.decode().eq(&other.decode())
    }
}

impl<T> Eq for EncodedManagedVecItem<T> where T: Eq + ManagedVecItem {}

impl<T> PartialOrd for EncodedManagedVecItem<T>
where
    T: PartialOrd + ManagedVecItem,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.decode().partial_cmp(&other.decode())
    }
}

impl<T> Ord for EncodedManagedVecItem<T>
where
    T: Ord + ManagedVecItem,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.decode().cmp(&other.decode())
    }
}
