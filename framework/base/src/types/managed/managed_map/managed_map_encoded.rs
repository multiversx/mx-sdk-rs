use multiversx_sc_codec::multi_types::MultiValue2;
use multiversx_sc_codec::{
    multi_encode_iter_or_handle_err, TopDecodeMulti, TopEncode, TopEncodeMulti,
};
use unwrap_infallible::UnwrapInfallible;

use crate::api::{ErrorApi, ManagedMapApiImpl, ManagedTypeApi};
use crate::contract_base::ExitCodecErrorHandler;
use crate::err_msg;
use crate::types::{ManagedBuffer, ManagedMap, ManagedType};
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

// impl<M, K, V> ManagedMapEncoded<M, K, V>
// where
//     M: ManagedTypeApi + ErrorApi,
//     K: TopEncodeMulti,
//     V: TopEncodeMulti,
// {
//     pub fn put(&mut self, key: K, value: V) {
//         let iter = self.iter().map(MultiValue2::<K, V>::from);
//         multi_encode_iter_or_handle_err(iter, output, h)
//         key.multi_encode_or_handle_err(
//             &mut self.raw_map,
//             ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
//         ).unwrap_infallible();
//         value.multi_encode_or_handle_err(
//             &mut self.raw_map,
//             ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
//         ).unwrap_infallible()
//     }
// }
