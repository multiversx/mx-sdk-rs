use crate::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
    TopDecodeMultiLength, TopEncodeMulti, TopEncodeMultiOutput,
};

macro_rules! multi_value_impls_debug {
        ($(($mv_struct:ident $len:tt $($n:tt $name:ident)+) )+) => {
        $(
            #[derive(Clone, Debug, PartialEq)]
            pub struct $mv_struct<$($name,)+>(pub ($($name,)+));
        )+
    }
}
macro_rules! multi_value_impls_no_debug {
        ($(($mv_struct:ident $len:tt $($n:tt $name:ident)+) )+) => {
        $(
            #[derive(Clone)]
            pub struct $mv_struct<$($name,)+>(pub ($($name,)+));
        )+
    }
}

macro_rules! multi_value_impls {
    ($(($mv_struct:ident $len:tt $($n:tt $name:ident)+) )+) => {
        $(
            impl<$($name),+> From<($($name,)+)> for $mv_struct<$($name,)+> {
                #[inline]
                fn from(tuple: ($($name,)+)) -> Self {
                    $mv_struct(tuple)
                }
            }

            impl<$($name,)+> $mv_struct<$($name,)+> {
                #[inline]
                pub fn into_tuple(self) -> ($($name,)+) {
                    self.0
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

            impl<$($name),+ > TopDecodeMultiLength for $mv_struct<$($name,)+>
            where
                $($name: TopDecodeMulti,)+
            {
                const LEN: usize = $len;
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
    (MultiValue12 12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiValue13 13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiValue14 14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiValue15 15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiValue16 16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
