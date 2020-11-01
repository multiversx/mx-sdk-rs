
use crate::*;
use elrond_codec::*;


#[inline]
pub fn load_single_arg<A, BigInt, BigUint, T>(api: A, index: i32, arg_id: ArgId) -> T 
where
    T: TopDecode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    // the compiler is smart enough to evaluate this match at compile time
    match T::TYPE_INFO {
        TypeInfo::BigInt => {
            let big_int_arg = api.get_argument_big_int(index);
            let cast_big_int: T = unsafe { core::mem::transmute_copy(&big_int_arg) };
            core::mem::forget(big_int_arg); // otherwise the data behind big_int_arg/cast_big_int gets deallocated twice
            cast_big_int
        },
        TypeInfo::BigUint => {
            let big_uint_arg = api.get_argument_big_uint(index);
            let cast_big_uint: T = unsafe { core::mem::transmute_copy(&big_uint_arg) };
            core::mem::forget(big_uint_arg); // otherwise the data gets deallocated twice
            cast_big_uint
        },
        _ => {
            match T::top_decode(ArgDecodeInput::new(api.clone(), index)) {
                Ok(v) => v,
                Err(de_err) => ApiSignalError::new(api).signal_arg_de_error(arg_id, de_err),
            }
            // T::top_decode_or_exit(
            //     ArgDecodeInput::new(api.clone(), index),
            //     &|de_err| ApiSignalError::new(api.clone()).signal_arg_de_error(arg_id, de_err))
        }
    }
}

/// It's easier to generate code from macros using this function, instead of the DynArg method.
#[inline]
pub fn load_dyn_arg<I, D, T>(loader: &mut D, arg_id: ArgId) -> T
where
    I: TopDecodeInput,
    D: DynArgInput<I>,
    T: DynArg<I, D>,
{
    T::dyn_load(loader, arg_id)
}

#[inline]
pub fn load_dyn_multi_arg<I, D, T>(loader: &mut D, arg_id: ArgId, num: usize) -> T
where
    I: TopDecodeInput,
    D: DynArgInput<I>,
    T: DynArgMulti<I, D>,
{
    T::dyn_load_multi(loader, arg_id, num)
}
