use crate::abi::TypeAbi;
use crate::io::{ArgId, DynArg, DynArgInput};
use alloc::string::String;
use elrond_codec::TopDecodeInput;

macro_rules! multi_arg_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<I, D, $($name),+ > DynArg<I, D> for $mr<$($name,)+>
            where
                I: TopDecodeInput,
                D: DynArgInput<I>,
                $($name: DynArg<I, D>,)+
            {
                fn dyn_load(loader: &mut D, arg_id: ArgId) -> Self {
                    $mr((
                        $(
                            $name::dyn_load(loader, arg_id)
                        ),+
                    ))
                }
            }

            impl<$($name),+ > TypeAbi for $mr<$($name,)+>
            where
                $($name: TypeAbi,)+
            {
                fn type_name() -> String {
                    let mut repr = String::from("MultiArg<");
                    $(
                        repr.push_str($name::type_name().as_str());
                        repr.push(',');
                    )+
                    repr.push('>');
                    repr
                }
            }

            impl<$($name,)+> $mr<$($name,)+> {
                #[inline]
                pub fn into_tuple(self) -> ($($name,)+) {
                    self.0
                }
            }
        )+
    }
}

multi_arg_impls! {
	(MultiArg2  0 T0 1 T1)
	(MultiArg3  0 T0 1 T1 2 T2)
	(MultiArg4  0 T0 1 T1 2 T2 3 T3)
	(MultiArg5  0 T0 1 T1 2 T2 3 T3 4 T4)
	(MultiArg6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
	(MultiArg7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
	(MultiArg8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
	(MultiArg9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
	(MultiArg10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
	(MultiArg11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
	(MultiArg12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
	(MultiArg13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
	(MultiArg14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
	(MultiArg15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
	(MultiArg16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
