use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use alloc::{string::String, vec::Vec};

macro_rules! multi_arg_impls {
    ($(($mval_struct:ident $marg_struct:ident $mres_struct:ident $($n:tt $name:ident)+) )+) => {
        $(
            #[deprecated(
                since = "0.28.0",
                note = "Alias kept for backwards compatibility. Replace with `MultiValue*`"
            )]
            pub type $marg_struct<$($name,)+> = elrond_codec::multi_types::$mval_struct<$($name,)+>;

            #[deprecated(
                since = "0.28.0",
                note = "Alias kept for backwards compatibility. Replace with `MultiValue*`"
            )]
            pub type $mres_struct<$($name,)+> = elrond_codec::multi_types::$mval_struct<$($name,)+>;

            impl<$($name),+ > TypeAbi for elrond_codec::multi_types::$mval_struct<$($name,)+>
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
        )+
    }
}

multi_arg_impls! {
    (MultiValue2  MultiArg2  MultiResult2  0 T0 1 T1)
    (MultiValue3  MultiArg3  MultiResult3  0 T0 1 T1 2 T2)
    (MultiValue4  MultiArg4  MultiResult4  0 T0 1 T1 2 T2 3 T3)
    (MultiValue5  MultiArg5  MultiResult5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiValue6  MultiArg6  MultiResult6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiValue7  MultiArg7  MultiResult7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiValue8  MultiArg8  MultiResult8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiValue9  MultiArg9  MultiResult9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiValue10 MultiArg10 MultiResult10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiValue11 MultiArg11 MultiResult11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiValue12 MultiArg12 MultiResult12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiValue13 MultiArg13 MultiResult13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiValue14 MultiArg14 MultiResult14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiValue15 MultiArg15 MultiResult15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiValue16 MultiArg16 MultiResult16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
