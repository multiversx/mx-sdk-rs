use core::marker::PhantomData;

use alloc::boxed::Box;
use elrond_codec::TopDecodeInput;

use crate::{api::ManagedTypeApi, types::ManagedBuffer};

pub trait ManagedTopDecodeInput<M: ManagedTypeApi>: Sized {
    fn get_managed_buffer(&self) -> ManagedBuffer<M>;

    // fn decode_error_exit(&self, err: DecodeError) -> !;
}

// impl<M: ManagedTypeApi> ManagedTopDecodeInput<M> for ManagedBuffer<M> {
//     fn get_managed_buffer(&self) -> ManagedBuffer<M> {
//         self.clone()
//     }
// }


/// Takes a ManagedTopDecodeInput and provides a TopDecodeInput implementation.
pub struct TopDecodeInputAdapter<M, MI>
where
    M: ManagedTypeApi,
    MI: ManagedTopDecodeInput<M>,
{
    wrapped: MI,
    _phantom: PhantomData<M>,
}

impl<M, MI> TopDecodeInputAdapter<M, MI>
where
    M: ManagedTypeApi,
    MI: ManagedTopDecodeInput<M>,
{
    pub fn new(input: MI) -> Self {
        TopDecodeInputAdapter {
            wrapped: input,
            _phantom: PhantomData,
        }
    }
}

impl<M, MI> TopDecodeInput for TopDecodeInputAdapter<M, MI>
where
    M: ManagedTypeApi,
    MI: ManagedTopDecodeInput<M>,
{
    fn byte_len(&self) -> usize {
        self.wrapped.get_managed_buffer().len()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.wrapped
            .get_managed_buffer()
            .to_boxed_bytes()
            .into_box()
    }
}

// impl TopDecodeInput for Box<[u8]> {
//     fn byte_len(&self) -> usize {
//         self.len()
//     }

//     fn into_boxed_slice_u8(self) -> Box<[u8]> {
//         self
//     }
// }

// impl TopDecodeInput for Vec<u8> {
//     fn byte_len(&self) -> usize {
//         self.len()
//     }

//     fn into_boxed_slice_u8(self) -> Box<[u8]> {
//         vec_into_boxed_slice(self)
//     }
// }


