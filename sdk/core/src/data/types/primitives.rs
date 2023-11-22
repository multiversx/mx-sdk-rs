use crate::data::types::native::NativeConvertible;

macro_rules! native_convertible_impl_primitive {
    ($($type_name:ident )+) => {
        $(
            impl NativeConvertible for $type_name {
                type Native = Self;

                fn to_native(&self) -> Self::Native {
                    *self
                }
            }
        )+
    }
}

native_convertible_impl_primitive! {
    i8 i16 i32 i64 u8 u16 u32 u64 bool
}