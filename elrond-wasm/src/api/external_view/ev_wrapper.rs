use core::marker::PhantomData;

use crate::api::VMApi;

pub struct ExternalViewApi<A: VMApi> {
    _phantom: PhantomData<A>,
}

impl<A: VMApi> ExternalViewApi<A> {
    pub(super) fn new() -> Self {
        ExternalViewApi {
            _phantom: PhantomData,
        }
    }
}
