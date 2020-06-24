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

macro_rules! tuple_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<A, BigInt, BigUint, $($name),+> EndpointResult<A, BigInt, BigUint> for $mr<$($name,)+>
            where
                $($name: EndpointResult<A, BigInt, BigUint>,)+
                BigInt: BigIntApi<BigUint> + 'static,
                BigUint: BigUintApi + 'static,
                A: ContractIOApi<BigInt, BigUint> + 'static
            {
                #[inline]
				fn finish(&self, api: &A) {
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

tuple_impls! {
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