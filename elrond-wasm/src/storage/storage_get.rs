use crate::*;
use elrond_codec::*;
use core::marker::PhantomData;

fn storage_get_error<'a, A, BigInt, BigUint>(api: &'a A, de_err: DecodeError) -> !
where
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    let mut decode_err_message: Vec<u8> = Vec::new();
    decode_err_message.extend_from_slice(err_msg::STORAGE_DECODE_ERROR);
    decode_err_message.extend_from_slice(de_err.message_bytes());
    api.signal_error(decode_err_message.as_slice())
}


struct StorageGetInput<'a, 'k, A, BigInt, BigUint>
where
    'a: 'k,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    api: &'a A,
    key: &'k [u8],
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, 'k, A, BigInt, BigUint> StorageGetInput<'a, 'k, A, BigInt, BigUint>
where
    'a: 'k,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    #[inline]
    fn new(api: &'a A, key: &'k [u8]) -> Self {
        StorageGetInput {
            api,
            key,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, 'k, A, BigInt, BigUint> TopDecodeInput for StorageGetInput<'a, 'k, A, BigInt, BigUint>
where
    'a: 'k,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    fn byte_len(&self) -> usize {
        self.api.storage_load_len(self.key)
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.api.storage_load_boxed_slice_u8(self.key)
    }

    fn into_u64(self) -> u64 {
        self.api.storage_load_u64(self.key)
    }

    fn into_i64(self) -> i64 {
        self.api.storage_load_i64(self.key)
    }
}

pub fn storage_get<'a, 'k, A, BigInt, BigUint, T>(api: &'a A, key: &'k [u8]) -> T
where
    'a: 'k,
    T: TopDecode,
    BigInt: NestedEncode + 'static,
    BigUint: NestedEncode + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    // the compiler is smart enough to evaluate this match at compile time
    match T::TYPE_INFO {
        TypeInfo::BigUint => {
            // self must be of type BigUint
            // performing a forceful cast
            let big_uint_value = api.storage_load_big_uint(key);
            let cast_big_uint: T = unsafe { core::mem::transmute_copy(&big_uint_value) };
            core::mem::forget(big_uint_value); // otherwise the data gets deallocated twice
            cast_big_uint
        },
        _ => {
            T::top_decode(StorageGetInput::new(api, key), |res| match res {
                Ok(v) => v,
                Err(de_err) => storage_get_error(api, de_err),
            })
        }
    }
}
