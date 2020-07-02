use crate::*;
use core::ops::Deref;
use core::ops::DerefMut;
use core::marker::PhantomData;

enum BorrowedMutStorageKey {
    Const(&'static [u8]),
    Generated(Vec<u8>),
}

impl BorrowedMutStorageKey {
    fn as_bytes<'k>(&'k self) -> &'k [u8] {
        match self {
            BorrowedMutStorageKey::Generated(v) => v.as_slice(),
            BorrowedMutStorageKey::Const(v) => v,
        }
    }
}

pub struct BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: Encode + 'static,
    BigUint: Encode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: Encode + Decode,
{
    api: &'a A,
    key: BorrowedMutStorageKey,
    value: T,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint, T> BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: Encode + 'static,
    BigUint: Encode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: Encode + Decode,
{
    pub fn with_const_key(api: &'a A, key: &'static [u8]) -> Self {
        let value: T = storage_get(api, key);
        BorrowedMutStorage {
            api: api,
            key : BorrowedMutStorageKey::Const(key),
            value: value,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn with_generated_key(api: &'a A, key: Vec<u8>) -> Self {
        let value: T = storage_get(api, key.as_slice());
        BorrowedMutStorage {
            api: api,
            key : BorrowedMutStorageKey::Generated(key),
            value: value,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}


impl<'a, A, BigInt, BigUint, T> Drop for BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: Encode + 'static,
    BigUint: Encode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: Encode + Decode,
{
    fn drop(&mut self) {
        storage_set(self.api, self.key.as_bytes(), &self.value);
    }
}

impl<'a, A, BigInt, BigUint, T> Deref for BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: Encode + 'static,
    BigUint: Encode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: Encode + Decode,
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
    T: Encode + Decode,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
