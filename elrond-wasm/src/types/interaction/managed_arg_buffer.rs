use elrond_codec::{EncodeError, TopEncode};

use crate::{
    api::{ErrorApi, Handle, ManagedTypeApi},
    err_msg,
    types::{ManagedBuffer, ManagedType, ManagedVec, ManagedVecIterator},
};

use super::ArgBuffer;

#[derive(Debug)]
pub struct ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    api: M,
    data: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> ManagedType<M> for ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    fn from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedArgBuffer {
            api: api.clone(),
            data: ManagedVec::from_raw_handle(api, handle),
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.data.get_raw_handle()
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.data.type_manager()
    }
}

impl<M: ManagedTypeApi> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    pub fn new_empty(api: M) -> Self {
        ManagedArgBuffer {
            api: api.clone(),
            data: ManagedVec::new_empty(api),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push_arg<T: TopEncode>(&mut self, arg: T) {
        let mut encoded_buffer = ManagedBuffer::new_empty(self.api.clone());
        arg.top_encode_or_exit(
            &mut encoded_buffer,
            self.api.clone(),
            managed_arg_buffer_push_exit,
        );
        self.push_arg_raw(encoded_buffer);
    }

    #[inline]
    pub fn push_arg_raw(&mut self, raw_arg: ManagedBuffer<M>) {
        self.data.push(raw_arg);
    }

    /// Concatenates 2 ArgBuffer. Consumes both arguments in the process.
    #[inline]
    pub fn concat(mut self, mut other: ManagedArgBuffer<M>) -> Self {
        self.data.append_vec(other.data);
        self
    }
}

#[inline(always)]
fn managed_arg_buffer_push_exit<A>(api: A, encode_err: EncodeError) -> !
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer =
        ManagedBuffer::new_from_bytes(api.clone(), err_msg::CONTRACT_CALL_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}

impl<M: ManagedTypeApi> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn to_legacy_arg_buffer(&self) -> ArgBuffer {
        let mut result = ArgBuffer::new();
        for m_arg in self.data.into_iter() {
            result.push_argument_bytes(m_arg.to_boxed_bytes().as_slice());
        }
        result
    }
}

impl<'a, M: ManagedTypeApi> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn raw_arg_iter(&'a self) -> ManagedVecIterator<'a, M, ManagedBuffer<M>> {
        ManagedVecIterator::new(&self.data)
    }
}
