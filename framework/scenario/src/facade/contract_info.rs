use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use multiversx_sc::api::VMApi;

use crate::multiversx_sc::{
    api::ManagedTypeApi,
    codec::{CodecFrom, EncodeErrorHandler, TopEncode, TopEncodeOutput},
    contract_base::ProxyObjBase,
    types::{Address, ManagedAddress},
};

use crate::scenario::model::{AddressKey, AddressValue};

/// Bundles a representation of a contract with the contract proxy,
/// so that it can be easily called in the context of a blockchain mock.
pub struct ContractInfo<Api: VMApi, P: ProxyObjBase<Api>> {
    pub scenario_address_expr: AddressKey,
    proxy_inst: P,
    _phantom: PhantomData<Api>
}

impl<Api: VMApi, P: ProxyObjBase<Api>> ContractInfo<Api, P> {
    pub fn new<A>(address_expr: A) -> Self
    where
        AddressKey: From<A>,
    {
        let mandos_address_expr = AddressKey::from(address_expr);
        let proxy_inst = P::new_proxy_obj().contract(mandos_address_expr.value.clone().into());
        ContractInfo {
            scenario_address_expr: mandos_address_expr,
            proxy_inst,
            _phantom: PhantomData
        }
    }

    pub fn to_address(&self) -> Address {
        self.scenario_address_expr.to_address()
    }
}

impl<Api: VMApi, P: ProxyObjBase<Api>> From<&ContractInfo<Api, P>> for AddressKey {
    fn from(from: &ContractInfo<Api, P>) -> Self {
        from.scenario_address_expr.clone()
    }
}

impl<Api: VMApi, P: ProxyObjBase<Api>> From<ContractInfo<Api, P>> for AddressKey {
    fn from(from: ContractInfo<Api, P>) -> Self {
        from.scenario_address_expr
    }
}

impl<Api: VMApi, P: ProxyObjBase<Api>> From<&ContractInfo<Api, P>> for AddressValue {
    fn from(from: &ContractInfo<Api, P>) -> Self {
        AddressValue::from(&from.scenario_address_expr)
    }
}

impl<Api: VMApi, P: ProxyObjBase<Api>> From<ContractInfo<Api, P>> for AddressValue {
    fn from(from: ContractInfo<Api, P>) -> Self {
        AddressValue::from(&from.scenario_address_expr)
    }
}

impl<Api: VMApi, P: ProxyObjBase<Api>> Deref for ContractInfo<Api, P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        &self.proxy_inst
    }
}

impl<Api: VMApi, P: ProxyObjBase<Api>> DerefMut for ContractInfo<Api, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let proxy_inst = core::mem::replace(&mut self.proxy_inst, P::new_proxy_obj());
        let proxy_inst = proxy_inst.contract(self.scenario_address_expr.value.clone().into());
        let _ = core::mem::replace(&mut self.proxy_inst, proxy_inst);
        &mut self.proxy_inst
    }
}

impl<Api: VMApi, P: ProxyObjBase<Api>> TopEncode for ContractInfo<Api, P> {
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

impl<Api: VMApi, P: ProxyObjBase<Api>> CodecFrom<ContractInfo<Api, P>> for Address {}
impl<Api: VMApi, P: ProxyObjBase<Api>> CodecFrom<&ContractInfo<Api, P>> for Address {}
impl<M: ManagedTypeApi + VMApi, P: ProxyObjBase<M>> CodecFrom<ContractInfo<M, P>> for ManagedAddress<M> {}
impl<M: ManagedTypeApi + VMApi, P: ProxyObjBase<M>> CodecFrom<&ContractInfo<M, P>> for ManagedAddress<M> {}
