use crate::api::ManagedTypeApi;

pub trait ManagedFrom<M, F>
where
    M: ManagedTypeApi,
{
    fn managed_from(api: M, from: F) -> Self;
}

impl<M, F> ManagedFrom<M, F> for F
where
    M: ManagedTypeApi,
{
    fn managed_from(_: M, t: F) -> Self {
        t
    }
}

pub trait ManagedInto<M, T>
where
    M: ManagedTypeApi,
{
    fn managed_into(self, api: M) -> T;
}

impl<M, F, T> ManagedInto<M, T> for F
where
    T: ManagedFrom<M, F>,
    M: ManagedTypeApi,
{
    fn managed_into(self, api: M) -> T {
        T::managed_from(api, self)
    }
}
