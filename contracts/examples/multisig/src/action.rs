use elrond_wasm::{
    api::{CallTypeApi, EndpointFinishApi, ManagedTypeApi, StorageWriteApi},
    io::EndpointResult,
    types::{
        AsyncCall, BigUint, CodeMetadata, ManagedAddress, ManagedBuffer, ManagedVec, OptionalResult,
    },
};

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi)]
pub struct CallActionData<M: ManagedTypeApi> {
    pub to: ManagedAddress<M>,
    pub egld_amount: BigUint<M>,
    pub endpoint_name: ManagedBuffer<M>,
    pub arguments: ManagedVec<M, ManagedBuffer<M>>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum Action<M: ManagedTypeApi> {
    Nothing,
    AddBoardMember(ManagedAddress<M>),
    AddProposer(ManagedAddress<M>),
    RemoveUser(ManagedAddress<M>),
    ChangeQuorum(usize),
    SendTransferExecute(CallActionData<M>),
    SendAsyncCall(CallActionData<M>),
    SCDeployFromSource {
        amount: BigUint<M>,
        source: ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arguments: ManagedVec<M, ManagedBuffer<M>>,
    },
    SCUpgradeFromSource {
        sc_address: ManagedAddress<M>,
        amount: BigUint<M>,
        source: ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arguments: ManagedVec<M, ManagedBuffer<M>>,
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
    pub signers: ManagedVec<M, ManagedAddress<M>>,
}

#[derive(TypeAbi)]
pub enum PerformActionResult<SA>
where
    SA: CallTypeApi + ManagedTypeApi + StorageWriteApi + 'static,
{
    Nothing,
    DeployResult(ManagedAddress<SA>),
    SendAsyncCall(AsyncCall<SA>),
}

impl<SA> EndpointResult for PerformActionResult<SA>
where
    SA: CallTypeApi + StorageWriteApi,
{
    type DecodeAs = OptionalResult<ManagedAddress<SA>>;

    fn finish<FA>(&self)
    where
        FA: ManagedTypeApi + EndpointFinishApi,
    {
        match self {
            PerformActionResult::Nothing => (),
            PerformActionResult::DeployResult(address) => address.finish::<FA>(),
            PerformActionResult::SendAsyncCall(async_call) => async_call.finish::<FA>(),
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
