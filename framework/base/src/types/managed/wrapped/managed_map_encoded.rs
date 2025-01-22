use multiversx_sc_codec::{TopDecode, TopEncode};
use unwrap_infallible::UnwrapInfallible;

use crate::api::{const_handles, use_raw_handle, ErrorApi, ManagedTypeApi};
use crate::contract_base::ExitCodecErrorHandler;
use crate::err_msg;
use crate::types::{ManagedBuffer, ManagedMap, ManagedRefMut};
use core::marker::PhantomData;

#[derive(Default)]
pub struct ManagedMapEncoded<M, K, V>
where
    M: ManagedTypeApi,
{
    pub(super) raw_map: ManagedMap<M>,
    _phantom: PhantomData<(K, V)>,
}

impl<M, K, V> ManagedMapEncoded<M, K, V>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from_raw_map(raw_map: ManagedMap<M>) -> Self {
        ManagedMapEncoded {
            raw_map,
            _phantom: PhantomData,
        }
    }

    pub fn new() -> Self {
        ManagedMapEncoded::from_raw_map(ManagedMap::new())
    }
}

impl<M, K, V> ManagedMapEncoded<M, K, V>
where
    M: ManagedTypeApi + ErrorApi,
    K: TopEncode,
    V: TopEncode + TopDecode,
{
    fn temp_key_encode(key: K) -> ManagedRefMut<'static, M, ManagedBuffer<M>> {
        let mut key_ref =
            unsafe { ManagedRefMut::wrap_handle(use_raw_handle(const_handles::MBUF_TEMPORARY_1)) };
        key.top_encode_or_handle_err(
            &mut *key_ref,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        )
        .unwrap_infallible();
        key_ref
    }

    fn temp_value_encode(value: V) -> ManagedRefMut<'static, M, ManagedBuffer<M>> {
        let mut value_ref =
            unsafe { ManagedRefMut::wrap_handle(use_raw_handle(const_handles::MBUF_TEMPORARY_2)) };
        value
            .top_encode_or_handle_err(
                &mut *value_ref,
                ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
            )
            .unwrap_infallible();
        value_ref
    }

    fn value_decode(value_raw: ManagedBuffer<M>) -> V {
        V::top_decode_or_handle_err(
            value_raw,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        )
        .unwrap_infallible()
    }

    pub fn get(&mut self, key: K) -> V {
        let temp_key = Self::temp_key_encode(key);
        let value_raw = self.raw_map.get(&temp_key);
        Self::value_decode(value_raw)
    }

    pub fn put(&mut self, key: K, value: V) {
        let temp_key = Self::temp_key_encode(key);
        let temp_value = Self::temp_value_encode(value);
        self.raw_map.put(&temp_key, &temp_value);
    }

    pub fn remove(&mut self, key: K) -> V {
        let temp_key = Self::temp_key_encode(key);
        let value_raw = self.raw_map.remove(&temp_key);
        Self::value_decode(value_raw)
    }

    pub fn contains(&self, key: K) -> bool {
        let temp_key = Self::temp_key_encode(key);
        self.raw_map.contains(&temp_key)
    }
}
