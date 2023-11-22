use crate::data::types::native::NativeConvertible;

macro_rules! multi_value_native_convertible_impl {
        ($(($mv_struct:ident $len:tt $($n:tt $name:ident)+) )+) => {
        $(
            use multiversx_sc_codec::multi_types::$mv_struct;
            impl<$($name: NativeConvertible,)+> NativeConvertible for $mv_struct<$($name,)+> {
                type Native = ($($name::Native,)+);

                fn to_native(&self) -> Self::Native {
                    ($((self.0).$n.to_native()),+)
                }
            }
        )+
    }
}

multi_value_native_convertible_impl! {
    (MultiValue2   2 0 T0 1 T1)
    (MultiValue3   3 0 T0 1 T1 2 T2)
    (MultiValue4   4 0 T0 1 T1 2 T2 3 T3)
    (MultiValue5   5 0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiValue6   6 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiValue7   7 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiValue8   8 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiValue9   9 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiValue10 10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiValue11 11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
}