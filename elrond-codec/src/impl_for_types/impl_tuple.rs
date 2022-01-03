use crate::{
    codec_err::{DecodeError, EncodeError},
    nested_de::NestedDecode,
    nested_de_input::NestedDecodeInput,
    nested_ser::NestedEncode,
    nested_ser_output::NestedEncodeOutput,
    top_de::{top_decode_from_nested, top_decode_from_nested_or_exit, TopDecode},
    top_de_input::TopDecodeInput,
    top_ser::TopEncode,
    top_ser_output::TopEncodeOutput,
};

macro_rules! tuple_impls {
    ($(($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TopEncode for ($($name,)+)
            where
                $($name: NestedEncode,)+
            {
				fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
					let mut buffer = output.start_nested_encode();
					$(
                        self.$n.dep_encode(&mut buffer)?;
                    )+
					output.finalize_nested_encode(buffer);
					Ok(())
				}

				fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
					let mut buffer = output.start_nested_encode();
					$(
                        self.$n.dep_encode_or_exit(&mut buffer, c.clone(), exit);
                    )+
					output.finalize_nested_encode(buffer);
				}
            }
            impl<$($name),+> TopDecode for ($($name,)+)
            where
                $($name: NestedDecode,)+
            {
                fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                    top_decode_from_nested(input)
                }

                fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(input: I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
                    top_decode_from_nested_or_exit(input, c, exit)
                }
            }
            impl<$($name),+> NestedEncode for ($($name,)+)
            where
                $($name: NestedEncode,)+
            {
				fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
					$(
                        self.$n.dep_encode(dest)?;
                    )+
					Ok(())
				}

				fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(&self, dest: &mut O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
					$(
                        self.$n.dep_encode_or_exit(dest, c.clone(), exit);
                    )+
				}
            }
            impl<$($name),+> NestedDecode for ($($name,)+)
            where
                $($name: NestedDecode,)+
            {
                fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                    Ok((
                        $(
                            $name::dep_decode(input)?,
                        )+
                    ))
                }

                fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(input: &mut I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
                    (
                        $(
                            $name::dep_decode_or_exit(input, c.clone(), exit),
                        )+
                    )
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
