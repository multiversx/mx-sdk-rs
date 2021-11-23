use crate::{api::ManagedTypeApi, types::ManagedSCError};

pub struct ErrorHelper<M: ManagedTypeApi> {
    _api: M,
}

impl<M: ManagedTypeApi> ErrorHelper<M> {
    pub(crate) fn new_instance(_api: M) -> Self {
        ErrorHelper { _api }
    }

    pub fn new_error(&self) -> ManagedSCError<M> {
        ManagedSCError::new_empty()
    }
}
