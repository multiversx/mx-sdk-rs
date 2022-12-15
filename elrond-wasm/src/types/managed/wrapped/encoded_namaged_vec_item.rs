use super::ManagedVecItem;
use core::{cmp::Ordering, marker::PhantomData};

pub(crate) struct EncodedManagedVecItem<T: ManagedVecItem, const N: usize>
where
    T: ManagedVecItem,
{
    encoded: [u8; N],
    _phantom: PhantomData<T>,
}

impl<T, const N: usize> EncodedManagedVecItem<T, N>
where
    T: ManagedVecItem,
{
    fn decode(&self) -> T {
        T::from_byte_reader(|item_bytes| {
            item_bytes.copy_from_slice(&self.encoded);
        })
    }
}

impl<T, const N: usize> PartialEq for EncodedManagedVecItem<T, N>
where
    T: PartialEq + ManagedVecItem,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.decode().eq(&other.decode())
    }
}

impl<T, const N: usize> Eq for EncodedManagedVecItem<T, N> where T: Eq + ManagedVecItem {}

impl<T, const N: usize> PartialOrd for EncodedManagedVecItem<T, N>
where
    T: PartialOrd + ManagedVecItem,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.decode().partial_cmp(&other.decode())
    }
}

impl<T, const N: usize> Ord for EncodedManagedVecItem<T, N>
where
    T: Ord + ManagedVecItem,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.decode().cmp(&other.decode())
    }
}
