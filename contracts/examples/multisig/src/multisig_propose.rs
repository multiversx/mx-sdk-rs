use crate::action::{Action, CallActionData};

multiversx_sc::imports!();

/// Contains all events that can be emitted by the contract.
#[multiversx_sc::module]
pub trait MultisigProposeModule: crate::multisig_state::MultisigStateModule {
    fn propose_action(&self, action: Action<Self::Api>) -> usize {
        let (caller_id, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let action_id = self.action_mapper().push(&action);
        if caller_role.can_sign() {
            // also sign
            // since the action is newly created, the caller can be the only signer
            self.action_signer_ids(action_id).insert(caller_id);
        }

        action_id
    }

    /// Initiates board member addition process.
    /// Can also be used to promote a proposer to board member.
    #[endpoint(proposeAddBoardMember)]
    fn propose_add_board_member(&self, board_member_address: ManagedAddress) -> usize {
        self.propose_action(Action::AddBoardMember(board_member_address))
    }

    /// Initiates proposer addition process..
    /// Can also be used to demote a board member to proposer.
    #[endpoint(proposeAddProposer)]
    fn propose_add_proposer(&self, proposer_address: ManagedAddress) -> usize {
        self.propose_action(Action::AddProposer(proposer_address))
    }

    /// Removes user regardless of whether it is a board member or proposer.
    #[endpoint(proposeRemoveUser)]
    fn propose_remove_user(&self, user_address: ManagedAddress) -> usize {
        self.propose_action(Action::RemoveUser(user_address))
    }

    #[endpoint(proposeChangeQuorum)]
    fn propose_change_quorum(&self, new_quorum: usize) -> usize {
        self.propose_action(Action::ChangeQuorum(new_quorum))
    }

    fn prepare_call_data(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
        opt_function: OptionalValue<ManagedBuffer>,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> CallActionData<Self::Api> {
        require!(
            egld_amount > 0 || opt_function.is_some(),
            "proposed action has no effect"
        );

        let endpoint_name = match opt_function {
            OptionalValue::Some(data) => data,
            OptionalValue::None => ManagedBuffer::new(),
        };
        CallActionData {
            to,
            egld_amount,
            endpoint_name,
            arguments: arguments.into_vec_of_buffers(),
        }
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call.
    /// Can send EGLD without calling anything.
    /// Can call smart contract endpoints directly.
    /// Doesn't really work with builtin functions.
    #[endpoint(proposeTransferExecute)]
    fn propose_transfer_execute(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
        opt_function: OptionalValue<ManagedBuffer>,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> usize {
        let call_data = self.prepare_call_data(to, egld_amount, opt_function, arguments);
        self.propose_action(Action::SendTransferExecute(call_data))
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call.
    /// Can call smart contract endpoints directly.
    /// Can use ESDTTransfer/ESDTNFTTransfer/MultiESDTTransfer to send tokens, while also optionally calling endpoints.
    /// Works well with builtin functions.
    /// Cannot simply send EGLD directly without calling anything.
    #[endpoint(proposeAsyncCall)]
    fn propose_async_call(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
        opt_function: OptionalValue<ManagedBuffer>,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> usize {
        let call_data = self.prepare_call_data(to, egld_amount, opt_function, arguments);
        self.propose_action(Action::SendAsyncCall(call_data))
    }

    #[endpoint(proposeSCDeployFromSource)]
    fn propose_sc_deploy_from_source(
        &self,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> usize {
        self.propose_action(Action::SCDeployFromSource {
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCUpgradeFromSource)]
    fn propose_sc_upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> usize {
        self.propose_action(Action::SCUpgradeFromSource {
            sc_address,
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }
}
