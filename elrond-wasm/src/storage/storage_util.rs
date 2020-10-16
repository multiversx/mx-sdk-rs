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

#[inline]
pub fn storage_get<'a, 'k, A, BigInt, BigUint, T>(api: &'a A, key: &'k [u8]) -> T
where
    'a: 'k,
    T: NestedDecode,
    BigInt: Encode + 'static,
    BigUint: Encode + 'static,
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
        TypeInfo::U64 => {
            let value_i64 = match api.storage_load_i64(key) {
                Some(v) => v,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_i64) };
            value_t
        },
        TypeInfo::U32 => {
            let value_u32: u32 = match api.storage_load_i64(key) {
                Some(v) => v as u32,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_u32) };
            value_t
        },
        TypeInfo::USIZE => {
            let value_usize: usize = match api.storage_load_i64(key) {
                Some(v) => v as usize,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_usize) };
            value_t
        },
        TypeInfo::U8 => {
            let value_u8: u8 = match api.storage_load_i64(key) {
                Some(v) => v as u8,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_u8) };
            value_t
        },
        _ => {
            if let Some(res_i64) = T::top_decode_from_i64_old(|| {
                match api.storage_load_i64(key) {
                    Some(v) => v,
                    None => api.signal_error(err_msg::STORAGE_NOT_I64),
                }
            }) {
                match res_i64 {
                    Ok(from_i64) => from_i64,
                    Err(de_err) => {
                        let mut decode_err_message: Vec<u8> = Vec::new();
                        decode_err_message.extend_from_slice(err_msg::STORAGE_DECODE_ERROR);
                        decode_err_message.extend_from_slice(de_err.message_bytes());
                        api.signal_error(decode_err_message.as_slice())
                    }
                }
            } else {
                let value_bytes = api.storage_load(key);
                match decode_from_byte_slice(value_bytes.as_slice()) {
                    Ok(v) => v,
                    Err(de_err) => {
                        let mut decode_err_message: Vec<u8> = Vec::new();
                        decode_err_message.extend_from_slice(err_msg::STORAGE_DECODE_ERROR);
                        decode_err_message.extend_from_slice(de_err.message_bytes());
                        api.signal_error(decode_err_message.as_slice())
                    },
                }
            }
        }
    }
}
