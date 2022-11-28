use crate::{
    top_decode_from_nested_or_handle_err, DecodeErrorHandler, EncodeErrorHandler, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
    TopEncodeOutput,
};

macro_rules! tuple_impls {
    ($(($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TopEncode for ($($name,)+)
            where
                $($name: NestedEncode,)+
            {
				fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
                where
                    O: TopEncodeOutput,
                    H: EncodeErrorHandler,
                {
					let mut buffer = output.start_nested_encode();
					$(
                        self.$n.dep_encode_or_handle_err(&mut buffer, h)?;
                    )+
					output.finalize_nested_encode(buffer);
					Ok(())
				}
            }

            impl<$($name),+> TopDecode for ($($name,)+)
            where
                $($name: NestedDecode,)+
            {
                fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
                where
                    I: TopDecodeInput,
                    H: DecodeErrorHandler,
                {
                    top_decode_from_nested_or_handle_err(input, h)
                }
            }

            impl<$($name),+> NestedEncode for ($($name,)+)
            where
                $($name: NestedEncode,)+
            {
				fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
                where
                    O: NestedEncodeOutput,
                    H: EncodeErrorHandler,
                {
					$(
                        self.$n.dep_encode_or_handle_err(dest, h)?;
                    )+
					Ok(())
				}
            }

            impl<$($name),+> NestedDecode for ($($name,)+)
            where
                $($name: NestedDecode,)+
            {
                fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
                where
                    I: NestedDecodeInput,
                    H: DecodeErrorHandler,
                {
                    Ok((
                        $(
                            $name::dep_decode_or_handle_err(input, h)?,
                        )+
                    ))
                }
            }
        )+
    }
}

tuple_impls! {
    (0 T0)
    (0 T0 1 T1)
    (0 T0 1 T1 2 T2)
    (0 T0 1 T1 2 T2 3 T3)
    (0 T0 1 T1 2 T2 3 T3 4 T4)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

#[cfg(test)]
mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};

    #[test]
    fn test_top() {
        let t = (1i8, 2u32, 3i16);
        let expected: &[u8] = &[1, 0, 0, 0, 2, 0, 3];
        check_top_encode_decode(t, expected);
    }
    #[test]
    fn test_dep() {
        let t = (1i8, 2u32, 3i16);
        let expected: &[u8] = &[1, 0, 0, 0, 2, 0, 3];
        check_dep_encode_decode(t, expected);
    }
}
