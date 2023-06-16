use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::types::{ManagedVec, ManagedVecItem};
use crate::data::types::native::NativeConvertible;

impl<M: ManagedTypeApi, T: NativeConvertible + ManagedVecItem> NativeConvertible for ManagedVec<M, T> {
    type Native = Vec<T::Native>;

    fn to_native(&self) -> Self::Native {
        self.into_iter().map(|e| e.to_native()).collect()
    }
}