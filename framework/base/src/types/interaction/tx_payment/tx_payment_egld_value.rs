use crate::types::{AnnotatedValue, BigUint};

use super::TxEnv;

pub trait TxEgldValue<Env>: AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R;
}

impl<Env> TxEgldValue<Env> for BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> TxEgldValue<Env> for &BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(*self)
    }
}

impl<Env> TxEgldValue<Env> for u64
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(&BigUint::<Env::Api>::from(*self))
    }
}
