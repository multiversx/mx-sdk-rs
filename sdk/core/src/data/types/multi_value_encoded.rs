use multiversx_sc::api::{ManagedTypeApi};
use multiversx_sc::types::{ManagedVecItem, MultiValueEncoded};
use multiversx_sc_codec::TopDecode;
use crate::data::types::native::NativeConvertible;

impl<M: ManagedTypeApi, T: NativeConvertible + ManagedVecItem + TopDecode> NativeConvertible for MultiValueEncoded<M, T> {
    type Native = Vec<T::Native>;

    fn to_native(&self) -> Self::Native {
        self.to_vec().to_native()
    }
}