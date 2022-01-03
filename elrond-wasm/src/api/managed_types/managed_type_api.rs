use crate::api::ErrorApi;

use super::{ManagedTypeApiImpl, StaticVarApi};

pub trait ManagedTypeApi: StaticVarApi + ErrorApi + Clone + 'static {
    type ManagedTypeApiImpl: ManagedTypeApiImpl;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl;
}
