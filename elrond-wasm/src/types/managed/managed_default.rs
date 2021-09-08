use crate::api::ManagedTypeApi;

pub trait ManagedDefault<M>
where
    M: ManagedTypeApi,
{
    fn managed_default(api: M) -> Self;
}
