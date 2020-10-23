use crate::io::{ArgId, ArgType, DynArgLoader};
use super::SCError;

macro_rules! multi_arg_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<$($name,)+ D> ArgType<D> for $mr<$($name,)+>
            where
                $($name: ArgType<D>,)+
                D: $(DynArgLoader<$name> + )+ Sized
            {
                fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
                    Ok($mr((
                        $(
                            $name::load(loader, arg_id)?
                        ),+
                    )))
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
