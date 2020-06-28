use crate::*;
// use crate::io::*;
use crate::esd_light::*;
use core::marker::PhantomData;

pub fn load_single_arg<'a, A, BigInt, BigUint, T>(api: &'a A, index: i32) -> T 
where
    T: Decode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    // the compiler is smart enough to evaluate this match at compile time
    match T::TYPE_INFO {
        TypeInfo::BigUint => {
            // self must be of type BigUint
            // performing a forceful cast
            let big_uint_arg = api.get_argument_big_uint(index);
            let cast_big_uint: T = unsafe { core::mem::transmute_copy(&big_uint_arg) };
            cast_big_uint
        },
        TypeInfo::I64 => {
            let arg_i64 = api.get_argument_i64(index);
            let arg_t: T = unsafe { core::mem::transmute_copy(&arg_i64) };
            arg_t
        },
        TypeInfo::I32 => {
            let arg_i64 = api.get_argument_i32(index);
            let arg_t: T = unsafe { core::mem::transmute_copy(&arg_i64) };
            arg_t
        },
        TypeInfo::I8 => {
            let arg_i64 = api.get_argument_i8(index);
            let arg_t: T = unsafe { core::mem::transmute_copy(&arg_i64) };
            arg_t
        },
        _ => {
            let arg_bytes = api.get_argument_vec(index);
            match esd_light::decode_from_byte_slice(arg_bytes.as_slice()) {
                Ok(v) => v,
                Err(de_err) => {
                    let mut decode_err_message: Vec<u8> = Vec::new();
                    decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR);
                    decode_err_message.extend_from_slice(de_err.message_bytes());
                    api.signal_error(decode_err_message.as_slice())
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
            api: api,
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

    fn next_arg(&mut self) -> Result<Option<T>, SCError> {
        if self.current_index >= self.num_arguments {
            Ok(None)
        } else {
            let arg: T = load_single_arg(self.api, self.current_index);
            self.current_index += 1;
            Ok(Some(arg))
        }
    }
}
