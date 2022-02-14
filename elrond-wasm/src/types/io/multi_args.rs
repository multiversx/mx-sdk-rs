use crate::{
    abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
    elrond_codec::{
        DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
        TopEncodeMulti, TopEncodeMultiOutput,
    },
};
use alloc::{string::String, vec::Vec};

macro_rules! multi_arg_impls {
    ($(($marg_struct:ident $mres_struct:ident $($n:tt $name:ident)+) )+) => {
        $(
            #[derive(Clone)]
            pub struct $marg_struct<$($name,)+>(pub ($($name,)+));

            pub type $mres_struct<$($name,)+> = $marg_struct<$($name,)+>;

            impl<$($name),+ > TopEncodeMulti for $marg_struct<$($name,)+>
            where
                $($name: TopEncodeMulti,)+
            {
                type DecodeAs = Self; // TODO: reassemble from component DecodeAs

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

            impl<$($name),+ > TopDecodeMulti for $marg_struct<$($name,)+>
            where
                $($name: TopDecodeMulti,)+
            {
                fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
                where
                    I: TopDecodeMultiInput,
                    H: DecodeErrorHandler,
                {
                    Ok(Self((
                        $(
                            $name::multi_decode_or_handle_err(input, h)?
                        ),+
                    )))
                }
            }

            impl<$($name),+ > TypeAbi for $marg_struct<$($name,)+>
            where
                $($name: TypeAbi,)+
            {
                fn type_name() -> String {
                    let mut repr = String::from("multi");
                    repr.push('<');
                    $(
                        if $n > 0 {
                            repr.push(',');
                        }
                        repr.push_str($name::type_name().as_str());
                    )+
                    repr.push('>');
                    repr
                }

                fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
					$(
						$name::provide_type_descriptions(accumulator);
                    )+
                }

                fn is_multi_arg_or_result() -> bool {
                    true
                }

                fn output_abis(output_names: &[&'static str]) -> Vec<OutputAbi> {
                    let mut result = Vec::new();
                    $(
                        if output_names.len() > $n {
                            result.append(&mut $name::output_abis(&[output_names[$n]]));

                        } else {
                            result.append(&mut $name::output_abis(&[]));
                        }

                    )+
                    result
                }
            }

            impl<$($name),+> From<($($name,)+)> for $marg_struct<$($name,)+> {
                #[inline]
                fn from(tuple: ($($name,)+)) -> Self {
                    $marg_struct(tuple)
                }
            }

            impl<$($name,)+> $marg_struct<$($name,)+> {
                #[inline]
                pub fn into_tuple(self) -> ($($name,)+) {
                    self.0
                }
            }
        )+
    }
}

multi_arg_impls! {
    (MultiArg2  MultiResult2  0 T0 1 T1)
    (MultiArg3  MultiResult3  0 T0 1 T1 2 T2)
    (MultiArg4  MultiResult4  0 T0 1 T1 2 T2 3 T3)
    (MultiArg5  MultiResult5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiArg6  MultiResult6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiArg7  MultiResult7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiArg8  MultiResult8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiArg9  MultiResult9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiArg10 MultiResult10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiArg11 MultiResult11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiArg12 MultiResult12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiArg13 MultiResult13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiArg14 MultiResult14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiArg15 MultiResult15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiArg16 MultiResult16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
