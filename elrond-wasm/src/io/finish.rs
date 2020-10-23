use crate::*;
use crate::elrond_codec::*;
use core::marker::PhantomData;

struct ApiOutput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    api: &'a A,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> ApiOutput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    #[inline]
    fn new(api: &'a A) -> Self {
        ApiOutput {
            api,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint> TopEncodeOutput for ApiOutput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    fn set_slice_u8(self, bytes: &[u8]) {
        self.api.finish_slice_u8(bytes);
    }

    fn set_u64(self, value: u64) {
        self.api.finish_u64(value);
    }

    fn set_i64(self, value: i64) {
        self.api.finish_i64(value);
    }
    
}

pub trait EndpointResult<'a, A, BigInt, BigUint>: Sized
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a 
{
    fn finish(&self, api: &'a A);
}

/// All serializable objects can be used as smart contract function result.
impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for T
where
    T: TopEncode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    fn finish(&self, api: &'a A) {
        // the compiler is smart enough to evaluate this match at compile time
        match T::TYPE_INFO {
            TypeInfo::Unit => {},
            TypeInfo::BigInt => {
                let cast_big_int: &BigInt = unsafe { &*(self as *const T as *const BigInt) };
                api.finish_big_int(cast_big_int);
            },
            TypeInfo::BigUint => {
                let cast_big_uint: &BigUint = unsafe { &*(self as *const T as *const BigUint) };
                api.finish_big_uint(cast_big_uint);
            },
			_ => {
                let res = self.top_encode(ApiOutput::new(api));
                if let Err(encode_err_message) = res {
                    api.signal_error(encode_err_message.message_bytes());
                }
			}
		}
    }
}
