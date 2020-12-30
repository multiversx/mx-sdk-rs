use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::{BigIntApi, BigUintApi, ContractHookApi, ContractIOApi, EndpointResult};
use alloc::string::String;
use alloc::vec::Vec;

macro_rules! multi_result_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<A, BigInt, BigUint, $($name),+> EndpointResult<A, BigInt, BigUint> for $mr<$($name,)+>
            where
                $($name: EndpointResult<A, BigInt, BigUint>,)+
                BigInt: BigIntApi<BigUint> + 'static,
                BigUint: BigUintApi + 'static,
                A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static
            {
                #[inline]
				fn finish(&self, api: A) {
                    $(
                        (self.0).$n.finish(api.clone());
                    )+
                }
            }

            impl<$($name),+ > TypeAbi for $mr<$($name,)+>
            where
                $($name: TypeAbi,)+
            {
                fn type_name() -> String {
                    let mut repr = String::from("MultiResult<");
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
