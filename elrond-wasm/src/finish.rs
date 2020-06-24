use super::*;
use crate::esd_light::*;

pub trait EndpointResult<A, BigInt, BigUint>: Sized
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static 
{
    fn finish(&self, api: &A);
}

impl<A, BigInt, BigUint, T> EndpointResult<A, BigInt, BigUint> for T
where
    T: Encode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    #[inline]
    fn finish(&self, api: &A) {
        match T::TYPE_INFO {
            TypeInfo::Unit => {},
			TypeInfo::BigUint => {
                // self must be of type BigUint
                // performing a forceful cast
                let cast_big_uint: &BigUint = unsafe { core::mem::transmute(self) };
                api.finish_big_uint(cast_big_uint);
			},
			_ => {
				self.using_top_encoded(|buf| api.finish_slice_u8(buf));
			}
		}
    }
}

impl<A, BigInt, BigUint, T, E> EndpointResult<A, BigInt, BigUint> for Result<T, E>
where
    T: EndpointResult<A, BigInt, BigUint>,
    E: ErrorMessage,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    #[inline]
    fn finish(&self, api: &A) {
        match self {
            Ok(t) => {
                t.finish(api);
            },
            Err(e) => {
                e.with_message_slice(|buf| api.signal_error(buf));
            }
        }
    }
}

pub struct MultiResultVec<T>(pub Vec<T>);

impl<T> From<Vec<T>> for MultiResultVec<T> {
    fn from(v: Vec<T>) -> Self {
        MultiResultVec(v)
    }
}

impl<A, BigInt, BigUint, T> EndpointResult<A, BigInt, BigUint> for MultiResultVec<T>
where
    T: EndpointResult<A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    #[inline]
    fn finish(&self, api: &A) {
        for elem in self.0.iter() {
            elem.finish(api);
        }
    }
}

pub enum OptionalResult<T> {
    Some(T),
    None
}

impl<A, BigInt, BigUint, T> EndpointResult<A, BigInt, BigUint> for OptionalResult<T>
where
    T: EndpointResult<A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    #[inline]
    fn finish(&self, api: &A) {
        if let OptionalResult::Some(t) = self {
            t.finish(api);
        }
    }
}
