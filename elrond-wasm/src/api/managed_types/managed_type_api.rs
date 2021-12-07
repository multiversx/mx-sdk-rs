use elrond_codec::TryStaticCast;

use crate::api::ErrorApi;

use super::ManagedTypeApiImpl;

pub trait ManagedTypeApi: TryStaticCast + ErrorApi + Clone + 'static {
    type Impl: ManagedTypeApiImpl;

    fn instance() -> Self::Impl;

    #[inline]
    fn error_api() -> Self::Impl {
        Self::instance()
    }
}
