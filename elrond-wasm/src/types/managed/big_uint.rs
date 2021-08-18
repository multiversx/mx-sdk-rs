use crate::api::ManagedTypeApi;

pub struct BigUint<M: ManagedTypeApi> {
    pub handle: i32,
    pub api: M,
}
