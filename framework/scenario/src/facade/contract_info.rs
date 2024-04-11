use std::ops::{Deref, DerefMut};

use crate::multiversx_sc::{
    api::ManagedTypeApi,
    codec::{CodecFrom, EncodeErrorHandler, TopEncode, TopEncodeOutput},
    contract_base::ProxyObjBase,
    types::{Address, ManagedAddress},
};

use crate::scenario::model::{AddressKey, AddressValue};

/// Bundles a representation of a contract with the contract proxy,
/// so that it can be easily called in the context of a blockchain mock.
pub struct ContractInfo<P: ProxyObjBase> {
    pub scenario_address_expr: AddressKey,
    proxy_inst: P,
}

impl<P: ProxyObjBase> ContractInfo<P> {
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

impl<P: ProxyObjBase> From<&ContractInfo<P>> for AddressKey {
    fn from(from: &ContractInfo<P>) -> Self {
        from.scenario_address_expr.clone()
    }
}

impl<P: ProxyObjBase> From<ContractInfo<P>> for AddressKey {
    fn from(from: ContractInfo<P>) -> Self {
        from.scenario_address_expr
    }
}

impl<P: ProxyObjBase> From<&ContractInfo<P>> for AddressValue {
    fn from(from: &ContractInfo<P>) -> Self {
        AddressValue::from(&from.scenario_address_expr)
    }
}

impl<P: ProxyObjBase> From<ContractInfo<P>> for AddressValue {
    fn from(from: ContractInfo<P>) -> Self {
        AddressValue::from(&from.scenario_address_expr)
    }
}

impl<P: ProxyObjBase> Deref for ContractInfo<P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        &self.proxy_inst
    }
}

impl<P: ProxyObjBase> DerefMut for ContractInfo<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let proxy_inst = core::mem::replace(&mut self.proxy_inst, P::new_proxy_obj());
        let proxy_inst = proxy_inst.contract(self.scenario_address_expr.value.clone().into());
        let _ = core::mem::replace(&mut self.proxy_inst, proxy_inst);
        &mut self.proxy_inst
    }
}

impl<P: ProxyObjBase> TopEncode for ContractInfo<P> {
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

impl<P: ProxyObjBase> CodecFrom<ContractInfo<P>> for Address {}
impl<P: ProxyObjBase> CodecFrom<&ContractInfo<P>> for Address {}
impl<M: ManagedTypeApi, P: ProxyObjBase> CodecFrom<ContractInfo<P>> for ManagedAddress<M> {}
impl<M: ManagedTypeApi, P: ProxyObjBase> CodecFrom<&ContractInfo<P>> for ManagedAddress<M> {}
