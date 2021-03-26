use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::io::{ArgId, ContractCallArg, DynArg, DynArgInput};
use crate::types::{ArgBuffer, SCError};
use crate::{api::EndpointFinishApi, EndpointResult};
use alloc::string::String;
use alloc::vec::Vec;
use elrond_codec::TopDecodeInput;

macro_rules! multi_arg_impls {
    ($(($marg_struct:ident $mres_struct:ident $abi_name:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $marg_struct<$($name,)+>(pub ($($name,)+));

            pub type $mres_struct<$($name,)+> = $marg_struct<$($name,)+>;

            impl<$($name),+ > DynArg for $marg_struct<$($name,)+>
            where
                $($name: DynArg,)+
            {
                fn dyn_load<I, D>(loader: &mut D, arg_id: ArgId) -> Self
                where
                    I: TopDecodeInput,
                    D: DynArgInput<I>,
                {
                    $marg_struct((
                        $(
                            $name::dyn_load(loader, arg_id)
                        ),+
                    ))
                }
            }

            impl<FA, $($name),+> EndpointResult<FA> for $marg_struct<$($name,)+>
            where
                FA: EndpointFinishApi + Clone + 'static,
                $($name: EndpointResult<FA>,)+
            {
                #[inline]
				fn finish(&self, api: FA) {
                    $(
                        (self.0).$n.finish(api.clone());
                    )+
                }
            }

            impl<$($name),+> ContractCallArg for &$marg_struct<$($name,)+>
            where
                $($name: ContractCallArg,)+
            {
                #[inline]
                fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
                    $(
                        (self.0).$n.push_async_arg(serializer)?;
                    )+
                    Ok(())
                }
            }

            impl<$($name),+> ContractCallArg for $marg_struct<$($name,)+>
            where
                $($name: ContractCallArg,)+
            {
                fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
                    (&self).push_async_arg(serializer)
                }
            }

            impl<$($name),+ > TypeAbi for $marg_struct<$($name,)+>
            where
                $($name: TypeAbi,)+
            {
                fn type_name() -> String {
                    let mut repr = String::from(stringify!($abi_name));
                    repr.push('<');
                    $(
                        repr.push_str($name::type_name().as_str());
                        repr.push(',');
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
	(MultiArg2  MultiResult2  multi2  0 T0 1 T1)
	(MultiArg3  MultiResult3  multi3  0 T0 1 T1 2 T2)
	(MultiArg4  MultiResult4  multi4  0 T0 1 T1 2 T2 3 T3)
	(MultiArg5  MultiResult5  multi5  0 T0 1 T1 2 T2 3 T3 4 T4)
	(MultiArg6  MultiResult6  multi6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
	(MultiArg7  MultiResult7  multi7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
	(MultiArg8  MultiResult8  multi8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
	(MultiArg9  MultiResult9  multi9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
	(MultiArg10 MultiResult10 multi10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
	(MultiArg11 MultiResult11 multi11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
	(MultiArg12 MultiResult12 multi12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
	(MultiArg13 MultiResult13 multi13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
	(MultiArg14 MultiResult14 multi14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
	(MultiArg15 MultiResult15 multi15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
	(MultiArg16 MultiResult16 multi16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
