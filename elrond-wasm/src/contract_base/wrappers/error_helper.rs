use crate::{api::ManagedTypeApi, types::ManagedSCError};

pub struct ErrorHelper<M: ManagedTypeApi> {
    api: M,
}

impl<M: ManagedTypeApi> ErrorHelper<M> {
    pub(crate) fn new_instance(api: M) -> Self {
        ErrorHelper { api }
    }

    pub fn new_error(&self) -> ManagedSCError<M> {
        ManagedSCError::new_empty(self.api.clone())
    }
}
