use crate::*;
use elrond_codec::*;
use core::marker::PhantomData;

pub fn load_single_arg<A, BigInt, BigUint, T>(api: &A, index: i32, arg_id: ArgId) -> T 
where
    T: Decode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint>
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
            // the compiler is also smart enough to evaluate this if let at compile time
            if let Some(res_i64) = T::top_decode_from_i64(|| api.get_argument_i64(index)) {
                match res_i64 {
                    Ok(from_i64) => from_i64,
                    Err(de_err) => {
                        let mut decode_err_message: Vec<u8> = Vec::new();
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_1);
                        decode_err_message.extend_from_slice(arg_id);
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_2);
                        decode_err_message.extend_from_slice(de_err.message_bytes());
                        api.signal_error(decode_err_message.as_slice())
                    }
                }
            } else {
                let arg_bytes = api.get_argument_vec(index);
                match elrond_codec::decode_from_byte_slice(arg_bytes.as_slice()) {
                    Ok(v) => v,
                    Err(de_err) => {
                        let mut decode_err_message: Vec<u8> = Vec::new();
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_1);
                        decode_err_message.extend_from_slice(arg_id);
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_2);
                        decode_err_message.extend_from_slice(de_err.message_bytes());
                        api.signal_error(decode_err_message.as_slice())
                    }
                }
            }
        }
    }
}

pub struct DynEndpointArgLoader<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    api: &'a A,
    current_index: i32,
    num_arguments: i32,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> DynEndpointArgLoader<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    pub fn new(api: &'a A) -> Self {
        DynEndpointArgLoader {
            api,
            current_index : 0,
            num_arguments: api.get_num_arguments(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint, T> DynArgLoader<T> for DynEndpointArgLoader<'a, A, BigInt, BigUint>
where
    T: Decode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    #[inline]
    fn has_next(&self) -> bool {
        self.current_index < self.num_arguments
    }

    fn next_arg(&mut self, arg_id: ArgId) -> Result<Option<T>, SCError> {
        if self.current_index >= self.num_arguments {
            Ok(None)
        } else {
            let arg: T = load_single_arg(self.api, self.current_index, arg_id);
            self.current_index += 1;
            Ok(Some(arg))
        }
    }
}
