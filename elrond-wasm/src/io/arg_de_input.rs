use crate::api::{EndpointArgumentApi, ManagedTypeApi};
use crate::managed_codec::ManagedTopDecodeInput;
use crate::types::ManagedBuffer;
// use crate::Box;
// use elrond_codec::TopDecodeInput;

/// Adapter from the API to the TopDecodeInput trait.
/// Allows objects to be deserialized directly from the API as arguments.
///
/// Of course the implementation provides shortcut deserialization computation paths directly from API:
/// into_u64, into_i64, ...
///
/// This is a performance-critical struct.
/// Since the wasm EndpointArgumentApi (ArwenApiImpl) is zero-size,
/// it means that this structures translates to a single glorified i32 in wasm.
pub struct ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi + 'static,
{
    api: AA,
    arg_index: i32,
}

impl<AA> ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi + 'static,
{
    #[inline]
    pub fn new(api: AA, arg_index: i32) -> Self {
        ArgDecodeInput { api, arg_index }
    }
}

impl<AA> ManagedTopDecodeInput<AA> for ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi + 'static,
{
    fn get_managed_buffer(&self) -> ManagedBuffer<AA> {
        ManagedBuffer::new_from_raw_handle(
            self.api.clone(),
            self.api.get_argument_managed_buffer_raw(self.arg_index),
        )
    }

    // fn byte_len(&self) -> usize {
    //     self.api.get_argument_len(self.arg_index)
    // }

    // fn into_boxed_slice_u8(self) -> Box<[u8]> {
    //     self.api.get_argument_boxed_bytes(self.arg_index).into_box()
    // }

    // fn into_u64(self) -> u64 {
    //     self.api.get_argument_u64(self.arg_index)
    // }

    // fn into_i64(self) -> i64 {
    //     self.api.get_argument_i64(self.arg_index)
    // }

    // #[inline]
    // fn try_get_big_uint_handle(&self) -> (bool, i32) {
    //     (true, self.api.get_argument_big_uint_raw(self.arg_index))
    // }

    // #[inline]
    // fn try_get_big_int_handle(&self) -> (bool, i32) {
    //     (true, self.api.get_argument_big_int_raw(self.arg_index))
    // }
}
