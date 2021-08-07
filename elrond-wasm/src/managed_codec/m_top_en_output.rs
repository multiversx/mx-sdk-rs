use core::marker::PhantomData;

use elrond_codec::TopEncodeOutput;

use crate::{api::ManagedTypeApi, types::ManagedBuffer};

pub trait ManagedTopEncodeOutput<M: ManagedTypeApi>: Sized {
    fn get_api(&self) -> M;

    fn set_managed_buffer(&self, value: &ManagedBuffer<M>);

    fn set_unit(&self) {
        self.set_managed_buffer(&ManagedBuffer::new_empty(self.get_api()));
    }
}

pub struct TopEncodeOutputAdapter<M, MO>
where
    M: ManagedTypeApi,
    MO: ManagedTopEncodeOutput<M>,
{
    wrapped: MO,
    _phantom: PhantomData<M>,
}

impl<M, MO> TopEncodeOutputAdapter<M, MO>
where
    M: ManagedTypeApi,
    MO: ManagedTopEncodeOutput<M>,
{
    pub fn new(output: MO) -> Self {
        TopEncodeOutputAdapter {
            wrapped: output,
            _phantom: PhantomData,
        }
    }
}

impl<M, MO> TopEncodeOutput for TopEncodeOutputAdapter<M, MO>
where
    M: ManagedTypeApi,
    MO: ManagedTopEncodeOutput<M>,
{
    fn set_slice_u8(self, bytes: &[u8]) {
        self.wrapped
            .set_managed_buffer(&ManagedBuffer::new_from_bytes(
                self.wrapped.get_api(),
                bytes,
            ));
    }
}
