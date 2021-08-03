use super::BigIntApi;

pub type Handle = i32;

pub trait ManagedTypeApi: BigIntApi + Clone {}
