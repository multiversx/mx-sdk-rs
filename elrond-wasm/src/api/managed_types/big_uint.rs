use core::marker::PhantomData;

use super::ManagedTypeApi;

pub struct BigUint<M: ManagedTypeApi> {
    pub handle: i32,
    _phantom: PhantomData<M>,
}
