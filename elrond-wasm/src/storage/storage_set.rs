use crate::*;
use elrond_codec::*;
use core::marker::PhantomData;

fn storage_set_error<'a, A, BigInt, BigUint>(api: &'a A, encode_err: EncodeError) -> !
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    // TODO: more verbose error messages?
    api.signal_error(encode_err.message_bytes())
}

struct StorageSetOutput<'a, 'k, A, BigInt, BigUint>
where
    'a: 'k,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    api: &'a A,
    key: &'k [u8],
    buffer: Vec<u8>,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, 'k, A, BigInt, BigUint> StorageSetOutput<'a, 'k, A, BigInt, BigUint>
where
    'a: 'k,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    #[inline]
    fn new(api: &'a A, key: &'k [u8]) -> Self {
        StorageSetOutput {
            api,
            key,
            buffer: Vec::new(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, 'k, A, BigInt, BigUint> TopEncodeOutput<'a, Vec<u8>> for StorageSetOutput<'a, 'k, A, BigInt, BigUint>
where
    'a: 'k,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    fn set_slice_u8(self, bytes: &[u8]) {
        self.api.storage_store_slice_u8(self.key, bytes)
    }

    fn buffer_ref<'r>(&'r mut self) -> &'r mut Vec<u8>
    where 'a: 'r {
        &mut self.buffer
    }

    fn flush_buffer(self) {
        self.api.storage_store_slice_u8(self.key, self.buffer.as_slice());
    }

    fn set_u64(self, value: u64) {
        self.api.storage_store_u64(self.key, value);
    }

    fn set_i64(self, value: i64) {
        self.api.storage_store_i64(self.key, value);
    }
}

// #[inline]
pub fn storage_set<'a, 'k, A, BigInt, BigUint, T>(api: &'a A, key: &'k [u8], value: &T)
where
    'a: 'k,
    T: TopEncode,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
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
            match value.top_encode(StorageSetOutput::new(api, key)) {
                Ok(v) => v,
                Err(encode_err) => storage_set_error(api, encode_err),
            }
        }
    }
}
