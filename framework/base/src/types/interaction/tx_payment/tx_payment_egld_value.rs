use crate::types::{AnnotatedValue, BigUint};

use super::TxEnv;

pub trait TxEgldValue<Env>: AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
}

impl<Env> TxEgldValue<Env> for BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxEgldValue<Env> for &BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxEgldValue<Env> for u64 where Env: TxEnv {}
