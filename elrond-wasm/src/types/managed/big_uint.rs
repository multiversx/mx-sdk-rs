use crate::api::{Handle, ManagedTypeApi};

pub struct BigUint<M: ManagedTypeApi> {
    pub handle: Handle,
    pub api: M,
}
