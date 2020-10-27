use crate::*;
use crate::arg_loader_err::load_arg_error;
use elrond_codec::*;
use core::marker::PhantomData;

struct ArgInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    api: &'a A,
    arg_index: i32,
    boxed_value: Box<[u8]>,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> ArgInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    #[inline]
    fn new(api: &'a A, arg_index: i32) -> Self {
        ArgInput {
            api,
            arg_index,
            boxed_value: Box::new([]),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint> TopDecodeInput for ArgInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    fn byte_len(&self) -> usize {
        self.api.get_argument_len(self.arg_index)
    }

    fn get_slice_u8(&mut self) -> &[u8] {
        self.boxed_value = self.api.get_argument_boxed_slice_u8(self.arg_index);
        &*self.boxed_value
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.api.get_argument_boxed_slice_u8(self.arg_index)
    }

    fn get_u64(&mut self) -> u64 {
        self.api.get_argument_u64(self.arg_index)
    }

    fn get_i64(&mut self) -> i64 {
        self.api.get_argument_i64(self.arg_index)
    }
}

#[inline]
pub fn load_single_arg<A, BigInt, BigUint, T>(api: &A, index: i32, arg_id: ArgId) -> T 
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
            T::top_decode(ArgInput::new(api, index), |res| match res {
                Ok(v) => v,
                Err(de_err) => load_arg_error(api, arg_id, de_err),
            })
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
    T: TopDecode,
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
