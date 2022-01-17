use core::marker::PhantomData;

use crate::{api::ManagedTypeApi, types::ManagedSCError};

#[derive(Default)]
pub struct ErrorHelper<M: ManagedTypeApi> {
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ErrorHelper<M> {
    pub(crate) fn new_instance() -> Self {
        ErrorHelper {
            _phantom: PhantomData,
        }
    }

    pub fn new_error(&self) -> ManagedSCError<M> {
        ManagedSCError::new_empty()
    }
}
