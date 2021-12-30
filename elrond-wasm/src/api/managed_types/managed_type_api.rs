use crate::api::ErrorApi;

use super::{ManagedTypeApiImpl, StaticBufferApi};

pub trait ManagedTypeApi: StaticBufferApi + ErrorApi + Clone + 'static {
    type ManagedTypeApiImpl: ManagedTypeApiImpl;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl;
}
