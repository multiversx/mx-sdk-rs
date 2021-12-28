use crate::api::ErrorApi;

use super::ManagedTypeApiImpl;

pub trait ManagedTypeApi: ErrorApi + Clone + 'static {
    type Impl: ManagedTypeApiImpl;

    fn managed_type_impl() -> Self::Impl;
}
