use crate::api::ManagedTypeApi;
use crate::types::ManagedBufferNestedDecodeInput;
use crate::Box;
use crate::{api::EndpointArgumentApi, types::ManagedBuffer};
use elrond_codec::{TopDecodeInput, TryStaticCast};

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
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    api: AA,
    arg_index: i32,
}

impl<AA> ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    #[inline]
    pub fn new(api: AA, arg_index: i32) -> Self {
        ArgDecodeInput { api, arg_index }
    }

    fn to_managed_buffer(&self) -> ManagedBuffer<AA> {
        let mbuf_handle = self.api.get_argument_managed_buffer_raw(self.arg_index);
        ManagedBuffer::new_from_raw_handle(self.api.clone(), mbuf_handle)
    }
}

impl<AA> TopDecodeInput for ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    type NestedBuffer = ManagedBufferNestedDecodeInput<AA>;

    fn byte_len(&self) -> usize {
        self.api.get_argument_len(self.arg_index)
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.api.get_argument_boxed_bytes(self.arg_index).into_box()
    }

    fn into_u64(self) -> u64 {
        self.api.get_argument_u64(self.arg_index)
    }

    fn into_i64(self) -> i64 {
        self.api.get_argument_i64(self.arg_index)
    }

    fn into_specialized<T: TryStaticCast>(self) -> Option<T> {
        if T::type_eq::<ManagedBuffer<AA>>() {
            self.to_managed_buffer().try_cast()
        } else {
            None
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBufferNestedDecodeInput::new(self.to_managed_buffer())
    }

    #[inline]
    fn try_get_big_uint_handle(&self) -> (bool, i32) {
        (true, self.api.get_argument_big_uint_raw(self.arg_index))
    }

    #[inline]
    fn try_get_big_int_handle(&self) -> (bool, i32) {
        (true, self.api.get_argument_big_int_raw(self.arg_index))
    }
}
