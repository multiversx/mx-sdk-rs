use multiversx_sc_codec::multi_types::OptionalValue;
use crate::data::types::native::NativeConvertible;

impl<T: Clone + NativeConvertible> NativeConvertible for Option<T> {
    type Native = Option<T::Native>;

    fn to_native(&self) -> Self::Native {
        self.clone().map(|e| e.to_native())
    }
}

impl<T: Clone + NativeConvertible> NativeConvertible for OptionalValue<T> {
    type Native = Option<T::Native>;

    fn to_native(&self) -> Self::Native {
       self.clone().into_option().to_native()
    }
}