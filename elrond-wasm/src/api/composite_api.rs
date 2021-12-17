use super::{ErrorApi, ErrorApiImpl, ManagedTypeApi, ManagedTypeApiImpl, SendApi};

pub trait ManagedTypeErrorApi: ManagedTypeApi + ErrorApi {
    type ManagedTypeErrorApiImpl: ManagedTypeApiImpl + ErrorApiImpl;

    fn managed_type_error_api() -> Self::ManagedTypeErrorApiImpl;
}

pub trait CallTypeApi: SendApi + ManagedTypeErrorApi {}
