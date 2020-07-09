use crate::*;
use elrond_codec::*;

#[inline]
pub fn storage_set<'a, 'k, A, BigInt, BigUint, T>(api: &'a A, key: &'k [u8], value: &T)
where
    'a: 'k,
    T: Encode,
    BigInt: Encode + 'static,
    BigUint: Encode + 'static,
    A: ContractHookApi<BigInt, BigUint> + 'a
{
    // the compiler is smart enough to evaluate this match at compile time
    match T::TYPE_INFO {
        TypeInfo::BigUint => {
            // self must be of type BigUint
            // performing a forceful cast
            let cast_big_uint: &BigUint = unsafe { &*(value as *const T as *const BigUint) };
            api.storage_store_big_uint(key, cast_big_uint);
        },
        TypeInfo::I64 | TypeInfo::U64 => {
            let value_i64: i64 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_i64);
        },
        TypeInfo::I32 => {
            let value_i32: i32 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_i32 as i64);
        },
        TypeInfo::U32 => {
            // we have to be a bit careful with sign extension
            let value_u32: u32 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_u32 as i64);
        },
        TypeInfo::I8 => {
            let value_i8: i8 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_i8 as i64);
        },
        TypeInfo::U8 => {
            // we have to be a bit careful with sign extension
            let value_u8: u8 = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, value_u8 as i64);
        },
        TypeInfo::Bool => {
            let value_bool: bool = unsafe { core::mem::transmute_copy(value) };
            api.storage_store_i64(key, if value_bool { 1i64 } else { 0i64 });
        },
        _ => {
            value.using_top_encoded(|bytes| {
                api.storage_store(key, bytes);
            });
        }
    }
}

#[inline]
pub fn storage_get<'a, 'k, A, BigInt, BigUint, T>(api: &'a A, key: &'k [u8]) -> T
where
    'a: 'k,
    T: Decode,
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
            cast_big_uint
        },
        TypeInfo::I64 | TypeInfo::U64 => {
            let value_i64 = match api.storage_load_i64(key) {
                Some(v) => v,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_i64) };
            value_t
        },
        TypeInfo::I32 => {
            let value_i32: i32 = match api.storage_load_i64(key) {
                Some(v) => v as i32,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_i32) };
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
        TypeInfo::I8 => {
            let value_i8: i8 = match api.storage_load_i64(key) {
                Some(v) => v as i8,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_i8) };
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
        TypeInfo::Bool => {
            let value_bool = match api.storage_load_i64(key) {
                Some(v) => v != 0,
                None => api.signal_error(err_msg::STORAGE_NOT_I64),
            };
            let value_t: T = unsafe { core::mem::transmute_copy(&value_bool) };
            value_t
        },
        _ => {
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
