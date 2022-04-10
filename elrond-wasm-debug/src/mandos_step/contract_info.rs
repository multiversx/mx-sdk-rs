use std::ops::{Deref, DerefMut};

use elrond_wasm::contract_base::ProxyObjBase;
use mandos::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressKey, AddressValue},
};

/// Bundles a mandos representation of a contract with the contract proxy,
/// so that it can be easily called in the context of a blockchain mock.
pub struct ContractInfo<P: ProxyObjBase> {
    pub mandos_address_expr: AddressKey,
    proxy_inst: P,
}

impl<P: ProxyObjBase> ContractInfo<P> {
    pub fn new<A>(address_expr: A, ic: &InterpreterContext) -> Self
    where
        AddressKey: InterpretableFrom<A>,
    {
        let mandos_address_expr = AddressKey::interpret_from(address_expr, ic);
        let proxy_inst = P::new_proxy_obj().contract(mandos_address_expr.value.into());
        ContractInfo {
            mandos_address_expr,
            proxy_inst,
        }
    }
}

impl<P: ProxyObjBase> InterpretableFrom<&ContractInfo<P>> for AddressKey {
    fn interpret_from(from: &ContractInfo<P>, _context: &InterpreterContext) -> Self {
        from.mandos_address_expr.clone()
    }
}

impl<P: ProxyObjBase> InterpretableFrom<&ContractInfo<P>> for AddressValue {
    fn interpret_from(from: &ContractInfo<P>, context: &InterpreterContext) -> Self {
        AddressValue::interpret_from(&from.mandos_address_expr, context)
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
        let proxy_inst = proxy_inst.contract(self.mandos_address_expr.value.into());
        let _ = core::mem::replace(&mut self.proxy_inst, proxy_inst);
        &mut self.proxy_inst
    }
}
