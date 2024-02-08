use crate::types::{ManagedAddress, ManagedBuffer};

use super::TxEnv;

pub trait AnnotatedValue<Env, T>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api>;

    fn into_value(self) -> T;

    fn with_value_ref<F: FnOnce(&T)>(&self, f: F);
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn into_value(self) -> ManagedAddress<Env::Api> {
        self
    }

    fn with_value_ref<F: FnOnce(&ManagedAddress<Env::Api>)>(&self, f: F) {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> crate::types::ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn into_value(self) -> ManagedAddress<Env::Api> {
        self.clone()
    }

    fn with_value_ref<F: FnOnce(&ManagedAddress<Env::Api>)>(&self, f: F) {
        f(self)
    }
}
