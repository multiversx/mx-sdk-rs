use crate::*;
use elrond_codec::*;

#[inline]
pub fn storage_set<'a, 'k, A, BigInt, BigUint, T>(api: &'a A, key: &'k [u8], value: &T)
where
    'a: 'k,
    T: Encode,
    BigInt: Encode + 'static,
    BigUint: Encode + 'static,
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
        TypeInfo::U64 => {
            let value_i64: i64 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_i64);
        },
        TypeInfo::U32 => {
            // we have to be a bit careful with sign extension
            let value_u32: u32 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_u32 as i64);
        },
        TypeInfo::USIZE => {
            // we have to be a bit careful with sign extension
            let value_usize: usize = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_usize as i64);
        },
        TypeInfo::U8 => {
            // we have to be a bit careful with sign extension
            let value_u8: u8 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_u8 as i64);
        },
        _ => {
            // the compiler is also smart enough to evaluate this if let at compile time
            if let Some(res_i64) = value.top_encode_as_i64() {
                match res_i64 {
                    Ok(encoded_i64) => {
                        api.storage_store_i64(key, encoded_i64);
                    },
                    Err(encode_err_message) => {
                        api.signal_error(encode_err_message.message_bytes());
                    }
                }
            } else {
                let res = value.using_top_encoded(|bytes| {
                    api.storage_store(key, bytes);
                });
                if let Err(encode_err_message) = res {
                    api.signal_error(encode_err_message.message_bytes());
                }
            }
        }
    }
}
