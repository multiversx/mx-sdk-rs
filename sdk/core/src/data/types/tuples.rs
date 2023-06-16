use crate::data::types::native::NativeConvertible;

macro_rules! tuple_native_convertible_impl {
    ($(($($num:tt $type_name:ident)+) )+) => {
        $(
            impl<$($type_name, )+> NativeConvertible for ($($type_name, )+)
            where
                $($type_name: NativeConvertible, )+
            {
                type Native = ($($type_name::Native, )+);

                fn to_native(&self) -> Self::Native {
                    ($(self.$num.to_native(), )+)
                }
            }

        )+
    }
}

tuple_native_convertible_impl! {
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
}