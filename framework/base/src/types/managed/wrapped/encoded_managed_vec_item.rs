use super::ManagedVecItem;
use core::{cmp::Ordering, marker::PhantomData};

pub struct EncodedManagedVecItem<T: ManagedVecItem>
where
    T: ManagedVecItem,
    [(); <T as ManagedVecItem>::PAYLOAD_SIZE]:,
{
    pub encoded: [u8; <T as ManagedVecItem>::PAYLOAD_SIZE],
    _phantom: PhantomData<T>,
}

impl<T> EncodedManagedVecItem<T>
where
    T: ManagedVecItem,
    [(); <T as ManagedVecItem>::PAYLOAD_SIZE]:,
{
    pub(crate) fn decode(&self) -> T {
        T::from_byte_reader(|item_bytes| {
            item_bytes.copy_from_slice(&self.encoded);
        })
    }
}

impl<T> PartialEq for EncodedManagedVecItem<T>
where
    T: PartialEq + ManagedVecItem,
    [(); <T as ManagedVecItem>::PAYLOAD_SIZE]:,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.decode().eq(&other.decode())
    }
}

impl<T> Eq for EncodedManagedVecItem<T>
where
    T: Eq + ManagedVecItem,
    [(); <T as ManagedVecItem>::PAYLOAD_SIZE]:,
{
}

impl<T> PartialOrd for EncodedManagedVecItem<T>
where
    T: PartialOrd + ManagedVecItem,
    [(); <T as ManagedVecItem>::PAYLOAD_SIZE]:,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.decode().partial_cmp(&other.decode())
    }
}

impl<T> Ord for EncodedManagedVecItem<T>
where
    T: Ord + ManagedVecItem,
    [(); <T as ManagedVecItem>::PAYLOAD_SIZE]:,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.decode().cmp(&other.decode())
    }
}
