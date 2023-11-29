use crate::{
    action::{Action, ActionFullInfo},
    user_role::UserRole,
};

multiversx_sc::imports!();

/// Gas required to finish transaction after transfer-execute.
const PERFORM_ACTION_FINISH_GAS: u64 = 300_000;

fn usize_add_isize(value: &mut usize, delta: isize) {
    *value = (*value as isize + delta) as usize;
}

/// Contains all events that can be emitted by the contract.
#[multiversx_sc::module]
pub trait MultisigPerformModule:
    crate::multisig_state::MultisigStateModule + crate::multisig_events::MultisigEventsModule
{
    fn gas_for_transfer_exec(&self) -> u64 {
        let gas_left = self.blockchain().get_gas_left();
        if gas_left <= PERFORM_ACTION_FINISH_GAS {
            sc_panic!("insufficient gas for call");
        }
        gas_left - PERFORM_ACTION_FINISH_GAS
    }

    /// Can be used to:
    /// - create new user (board member / proposer)
    /// - remove user (board member / proposer)
    /// - reactivate removed user
    /// - convert between board member and proposer
    /// Will keep the board size and proposer count in sync.
    fn change_user_role(&self, action_id: usize, user_address: ManagedAddress, new_role: UserRole) {
        let user_id = if new_role == UserRole::None {
            // avoid creating a new user just to delete it
            let user_id = self.user_mapper().get_user_id(&user_address);
            if user_id == 0 {
                return;
            }
            user_id
        } else {
            self.user_mapper().get_or_create_user(&user_address)
        };

        let user_id_to_role_mapper = self.user_id_to_role(user_id);
        let old_role = user_id_to_role_mapper.get();
        user_id_to_role_mapper.set(new_role);

        self.perform_change_user_event(action_id, &user_address, old_role, new_role);

        // update board size
        let mut board_members_delta = 0isize;
        if old_role == UserRole::BoardMember {
            board_members_delta -= 1;
        }
        if new_role == UserRole::BoardMember {
            board_members_delta += 1;
        }
        if board_members_delta != 0 {
            self.num_board_members()
                .update(|value| usize_add_isize(value, board_members_delta));
        }

        let mut proposers_delta = 0isize;
        if old_role == UserRole::Proposer {
            proposers_delta -= 1;
        }
        if new_role == UserRole::Proposer {
            proposers_delta += 1;
        }
        if proposers_delta != 0 {
            self.num_proposers()
                .update(|value| usize_add_isize(value, proposers_delta));
        }
    }

    /// Returns `true` (`1`) if `getActionValidSignerCount >= getQuorum`.
    #[view(quorumReached)]
    fn quorum_reached(&self, action_id: usize) -> bool {
        let quorum = self.quorum().get();
        let valid_signers_count = self.get_action_valid_signer_count(action_id);
        valid_signers_count >= quorum
    }

    fn clear_action(&self, action_id: usize) {
        self.action_mapper().clear_entry_unchecked(action_id);
        self.action_signer_ids(action_id).clear();
    }

    /// Proposers and board members use this to launch signed actions.
    #[endpoint(performAction)]
    fn perform_action_endpoint(&self, action_id: usize) -> OptionalValue<ManagedAddress> {
        let (_, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_perform_action(),
            "only board members and proposers can perform actions"
        );
        require!(
            self.quorum_reached(action_id),
            "quorum has not been reached"
        );

        self.perform_action(action_id)
    }

    fn perform_action(&self, action_id: usize) -> OptionalValue<ManagedAddress> {
        let action = self.action_mapper().get(action_id);

        self.start_perform_action_event(&ActionFullInfo {
            action_id,
            action_data: action.clone(),
            signers: self.get_action_signers(action_id),
        });

        // clean up storage
        // happens before actual execution, because the match provides the return on each branch
        // syntax aside, the async_call_raw kills contract execution so cleanup cannot happen afterwards
        self.clear_action(action_id);

        match action {
            Action::Nothing => OptionalValue::None,
            Action::AddBoardMember(board_member_address) => {
                self.change_user_role(action_id, board_member_address, UserRole::BoardMember);
                OptionalValue::None
            },
            Action::AddProposer(proposer_address) => {
                self.change_user_role(action_id, proposer_address, UserRole::Proposer);

                // validation required for the scenario when a board member becomes a proposer
                require!(
                    self.quorum().get() <= self.num_board_members().get(),
                    "quorum cannot exceed board size"
                );
                OptionalValue::None
            },
            Action::RemoveUser(user_address) => {
                self.change_user_role(action_id, user_address, UserRole::None);
                let num_board_members = self.num_board_members().get();
                let num_proposers = self.num_proposers().get();
                require!(
                    num_board_members + num_proposers > 0,
                    "cannot remove all board members and proposers"
                );
                require!(
                    self.quorum().get() <= num_board_members,
                    "quorum cannot exceed board size"
                );
                OptionalValue::None
            },
            Action::ChangeQuorum(new_quorum) => {
                require!(
                    new_quorum <= self.num_board_members().get(),
                    "quorum cannot exceed board size"
                );
                self.quorum().set(new_quorum);
                self.perform_change_quorum_event(action_id, new_quorum);
                OptionalValue::None
            },
            Action::SendTransferExecute(call_data) => {
                let gas = self.gas_for_transfer_exec();
                self.perform_transfer_execute_event(
                    action_id,
                    &call_data.to,
                    &call_data.egld_amount,
                    gas,
                    &call_data.endpoint_name,
                    call_data.arguments.as_multi(),
                );
                let result = self.send_raw().direct_egld_execute(
                    &call_data.to,
                    &call_data.egld_amount,
                    gas,
                    &call_data.endpoint_name,
                    &call_data.arguments.into(),
                );
                if let Result::Err(e) = result {
                    sc_panic!(e);
                }
                OptionalValue::None
            },
            Action::SendAsyncCall(call_data) => {
                let gas_left = self.blockchain().get_gas_left();
                self.perform_async_call_event(
                    action_id,
                    &call_data.to,
                    &call_data.egld_amount,
                    gas_left,
                    &call_data.endpoint_name,
                    call_data.arguments.as_multi(),
                );
                self.send()
                    .contract_call::<()>(call_data.to, call_data.endpoint_name)
                    .with_egld_transfer(call_data.egld_amount)
                    .with_raw_arguments(call_data.arguments.into())
                    .async_call()
                    .with_callback(self.callbacks().perform_async_call_callback())
                    .call_and_exit()
            },
            Action::SCDeployFromSource {
                amount,
                source,
                code_metadata,
                arguments,
            } => {
                let gas_left = self.blockchain().get_gas_left();
                self.perform_deploy_from_source_event(
                    action_id,
                    &amount,
                    &source,
                    code_metadata,
                    gas_left,
                    arguments.as_multi(),
                );
                let (new_address, _) = self.send_raw().deploy_from_source_contract(
                    gas_left,
                    &amount,
                    &source,
                    code_metadata,
                    &arguments.into(),
                );
                OptionalValue::Some(new_address)
            },
            Action::SCUpgradeFromSource {
                sc_address,
                amount,
                source,
                code_metadata,
                arguments,
            } => {
                let gas_left = self.blockchain().get_gas_left();
                self.perform_upgrade_from_source_event(
                    action_id,
                    &sc_address,
                    &amount,
                    &source,
                    code_metadata,
                    gas_left,
                    arguments.as_multi(),
                );
                self.send_raw().upgrade_from_source_contract(
                    &sc_address,
                    gas_left,
                    &amount,
                    &source,
                    code_metadata,
                    &arguments.into(),
                );
                OptionalValue::None
            },
        }
    }

    /// Callback only performs logging.
    #[callback]
    fn perform_async_call_callback(
        &self,
        #[call_result] call_result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) {
        match call_result {
            ManagedAsyncCallResult::Ok(results) => {
                self.async_call_success(results);
            },
            ManagedAsyncCallResult::Err(err) => {
                self.async_call_error(err.err_code, err.err_msg);
            },
        }
    }

    #[event("asyncCallSuccess")]
    fn async_call_success(&self, #[indexed] results: MultiValueEncoded<ManagedBuffer>);

    #[event("asyncCallError")]
    fn async_call_error(&self, #[indexed] err_code: u32, #[indexed] err_message: ManagedBuffer);
}
