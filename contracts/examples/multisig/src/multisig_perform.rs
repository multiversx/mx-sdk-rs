use crate::{
    action::{Action, PerformActionResult},
    user_role::UserRole,
};

elrond_wasm::imports!();

/// Gas required to finsh transaction after transfer-execute.
const PERFORM_ACTION_FINISH_GAS: u64 = 200_000;

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait MultisigPerformModule: crate::multisig_state::MultisigStateModule {
    fn gas_for_transfer_exec(&self) -> u64 {
        self.blockchain().get_gas_left() - PERFORM_ACTION_FINISH_GAS
    }

    /// Can be used to:
    /// - create new user (board member / proposer)
    /// - remove user (board member / proposer)
    /// - reactivate removed user
    /// - convert between board member and proposer
    /// Will keep the board size and proposer count in sync.
    fn change_user_role(&self, user_address: ManagedAddress, new_role: UserRole) {
        let user_id = self.user_mapper().get_or_create_user(&user_address);
        let old_role = if user_id == 0 {
            UserRole::None
        } else {
            self.get_user_id_to_role(user_id)
        };
        self.set_user_id_to_role(user_id, new_role);

        // update board size
        #[allow(clippy::collapsible_else_if)]
        if old_role == UserRole::BoardMember {
            if new_role != UserRole::BoardMember {
                self.num_board_members().update(|value| *value -= 1);
            }
        } else {
            if new_role == UserRole::BoardMember {
                self.num_board_members().update(|value| *value += 1);
            }
        }

        // update num_proposers
        #[allow(clippy::collapsible_else_if)]
        if old_role == UserRole::Proposer {
            if new_role != UserRole::Proposer {
                self.num_proposers().update(|value| *value -= 1);
            }
        } else {
            if new_role == UserRole::Proposer {
                self.num_proposers().update(|value| *value += 1);
            }
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
    fn perform_action_endpoint(
        &self,
        action_id: usize,
    ) -> SCResult<PerformActionResult<Self::Api>> {
        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.get_user_id_to_role(caller_id);
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

    fn perform_action(&self, action_id: usize) -> SCResult<PerformActionResult<Self::Api>> {
        let action = self.action_mapper().get(action_id);

        // clean up storage
        // happens before actual execution, because the match provides the return on each branch
        // syntax aside, the async_call_raw kills contract execution so cleanup cannot happen afterwards
        self.clear_action(action_id);

        match action {
            Action::Nothing => Ok(PerformActionResult::Nothing),
            Action::AddBoardMember(board_member_address) => {
                self.change_user_role(board_member_address, UserRole::BoardMember);
                Ok(PerformActionResult::Nothing)
            },
            Action::AddProposer(proposer_address) => {
                self.change_user_role(proposer_address, UserRole::Proposer);

                // validation required for the scenario when a board member becomes a proposer
                require!(
                    self.quorum().get() <= self.num_board_members().get(),
                    "quorum cannot exceed board size"
                );
                Ok(PerformActionResult::Nothing)
            },
            Action::RemoveUser(user_address) => {
                self.change_user_role(user_address, UserRole::None);
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
                Ok(PerformActionResult::Nothing)
            },
            Action::ChangeQuorum(new_quorum) => {
                require!(
                    new_quorum <= self.num_board_members().get(),
                    "quorum cannot exceed board size"
                );
                self.quorum().set(&new_quorum);
                Ok(PerformActionResult::Nothing)
            },
            Action::SendEGLD {
                to,
                amount,
                endpoint_name,
                arguments,
            } => {
                let result = self.raw_vm_api().direct_egld_execute(
                    &to,
                    &amount,
                    self.gas_for_transfer_exec(),
                    &endpoint_name,
                    &arguments.into(),
                );
                if let Result::Err(e) = result {
                    self.raw_vm_api().signal_error(e);
                }
                Ok(PerformActionResult::Nothing)
            },
            Action::SendESDT {
                to,
                esdt_payments,
                endpoint_name,
                arguments,
            } => {
                let result = self.raw_vm_api().direct_multi_esdt_transfer_execute(
                    &to,
                    &esdt_payments,
                    self.gas_for_transfer_exec(),
                    &endpoint_name,
                    &arguments.into(),
                );
                if let Result::Err(e) = result {
                    self.raw_vm_api().signal_error(e);
                }
                Ok(PerformActionResult::Nothing)
            },
            Action::SCDeploy {
                amount,
                code,
                code_metadata,
                arguments,
            } => {
                let gas_left = self.blockchain().get_gas_left();
                let (new_address, _) = self.raw_vm_api().deploy_contract(
                    gas_left,
                    &amount,
                    &code,
                    code_metadata,
                    &arguments.into(),
                );
                Ok(PerformActionResult::DeployResult(new_address))
            },
            Action::SCDeployFromSource {
                amount,
                source,
                code_metadata,
                arguments,
            } => {
                let gas_left = self.blockchain().get_gas_left();
                let (new_address, _) = self.raw_vm_api().deploy_from_source_contract(
                    gas_left,
                    &amount,
                    &source,
                    code_metadata,
                    &arguments.into(),
                );
                Ok(PerformActionResult::DeployResult(new_address))
            },
            Action::SCUpgrade {
                sc_address,
                amount,
                code,
                code_metadata,
                arguments,
            } => {
                let gas_left = self.blockchain().get_gas_left();
                self.raw_vm_api().upgrade_contract(
                    &sc_address,
                    gas_left,
                    &amount,
                    &code,
                    code_metadata,
                    &arguments.into(),
                );
                Ok(PerformActionResult::Nothing)
            },
            Action::SCUpgradeFromSource {
                sc_address,
                amount,
                source,
                code_metadata,
                arguments,
            } => {
                let gas_left = self.blockchain().get_gas_left();
                self.raw_vm_api().upgrade_from_source_contract(
                    &sc_address,
                    gas_left,
                    &amount,
                    &source,
                    code_metadata,
                    &arguments.into(),
                );
                Ok(PerformActionResult::Nothing)
            },
        }
    }
}
