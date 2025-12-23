use crate::{
    api::{ManagedTypeApi, quick_signal_error},
    types::ManagedBuffer,
};

pub fn to_buffered_json<M, T, const N: usize>(value: &T) -> ManagedBuffer<M>
where
    M: ManagedTypeApi,
    T: serde::Serialize,
{
    let mut buf = [0u8; N];
    let used = serde_json_core::to_slice(value, &mut buf)
        .unwrap_or_else(|_| quick_signal_error::<M>("serialization failed"));

    ManagedBuffer::new_from_bytes(&buf[..used])
}

// pub fn with_deserialized_buffered_json<'a, M, T, F, R, const N: usize>(
//     json: &'a ManagedBuffer<M>,
//     f: F,
// ) -> R
// where
//     M: ManagedTypeApi,
//     T: serde::Deserialize<'a>,
//     R: 'static,
//     F: FnOnce(&T) -> R,
// {
//     let mut buf = [0u8; N];
//     let slice = json.load_to_byte_array(&mut buf);
//     let (value, _) = serde_json_core::from_slice(slice)
//         .unwrap_or_else(|_| quick_signal_error::<M>("deserialization failed"));

//     f(&value)
// }
