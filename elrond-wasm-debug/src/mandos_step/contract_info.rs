use std::ops::{Deref, DerefMut};

use crate::{sc_call, sc_query, world_mock::BlockchainMock, DebugApi};
use elrond_wasm::{
    contract_base::ProxyObjBase,
    elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    types::ContractCall,
};
use mandos::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressKey, AddressValue, ScCallStep, ScQueryStep, Step},
};

/// Bundles a mandos representation of a contract with the contract proxy,
/// so that it can be easily called in the context of a blockchain mock.
pub struct ContractInfo<P: ProxyObjBase> {
    mandos_address_expr: AddressKey,
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

impl BlockchainMock {
    /// Performs a SC query to a contract, leaves a mandos trace behind.
    pub fn sc_query<OriginalResult, RequestedResult>(
        &mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let function =
            String::from_utf8(contract_call.endpoint_name.to_boxed_bytes().into_vec()).unwrap();
        let to_str = format!(
            "0x{}",
            hex::encode(contract_call.to.to_address().as_bytes())
        );
        let mut sc_query_step = ScQueryStep::new()
            .to(to_str.as_str())
            .function(function.as_str());

        for arg in contract_call.arg_buffer.to_raw_args_vec() {
            let arg_str = format!("0x{}", hex::encode(&arg));
            sc_query_step = sc_query_step.argument(arg_str.as_str());
        }

        let tx_result = self.with_borrowed(|state| sc_query::execute(state, &sc_query_step));
        self.mandos_trace.steps.push(Step::ScQuery(sc_query_step));

        let mut raw_result = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }

    /// Performs a SC query to a contract, leaves a mandos trace behind.
    pub fn sc_call<AE, OriginalResult, RequestedResult>(
        &mut self,
        from: AE,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> RequestedResult
    where
        AddressValue: InterpretableFrom<AE>,
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let function =
            String::from_utf8(contract_call.endpoint_name.to_boxed_bytes().into_vec()).unwrap();
        let to_str = format!(
            "0x{}",
            hex::encode(contract_call.to.to_address().as_bytes())
        );
        let mut sc_call_step = ScCallStep::new()
            .from(from)
            .to(to_str.as_str())
            .function(function.as_str());

        for arg in contract_call.arg_buffer.to_raw_args_vec() {
            let arg_str = format!("0x{}", hex::encode(&arg));
            sc_call_step = sc_call_step.argument(arg_str.as_str());
        }

        let tx_result = self.with_borrowed(|state| sc_call::execute(state, &sc_call_step));
        self.mandos_trace.steps.push(Step::ScCall(sc_call_step));

        let mut raw_result = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}
