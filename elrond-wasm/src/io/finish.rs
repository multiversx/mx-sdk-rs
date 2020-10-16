use crate::*;
use crate::elrond_codec::*;
use core::iter::FromIterator;


pub trait EndpointResult<'a, A, BigInt, BigUint>: Sized
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a 
{
    fn finish(&self, api: &'a A);
}

impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for T
where
    T: Encode,
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
                // the compiler is also smart enough to evaluate this if let at compile time
                if let Some(res_i64) = self.top_encode_as_i64() {
                    match res_i64 {
                        Ok(encoded_i64) => {
                            api.finish_i64(encoded_i64);
                        },
                        Err(encode_err_message) => {
                            api.signal_error(encode_err_message.message_bytes());
                        }
                    }
                } else {
                    let res = self.using_top_encoded(|buf| api.finish_slice_u8(buf));
                    if let Err(encode_err_message) = res {
                        api.signal_error(encode_err_message.message_bytes());
                    }
                }
			}
		}
    }
}

pub struct MultiResultVec<T>(pub Vec<T>);

impl<T> MultiResultVec<T> {
    #[inline]
    pub fn new() -> Self {
        MultiResultVec(Vec::new())
    }
}

impl<T> Default for MultiResultVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for MultiResultVec<T> {
    fn from(v: Vec<T>) -> Self {
        MultiResultVec(v)
    }
}

impl<T> FromIterator<T> for MultiResultVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let v = Vec::<T>::from_iter(iter);
        MultiResultVec(v)
    }
}

impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for MultiResultVec<T>
where
    T: EndpointResult<'a, A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    #[inline]
    fn finish(&self, api: &'a A) {
        for elem in self.0.iter() {
            elem.finish(api);
        }
    }
}

pub enum OptionalResult<T> {
    Some(T),
    None
}

impl<T> From<Option<T>> for OptionalResult<T> {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(result) => OptionalResult::Some(result),
            None => OptionalResult::None,
        }
    }
}

impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for OptionalResult<T>
where
    T: EndpointResult<'a, A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    #[inline]
    fn finish(&self, api: &'a A) {
        if let OptionalResult::Some(t) = self {
            t.finish(api);
        }
    }
}

macro_rules! multi_result_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<'a, A, BigInt, BigUint, $($name),+> EndpointResult<'a, A, BigInt, BigUint> for $mr<$($name,)+>
            where
                $($name: EndpointResult<'a, A, BigInt, BigUint>,)+
                BigInt: BigIntApi<BigUint> + 'static,
                BigUint: BigUintApi + 'static,
                A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
            {
                #[inline]
				fn finish(&self, api: &'a A) {
                    $(
                        (self.0).$n.finish(api);
                    )+
                }
            }

            impl<$($name),+> From<($($name,)+)> for $mr<$($name,)+> {
                #[inline]
                fn from(tuple: ($($name,)+)) -> Self {
                    $mr(tuple)
                }
            }
        )+
    }
}

multi_result_impls! {
    (MultiResult1  0 T0)
    (MultiResult2  0 T0 1 T1)
    (MultiResult3  0 T0 1 T1 2 T2)
    (MultiResult4  0 T0 1 T1 2 T2 3 T3)
    (MultiResult5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiResult6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiResult7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiResult8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiResult9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiResult10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiResult11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiResult12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiResult13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiResult14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiResult15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiResult16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for BorrowedMutStorage<'a, A, BigInt, BigUint, T>
where
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a,
    T: Encode + NestedDecode + EndpointResult<'a, A, BigInt, BigUint>,
{
    fn finish(&self, api: &'a A) {
        core::ops::Deref::deref(self).finish(api);
    }
}
