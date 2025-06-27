use multiversx_chain_core::std::Bech32Address;

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{
        AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified, TxTo,
        TxToSpecified,
    },
};

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for Bech32Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_bech32_expr().into()
    }

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.to_value(env)
    }
}

impl<Env> TxFrom<Env> for Bech32Address
where
    Env: TxEnv,
{
    fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.resolve_address(env)
    }
}
impl<Env> TxFromSpecified<Env> for Bech32Address where Env: TxEnv {}
impl<Env> TxTo<Env> for Bech32Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for Bech32Address where Env: TxEnv {}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &Bech32Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_bech32_expr().into()
    }

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.to_value(env)
    }
}

impl<Env> TxFrom<Env> for &Bech32Address
where
    Env: TxEnv,
{
    fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.resolve_address(env)
    }
}
impl<Env> TxFromSpecified<Env> for &Bech32Address where Env: TxEnv {}
impl<Env> TxTo<Env> for &Bech32Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &Bech32Address where Env: TxEnv {}

impl<M> TypeAbiFrom<Bech32Address> for ManagedAddress<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Bech32Address> for ManagedAddress<M> where M: ManagedTypeApi {}
