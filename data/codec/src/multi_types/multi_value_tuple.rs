#![allow(deprecated)]
#![allow(clippy::too_many_arguments)]

use crate::{
    DecodeErrorHandler, EncodeErrorHandler, MultiValueConstLength, MultiValueLength,
    TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
};

macro_rules! multi_value_impls_debug {
        ($(($mv_struct:ident $len:tt $($n:tt $name:ident)+) )+) => {
        $(
            #[derive(Clone, Debug, PartialEq)]
            pub struct $mv_struct<$($name,)+>(
                #[deprecated(since = "0.57.0", note = "use .into_tuple() or .as_tuple() instead")]
                pub ($($name,)+)
            );
        )+
    }
}
macro_rules! multi_value_impls_no_debug {
        ($(($mv_struct:ident $len:tt $($n:tt $name:ident)+) )+) => {
        $(
            #[derive(Clone)]
            pub struct $mv_struct<$($name,)+>(
                #[deprecated(since = "0.57.0", note = "use .into_tuple() or .as_tuple() instead")]
                pub ($($name,)+)
            );
        )+
    }
}

macro_rules! multi_value_impls {
    ($(($mv_struct:ident $len:tt $($n:tt $name:ident $p:ident)+) )+) => {
        $(
            impl<$($name),+> From<($($name,)+)> for $mv_struct<$($name,)+> {
                #[inline]
                fn from(tuple: ($($name,)+)) -> Self {
                    $mv_struct(tuple)
                }
            }

            impl<$($name,)+> $mv_struct<$($name,)+> {
                #[inline]
                pub fn new($($p: $name),+) -> Self {
                    $mv_struct(($($p,)+))
                }

                #[inline]
                pub fn into_tuple(self) -> ($($name,)+) {
                    self.0
                }

                #[inline]
                pub fn as_tuple(&self) -> &($($name,)+) {
                    &self.0
                }
            }

            impl<$($name),+ > TopEncodeMulti for $mv_struct<$($name,)+>
            where
                $($name: TopEncodeMulti,)+
            {
                fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
                where
                    O: TopEncodeMultiOutput,
                    H: EncodeErrorHandler,
                {
                    $(
                        (self.0).$n.multi_encode_or_handle_err(output, h)?;
                    )+
                    Ok(())
                }
            }

            impl<$($name),+ > MultiValueLength for $mv_struct<$($name,)+>
            where
                $($name: TopDecodeMulti + MultiValueLength,)+
            {
                fn multi_value_len(&self) -> usize {
                    0
                    $(
                        + <$name as MultiValueLength>::multi_value_len(&self.0.$n)
                    )+
                }
            }

            impl<$($name),+ > MultiValueConstLength for $mv_struct<$($name,)+>
            where
                $($name: TopDecodeMulti + MultiValueConstLength,)+
            {
                const MULTI_VALUE_CONST_LEN: usize = 0
                $(
                    + <$name as MultiValueConstLength>::MULTI_VALUE_CONST_LEN
                )+
                ;
            }

            impl<$($name),+ > TopDecodeMulti for $mv_struct<$($name,)+>
            where
                $($name: TopDecodeMulti,)+
            {
                fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
                where
                    I: TopDecodeMultiInput,
                    H: DecodeErrorHandler,
                {
                    Ok($mv_struct((
                        $(
                            $name::multi_decode_or_handle_err(input, h)?
                        ),+
                    )))
                }
            }
        )+
    }
}

