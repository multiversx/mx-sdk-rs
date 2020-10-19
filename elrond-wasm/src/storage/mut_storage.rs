use crate::*;
use core::ops::Deref;
use core::ops::DerefMut;
use core::marker::PhantomData;
use elrond_codec::*;

/// Internal key container for BorrowedMutStorage.
enum BorrowedMutStorageKey {
    Const(&'static [u8]),
    Generated(Vec<u8>),
}

impl BorrowedMutStorageKey {
    fn as_bytes(&self) -> &[u8] {
        match self {
            BorrowedMutStorageKey::Generated(v) => v.as_slice(),
            BorrowedMutStorageKey::Const(v) => v,
        }
    }
}

/// Contains a value taken from storage and a reference to the api.
/// The value can be changed and will be saved back to storage 
/// when the lifetime of the BorrowedMutStorage expires.
/// Optimization: will only save back to storage if the value is referenced with deref_mut(),
/// because only in such way can it be changed.
pub struct BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: NestedEncode + TopDecode,
{
    api: &'a A,
    key: BorrowedMutStorageKey,
    value: T,
    dirty: bool,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint, T> BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: NestedEncode + TopDecode,
{
    pub fn with_const_key(api: &'a A, key: &'static [u8]) -> Self {
        let value: T = storage_get(api, key);
        BorrowedMutStorage {
            api,
            key : BorrowedMutStorageKey::Const(key),
            value,
            dirty: false,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn with_generated_key(api: &'a A, key: Vec<u8>) -> Self {
        let value: T = storage_get(api, key.as_slice());
        BorrowedMutStorage {
            api,
            key : BorrowedMutStorageKey::Generated(key),
            value,
            dirty: false,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}


impl<'a, A, BigInt, BigUint, T> Drop for BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: NestedEncode + TopDecode,
{
    fn drop(&mut self) {
        if self.dirty {
            storage_set(self.api, self.key.as_bytes(), &self.value);
        }
    }
}

impl<'a, A, BigInt, BigUint, T> Deref for BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: NestedEncode + TopDecode,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<'a, A, BigInt, BigUint, T> DerefMut for BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: NestedEncode + TopDecode,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.value
    }
}
