use std::ops::{Deref, DerefMut};

use multiversx_sc::types::{
    AnnotatedValue, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified, TxTo, TxToSpecified,
};

use crate::multiversx_sc::{
    api::ManagedTypeApi,
    codec::{CodecFrom, EncodeErrorHandler, TopEncode, TopEncodeOutput},
    contract_base::ProxyObjNew,
    types::{Address, ManagedAddress},
};

use crate::scenario::model::{AddressKey, AddressValue};

/// Bundles a representation of a contract with the contract proxy,
/// so that it can be easily called in the context of a blockchain mock.
pub struct ContractInfo<P: ProxyObjNew> {
    pub scenario_address_expr: AddressKey,
    proxy_inst: P::ProxyTo,
}

impl<P> ContractInfo<P>
where
    P: ProxyObjNew,
{
    pub fn new<A>(address_expr: A) -> Self
    where
        AddressKey: From<A>,
    {
        let mandos_address_expr = AddressKey::from(address_expr);
        let proxy_inst = P::new_proxy_obj().contract(mandos_address_expr.value.clone().into());
        ContractInfo {
            scenario_address_expr: mandos_address_expr,
            proxy_inst,
        }
    }

    pub fn to_address(&self) -> Address {
        self.scenario_address_expr.to_address()
    }
}

impl<P: ProxyObjNew> From<&ContractInfo<P>> for AddressKey {
    fn from(from: &ContractInfo<P>) -> Self {
        from.scenario_address_expr.clone()
    }
}

impl<P: ProxyObjNew> From<ContractInfo<P>> for AddressKey {
    fn from(from: ContractInfo<P>) -> Self {
        from.scenario_address_expr
    }
}

impl<P: ProxyObjNew> From<&ContractInfo<P>> for AddressValue {
    fn from(from: &ContractInfo<P>) -> Self {
        AddressValue::from(&from.scenario_address_expr)
    }
}

impl<P: ProxyObjNew> From<ContractInfo<P>> for AddressValue {
    fn from(from: ContractInfo<P>) -> Self {
        AddressValue::from(&from.scenario_address_expr)
    }
}

impl<P: ProxyObjNew> Deref for ContractInfo<P> {
    type Target = P::ProxyTo;
    fn deref(&self) -> &Self::Target {
        &self.proxy_inst
    }
}

impl<P: ProxyObjNew> DerefMut for ContractInfo<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.proxy_inst
    }
}

impl<P: ProxyObjNew> TopEncode for ContractInfo<P> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.scenario_address_expr
            .value
            .top_encode_or_handle_err(output, h)
    }
}

impl<P: ProxyObjNew> CodecFrom<ContractInfo<P>> for Address {}
impl<P: ProxyObjNew> CodecFrom<&ContractInfo<P>> for Address {}
impl<M: ManagedTypeApi, P: ProxyObjNew> CodecFrom<ContractInfo<P>> for ManagedAddress<M> {}
impl<M: ManagedTypeApi, P: ProxyObjNew> CodecFrom<&ContractInfo<P>> for ManagedAddress<M> {}

impl<Env, P> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &ContractInfo<P>
where
    Env: TxEnv,
    P: ProxyObjNew,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.scenario_address_expr.original.as_str().into()
    }

    fn into_value(self) -> ManagedAddress<Env::Api> {
        (&self.scenario_address_expr.value).into()
    }

    fn with_value_ref<F: FnOnce(&ManagedAddress<Env::Api>)>(&self, f: F) {
        let ma: ManagedAddress<Env::Api> = (&self.scenario_address_expr.value).into();
        f(&ma);
    }
}
impl<P, Env> TxFrom<Env> for &ContractInfo<P>
where
    Env: TxEnv,
    P: ProxyObjNew,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        (&self.scenario_address_expr.value).into()
    }
}
impl<P, Env> TxFromSpecified<Env> for &ContractInfo<P>
where
    Env: TxEnv,
    P: ProxyObjNew,
{
}
impl<P, Env> TxTo<Env> for &ContractInfo<P>
where
    Env: TxEnv,
    P: ProxyObjNew,
{
}
impl<P, Env> TxToSpecified<Env> for &ContractInfo<P>
where
    Env: TxEnv,
    P: ProxyObjNew,
{
}
