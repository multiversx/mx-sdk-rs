use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, CodeMetadata, ManagedAddress, ManagedBuffer, ManagedVec},
};

use multiversx_sc::derive_imports::*;

#[derive(NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct CallActionData<'a, M: ManagedTypeApi<'a>> {
    pub to: ManagedAddress<'a, M>,
    pub egld_amount: BigUint<'a, M>,
    pub endpoint_name: ManagedBuffer<'a, M>,
    pub arguments: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone)]
pub enum Action<'a, M: ManagedTypeApi<'a>> {
    Nothing,
    AddBoardMember(ManagedAddress<'a, M>),
    AddProposer(ManagedAddress<'a, M>),
    RemoveUser(ManagedAddress<'a, M>),
    ChangeQuorum(usize),
    SendTransferExecute(CallActionData<'a, M>),
    SendAsyncCall(CallActionData<'a, M>),
    SCDeployFromSource {
        amount: BigUint<'a, M>,
        source: ManagedAddress<'a, M>,
        code_metadata: CodeMetadata,
        arguments: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
    },
    SCUpgradeFromSource {
        sc_address: ManagedAddress<'a, M>,
        amount: BigUint<'a, M>,
        source: ManagedAddress<'a, M>,
        code_metadata: CodeMetadata,
        arguments: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
    },
}

impl<'a, M: ManagedTypeApi<'a>> Action<'a, M> {
    /// Only pending actions are kept in storage,
    /// both executed and discarded actions are removed (converted to `Nothing`).
    /// So this is equivalent to `action != Action::Nothing`.
    pub fn is_pending(&self) -> bool {
        !matches!(*self, Action::Nothing)
    }
}

/// Not used internally, just to retrieve results via endpoint.
#[derive(TopEncode, TypeAbi)]
pub struct ActionFullInfo<'a, M: ManagedTypeApi<'a>> {
    pub action_id: usize,
    pub action_data: Action<'a, M>,
    pub signers: ManagedVec<'a, M, ManagedAddress<'a, M>>,
}

#[cfg(test)]
mod test {
    use multiversx_sc_scenario::api::StaticApi;

    use super::Action;

    #[test]
    fn test_is_pending() {
        assert!(!Action::<StaticApi>::Nothing.is_pending());
        assert!(Action::<StaticApi>::ChangeQuorum(5).is_pending());
    }
}
