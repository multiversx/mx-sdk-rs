use crate::abi::{TypeAbi, TypeDescriptionContainer};
use crate::io::EndpointResult;
use alloc::string::String;

macro_rules! choice_impls {
    ($(($ch:ident $($variant:tt $name:ident)+) )+) => {
        $(
            pub enum $ch<$($name,)+> {
				$(
					$variant($name),
				)+
			}

            impl<FA, $($name),+> EndpointResult<FA> for $ch<$($name,)+>
            where
                FA: 'static,
                $($name: EndpointResult<FA>,)+
            {
                #[inline]
				fn finish(&self, api: FA) {
					match self {
						$(
							$ch::$variant(t) => t.finish(api),
						)+
					}

                }
            }

            impl<$($name),+ > TypeAbi for $ch<$($name,)+>
            where
                $($name: TypeAbi,)+
            {
                fn type_name() -> String {
                    let mut repr = String::from("Choice<");
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
            }
        )+
    }
}

#[rustfmt::skip]
choice_impls! {
	(Choice1  Variant0 T0)
	(Choice2  Variant0 T0 Variant1 T1)
	(Choice3  Variant0 T0 Variant1 T1 Variant2 T2)
	(Choice4  Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3)
	(Choice5  Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4)
	(Choice6  Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5)
	(Choice7  Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6)
	(Choice8  Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7)
	(Choice9  Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8)
	(Choice10 Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8 Variant9 T9)
	(Choice11 Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8 Variant9 T9 Variant10 T10)
	(Choice12 Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8 Variant9 T9 Variant10 T10 Variant11 T11)
	(Choice13 Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8 Variant9 T9 Variant10 T10 Variant11 T11 Variant12 T12)
	(Choice14 Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8 Variant9 T9 Variant10 T10 Variant11 T11 Variant12 T12 Variant13 T13)
	(Choice15 Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8 Variant9 T9 Variant10 T10 Variant11 T11 Variant12 T12 Variant13 T13 Variant14 T14)
	(Choice16 Variant0 T0 Variant1 T1 Variant2 T2 Variant3 T3 Variant4 T4 Variant5 T5 Variant6 T6 Variant7 T7 Variant8 T8 Variant9 T9 Variant10 T10 Variant11 T11 Variant12 T12 Variant13 T13 Variant14 T14 Variant15 T15)
}
