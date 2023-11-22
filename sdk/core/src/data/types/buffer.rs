use std::str;
use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::types::{ManagedBuffer, TokenIdentifier};
use crate::data::types::native::NativeConvertible;

impl<M: ManagedTypeApi> NativeConvertible for ManagedBuffer<M> {
    type Native = String;

    fn to_native(&self) -> Self::Native {
        let bytes = self.to_boxed_bytes();
        let result = str::from_utf8(bytes.as_slice());

        if let Err(_) = result {
            panic!("Cannot parse the given ManagedBuffer to an utf8 String");
        }

        String::from(result.unwrap())
    }
}

impl<M: ManagedTypeApi> NativeConvertible for TokenIdentifier<M> {
    type Native = String;

    fn to_native(&self) -> Self::Native {
        self.as_managed_buffer().to_native()
    }
}