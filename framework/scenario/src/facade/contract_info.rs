use std::ops::{Deref, DerefMut};

use multiversx_sc::{
    abi::TypeAbiFrom,
    types::{AnnotatedValue, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified, TxTo, TxToSpecified},
};

use crate::multiversx_sc::{
    api::ManagedTypeApi,
    codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput},
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

    /// For historical reasons the proxy consumes its address whenever it is called.
    ///
    /// When using it in tests, as part of `ContractInfo`,
    /// it is convenient to refresh it before each call.
    ///
    /// It is sort of a hack, designed to optimize proxy use in contracts,
    /// while making it easier to use in tests.
    fn refresh_proxy_address(&mut self) {
        self.proxy_inst =
            P::new_proxy_obj().contract(self.scenario_address_expr.value.clone().into());
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
        self.refresh_proxy_address();
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

impl<P: ProxyObjNew> TypeAbiFrom<ContractInfo<P>> for Address {}
impl<P: ProxyObjNew> TypeAbiFrom<&ContractInfo<P>> for Address {}
impl<M: ManagedTypeApi, P: ProxyObjNew> TypeAbiFrom<ContractInfo<P>> for ManagedAddress<M> {}
impl<M: ManagedTypeApi, P: ProxyObjNew> TypeAbiFrom<&ContractInfo<P>> for ManagedAddress<M> {}

impl<Env, P> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &ContractInfo<P>
where
    Env: TxEnv,
    P: ProxyObjNew,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.scenario_address_expr.original.as_str().into()
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        (&self.scenario_address_expr.value).into()
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
