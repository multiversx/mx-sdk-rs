use elrond_wasm::{
    api::{EndpointFinishApi, ManagedTypeApi, SendApi, StorageWriteApi},
    io::EndpointResult,
    types::{
        AsyncCall, BigUint, BoxedBytes, CodeMetadata, EsdtTokenPayment, ManagedAddress,
        ManagedBuffer, ManagedVec, OptionalResult, SendEgld, Vec,
    },
};

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum Action<M: ManagedTypeApi> {
    Nothing,
    AddBoardMember(ManagedAddress<M>),
    AddProposer(ManagedAddress<M>),
    RemoveUser(ManagedAddress<M>),
    ChangeQuorum(usize),
    SendEgld {
        to: ManagedAddress<M>,
        amount: BigUint<M>,
        data: BoxedBytes,
    },
    SCAsyncCall {
        to: ManagedAddress<M>,
        egld_payment: BigUint<M>,
        endpoint_name: BoxedBytes,
        arguments: Vec<BoxedBytes>,
    },
    SCSyncCall {
        to: ManagedAddress<M>,
        egld_payment: BigUint<M>,
        endpoint_name: BoxedBytes,
        arguments: Vec<BoxedBytes>,
    },
    SCDeploy {
        amount: BigUint<M>,
        code: ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
    SCDeployFromSource {
        amount: BigUint<M>,
        source: ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
    SCUpgrade {
        sc_address: ManagedAddress<M>,
        amount: BigUint<M>,
        code: ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
    SCUpgradeFromSource {
        sc_address: ManagedAddress<M>,
        amount: BigUint<M>,
        source: ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
    ESDTTransferExecute {
        to: ManagedAddress<M>,
        payments: Vec<EsdtTokenPayment<M>>,
        endpoint_name: ManagedBuffer<M>,
        arguments: Vec<BoxedBytes>,
    },
}

impl<M: ManagedTypeApi> Action<M> {
    /// Only pending actions are kept in storage,
    /// both executed and discarded actions are removed (converted to `Nothing`).
    /// So this is equivalent to `action != Action::Nothing`.
    pub fn is_pending(&self) -> bool {
        !matches!(*self, Action::Nothing)
    }
}

/// Not used internally, just to retrieve results via endpoint.
#[derive(TopEncode, TypeAbi)]
pub struct ActionFullInfo<M: ManagedTypeApi> {
    pub action_id: usize,
    pub action_data: Action<M>,
    pub signers: Vec<ManagedAddress<M>>,
}

#[derive(TypeAbi)]
pub enum PerformActionResult<SA>
where
    SA: SendApi + ManagedTypeApi + StorageWriteApi + 'static,
{
    Nothing,
    SendEgld(SendEgld<SA>),
    DeployResult(ManagedAddress<SA>),
    SendAsyncCall(AsyncCall<SA>),
    ExecOnDestContext(ManagedVec<SA, ManagedBuffer<SA>>),
}

impl<SA> EndpointResult for PerformActionResult<SA>
where
    SA: SendApi + StorageWriteApi + Clone + 'static,
{
    type DecodeAs = OptionalResult<ManagedAddress<SA>>;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        match self {
            PerformActionResult::Nothing => (),
            PerformActionResult::SendEgld(send_egld) => send_egld.finish(api),
            PerformActionResult::DeployResult(address) => address.finish(api),
            PerformActionResult::SendAsyncCall(async_call) => async_call.finish(api),
            PerformActionResult::ExecOnDestContext(exec) => exec.finish(api),
        }
    }
}

#[cfg(test)]
mod test {
    use elrond_wasm_debug::DebugApi;

    use super::Action;

    #[test]
    fn test_is_pending() {
        assert!(!Action::<DebugApi>::Nothing.is_pending());
        assert!(Action::<DebugApi>::ChangeQuorum(5).is_pending());
    }
}
