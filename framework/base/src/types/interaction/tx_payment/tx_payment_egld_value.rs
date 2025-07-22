use crate::types::{AnnotatedValue, BigUint, ManagedRef};

use super::TxEnv;

pub trait TxEgldValue<Env>: AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
}

impl<Env> TxEgldValue<Env> for BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxEgldValue<Env> for &BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxEgldValue<Env> for ManagedRef<'_, Env::Api, BigUint<Env::Api>> where Env: TxEnv {}
impl<Env> TxEgldValue<Env> for i32 where Env: TxEnv {}
impl<Env> TxEgldValue<Env> for u64 where Env: TxEnv {}
impl<Env> TxEgldValue<Env> for u128 where Env: TxEnv {}
