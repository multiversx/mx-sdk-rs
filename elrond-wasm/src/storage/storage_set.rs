use crate::*;
use elrond_codec::*;
use core::marker::PhantomData;

struct StorageSetOutput<'k, A, BigInt, BigUint>
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static
{
    api: A,
    key: &'k [u8],
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'k, A, BigInt, BigUint> StorageSetOutput<'k, A, BigInt, BigUint>
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>
{
    #[inline]
    fn new(api: A, key: &'k [u8]) -> Self {
        StorageSetOutput {
            api,
            key,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'k, A, BigInt, BigUint> TopEncodeOutput for StorageSetOutput<'k, A, BigInt, BigUint>
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static
{
    fn set_slice_u8(self, bytes: &[u8]) {
        self.api.storage_store_slice_u8(self.key, bytes)
    }

    fn set_u64(self, value: u64) {
        self.api.storage_store_u64(self.key, value);
    }

    fn set_i64(self, value: i64) {
        self.api.storage_store_i64(self.key, value);
    }
}

// #[inline]
pub fn storage_set<'k, A, BigInt, BigUint, T>(api: A, key: &'k [u8], value: &T)
where
    T: TopEncode,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static
{
    // the compiler is smart enough to evaluate this match at compile time
    match T::TYPE_INFO {
        TypeInfo::BigUint => {
            // self must be of type BigUint
            // performing a forceful cast
            let cast_big_uint: &BigUint = unsafe { &*(value as *const T as *const BigUint) };
            api.storage_store_big_uint(key, cast_big_uint);
        },
        _ => {
            value.top_encode_or_exit(
                StorageSetOutput::new(api.clone(), key),
                api.clone(),
                storage_set_exit
            );
        }
    }
}

#[inline(always)]
fn storage_set_exit<A, BigInt, BigUint>(api: A, encode_err: EncodeError) -> !
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static
{
    api.signal_error(encode_err.message_bytes())
}