multi_value_impls_debug! {
    (MultiValue2   2 0 T0 1 T1)
    (MultiValue3   3 0 T0 1 T1 2 T2)
    (MultiValue4   4 0 T0 1 T1 2 T2 3 T3)
    (MultiValue5   5 0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiValue6   6 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiValue7   7 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiValue8   8 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiValue9   9 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiValue10 10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiValue11 11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
}
// tuples with size 12+ don't implement Debug + PartialEq traits
// https://doc.rust-lang.org/std/primitive.tuple.html#trait-implementations-1
multi_value_impls_no_debug! {
    (MultiValue12 12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiValue13 13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiValue14 14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiValue15 15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiValue16 16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

multi_value_impls! {
    (MultiValue2   2 0 T0 v0 1 T1 v1)
    (MultiValue3   3 0 T0 v0 1 T1 v1 2 T2 v2)
    (MultiValue4   4 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3)
    (MultiValue5   5 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4)
    (MultiValue6   6 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5)
    (MultiValue7   7 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6)
    (MultiValue8   8 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7)
    (MultiValue9   9 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8)
    (MultiValue10 10 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8 9 T9 v9)
    (MultiValue11 11 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8 9 T9 v9 10 T10 v10)
    (MultiValue12 12 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8 9 T9 v9 10 T10 v10 11 T11 v11)
    (MultiValue13 13 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8 9 T9 v9 10 T10 v10 11 T11 v11 12 T12 v12)
    (MultiValue14 14 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8 9 T9 v9 10 T10 v10 11 T11 v11 12 T12 v12 13 T13 v13)
    (MultiValue15 15 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8 9 T9 v9 10 T10 v10 11 T11 v11 12 T12 v12 13 T13 v13 14 T14 v14)
    (MultiValue16 16 0 T0 v0 1 T1 v1 2 T2 v2 3 T3 v3 4 T4 v4 5 T5 v5 6 T6 v6 7 T7 v7 8 T8 v8 9 T9 v9 10 T10 v10 11 T11 v11 12 T12 v12 13 T13 v13 14 T14 v14 15 T15 v15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_value2_from_tuple() {
        let mv = MultiValue2::from((1u32, 2u64));
        assert_eq!(mv.into_tuple(), (1u32, 2u64));
    }

    #[test]
    fn multi_value2_as_tuple() {
        let mv = MultiValue2::from((10i32, "hello"));
        assert_eq!(mv.as_tuple(), &(10i32, "hello"));
    }

    #[test]
    fn multi_value3_from_tuple() {
        let mv = MultiValue3::from((1u8, 2u16, 3u32));
        assert_eq!(mv.into_tuple(), (1u8, 2u16, 3u32));
    }

    #[test]
    fn multi_value3_as_tuple() {
        let mv = MultiValue3::from((true, 42u64, false));
        assert_eq!(mv.as_tuple(), &(true, 42u64, false));
    }

    #[test]
    fn multi_value4_round_trip() {
        let original = (1u8, 2u16, 3u32, 4u64);
        let mv = MultiValue4::from(original);
        assert_eq!(mv.into_tuple(), original);
    }

    #[test]
    fn multi_value5_round_trip() {
        let original = (10u8, 20u16, 30u32, 40u64, true);
        let mv = MultiValue5::from(original);
        assert_eq!(mv.into_tuple(), original);
    }

    #[test]
    fn multi_value2_clone() {
        let mv = MultiValue2::from((42u32, 99u64));
        let mv_clone = mv.clone();
        assert_eq!(mv, mv_clone);
    }

    #[test]
    fn multi_value2_debug() {
        let mv = MultiValue2::from((1u32, 2u32));
        let debug_str = alloc::format!("{:?}", mv);
        assert!(debug_str.contains("MultiValue2"));
    }

    #[test]
    fn multi_value2_eq() {
        let mv1 = MultiValue2::from((1u32, 2u32));
        let mv2 = MultiValue2::from((1u32, 2u32));
        let mv3 = MultiValue2::from((1u32, 3u32));
        assert_eq!(mv1, mv2);
        assert_ne!(mv1, mv3);
    }

    #[test]
    fn multi_value_mixed_types() {
        let mv = MultiValue3::from((true, 255u8, 1_000_000u64));
        let (a, b, c) = mv.into_tuple();
        assert!(a);
        assert_eq!(b, 255u8);
        assert_eq!(c, 1_000_000u64);
    }

    #[test]
    fn multi_value11_round_trip() {
        let original = (0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8);
        let mv = MultiValue11::from(original);
        assert_eq!(mv.into_tuple(), original);
    }

    #[test]
    fn multi_value2_new() {
        let mv = MultiValue2::new(1u32, 2u64);
        assert_eq!(mv.into_tuple(), (1u32, 2u64));
    }

    #[test]
    fn multi_value3_new() {
        let mv = MultiValue3::new(true, 42u8, 100u64);
        assert_eq!(mv.into_tuple(), (true, 42u8, 100u64));
    }

    #[test]
    fn multi_value5_new() {
        let mv = MultiValue5::new(1u8, 2u16, 3u32, 4u64, false);
        assert_eq!(mv.into_tuple(), (1u8, 2u16, 3u32, 4u64, false));
    }

    #[test]
    fn multi_value2_new_eq_from() {
        let from_tuple = MultiValue2::from((10u32, 20u32));
        let from_new = MultiValue2::new(10u32, 20u32);
        assert_eq!(from_tuple, from_new);
    }
}
