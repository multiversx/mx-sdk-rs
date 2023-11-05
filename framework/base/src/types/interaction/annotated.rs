use crate::{
    api::ManagedTypeApi,
    types::{ManagedAddress, ManagedBuffer},
};

pub trait AnnotatedValue<Api, T>
where
    Api: ManagedTypeApi,
{
    fn annotation(&self) -> ManagedBuffer<Api>;

    fn into_value(self) -> T;

    fn with_value_ref<F: FnOnce(&T)>(&self, f: F);
}

impl<Api> AnnotatedValue<Api, ManagedAddress<Api>> for ManagedAddress<Api>
where
    Api: ManagedTypeApi,
{
    fn annotation(&self) -> ManagedBuffer<Api> {
        self.hex_expr()
    }

    fn into_value(self) -> ManagedAddress<Api> {
        self
    }

    fn with_value_ref<F: FnOnce(&ManagedAddress<Api>)>(&self, f: F) {
        f(self)
    }
}

impl<Api> AnnotatedValue<Api, ManagedAddress<Api>> for &ManagedAddress<Api>
where
    Api: ManagedTypeApi,
{
    fn annotation(&self) -> crate::types::ManagedBuffer<Api> {
        self.hex_expr()
    }

    fn into_value(self) -> ManagedAddress<Api> {
        self.clone()
    }

    fn with_value_ref<F: FnOnce(&ManagedAddress<Api>)>(&self, f: F) {
        f(self)
    }
}
