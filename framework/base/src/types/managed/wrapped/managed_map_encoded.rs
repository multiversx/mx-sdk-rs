use multiversx_sc_codec::{TopDecode, TopEncode};
use unwrap_infallible::UnwrapInfallible;

use crate::api::{const_handles, ErrorApi, ManagedTypeApi};
use crate::contract_base::ExitCodecErrorHandler;
use crate::err_msg;
use crate::types::{ManagedBuffer, ManagedMap, ManagedRefMut};
use core::marker::PhantomData;

/// A managed map that works with any serializable key and value types.
///
/// It encodes both when saving, and decodes the value when getting.
///
/// ## Empty encodings
///
/// Just like the base ManagedMap, it makes no difference between a missing key
/// and a key with a corresponding empty encoded value.
///
/// So, for instance, here `contains` returns false, because `0` is encoded as an empty buffer:
///
/// ```no_run
/// # let mut mme = multiversx_sc::types::ManagedMapEncoded::<multiversx_sc::api::uncallable::UncallableApi, i32, i32>::new();
/// # let key = 1;
/// mme.put(&key, &0);
/// assert!(!mme.contains(&key));
/// ```
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
    /// Retrieves and decodes value associated with key.
    pub fn get(&self, key: &K) -> V {
        let temp_key = temp_key_encode(key);
        let value_raw = self.raw_map.get(&temp_key);
        value_decode(value_raw)
    }

    /// Since both the key and value are encoded before saving to the map, there is no need to take them owned.
    pub fn put(&mut self, key: &K, value: &V) {
        let temp_key = temp_key_encode(key);
        let temp_value = temp_value_encode(value);
        self.raw_map.put(&temp_key, &temp_value);
    }

    /// Clears value associated with key, and returns old value.
    pub fn remove(&mut self, key: &K) -> V {
        let temp_key = temp_key_encode(key);
        let value_raw = self.raw_map.remove(&temp_key);
        value_decode(value_raw)
    }

    /// Returns true if there is a non-empty encoded value associated with the key.
    pub fn contains(&self, key: &K) -> bool {
        let temp_key = temp_key_encode(key);
        self.raw_map.contains(&temp_key)
    }
}

fn temp_key_encode<M, K>(key: &K) -> ManagedRefMut<'static, M, ManagedBuffer<M>>
where
    M: ManagedTypeApi + ErrorApi,
    K: TopEncode,
{
    let mut key_ref = unsafe { ManagedBuffer::temp_const_ref_mut(const_handles::MBUF_TEMPORARY_1) };
    key_ref.overwrite(&[]);
    key.top_encode_or_handle_err(
        &mut *key_ref,
        ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
    )
    .unwrap_infallible();
    key_ref
}

fn temp_value_encode<M, V>(value: &V) -> ManagedRefMut<'static, M, ManagedBuffer<M>>
where
    M: ManagedTypeApi + ErrorApi,
    V: TopEncode,
{
    let mut value_ref =
        unsafe { ManagedBuffer::temp_const_ref_mut(const_handles::MBUF_TEMPORARY_2) };
    value_ref.overwrite(&[]);
    value
        .top_encode_or_handle_err(
            &mut *value_ref,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        )
        .unwrap_infallible();
    value_ref
}

fn value_decode<M, V>(value_raw: ManagedBuffer<M>) -> V
where
    M: ManagedTypeApi + ErrorApi,
    V: TopDecode,
{
    V::top_decode_or_handle_err(
        value_raw,
        ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
    )
    .unwrap_infallible()
}
