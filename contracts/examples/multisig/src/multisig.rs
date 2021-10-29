#![no_std]

mod action;
mod multisig_perform;
mod multisig_propose;
mod multisig_state;
mod user_role;

use action::{Action, ActionFullInfo};
use user_role::UserRole;

elrond_wasm::imports!();

/// Multi-signature smart contract implementation.
/// Acts like a wallet that needs multiple signers for any action performed.
/// See the readme file for more detailed documentation.
#[elrond_wasm::contract]
pub trait Multisig:
    multisig_state::MultisigStateModule
    + multisig_propose::MultisigProposeModule
    + multisig_perform::MultisigPerformModule
{
    #[init]
    fn init(
        &self,
        quorum: usize,
        #[var_args] board: ManagedVarArgs<ManagedAddress>,
    ) -> SCResult<()> {
        let board_vec = board.to_vec();
        require!(
            !board_vec.is_empty(),
            "board cannot be empty on init, no-one would be able to propose"
        );
        require!(quorum <= board_vec.len(), "quorum cannot exceed board size");
        self.quorum().set(&quorum);

        let mut duplicates = false;
        self.user_mapper()
            .get_or_create_users(board_vec.into_iter(), |user_id, new_user| {
                if !new_user {
                    duplicates = true;
                }
                self.set_user_id_to_role(user_id, UserRole::BoardMember);
            });
        require!(!duplicates, "duplicate board member");
        self.num_board_members().set(&board_vec.len());

        Ok(())
    }

    /// Allows the contract to receive funds even if it is marked as unpayable in the protocol.
    #[payable("*")]
    #[endpoint]
    fn deposit(&self) {}

    fn propose_action(&self, action: Action<Self::Api>) -> SCResult<usize> {
        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.get_user_id_to_role(caller_id);
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let action_id = self.action_mapper().push(&action);
        if caller_role.can_sign() {
            // also sign
            // since the action is newly created, the caller can be the only signer
            self.action_signer_ids(action_id).set(&[caller_id].to_vec());
        }

        Ok(action_id)
    }

    /// Iterates through all actions and retrieves those that are still pending.
    /// Serialized full action data:
    /// - the action id
    /// - the serialized action data
    /// - (number of signers followed by) list of signer addresses.
    #[view(getPendingActionFullInfo)]
    fn get_pending_action_full_info(&self) -> MultiResultVec<ActionFullInfo<Self::Api>> {
        let mut result = Vec::new();
        let action_last_index = self.get_action_last_index();
        let action_mapper = self.action_mapper();
        for action_id in 1..=action_last_index {
            let action_data = action_mapper.get(action_id);
            if action_data.is_pending() {
                result.push(ActionFullInfo {
                    action_id,
                    action_data,
                    signers: self.get_action_signers(action_id),
                });
            }
        }
        result.into()
    }

    /// Returns `true` (`1`) if the user has signed the action.
    /// Does not check whether or not the user is still a board member and the signature valid.
    #[view]
    fn signed(&self, user: ManagedAddress, action_id: usize) -> bool {
        let user_id = self.user_mapper().get_user_id(&user);
        if user_id == 0 {
            false
        } else {
            let signer_ids = self.action_signer_ids(action_id).get();
            signer_ids.contains(&user_id)
        }
    }

    /// Indicates user rights.
    /// `0` = no rights,
    /// `1` = can propose, but not sign,
    /// `2` = can propose and sign.
    #[view(userRole)]
    fn user_role(&self, user: ManagedAddress) -> UserRole {
        let user_id = self.user_mapper().get_user_id(&user);
        if user_id == 0 {
            UserRole::None
        } else {
            self.get_user_id_to_role(user_id)
        }
    }

    /// Lists all users that can sign actions.
    #[view(getAllBoardMembers)]
    fn get_all_board_members(&self) -> MultiResultVec<ManagedAddress> {
        self.get_all_users_with_role(UserRole::BoardMember)
    }

    /// Lists all proposers that are not board members.
    #[view(getAllProposers)]
    fn get_all_proposers(&self) -> MultiResultVec<ManagedAddress> {
        self.get_all_users_with_role(UserRole::Proposer)
    }

    fn get_all_users_with_role(&self, role: UserRole) -> MultiResultVec<ManagedAddress> {
        let mut result = Vec::new();
        let num_users = self.user_mapper().get_user_count();
        for user_id in 1..=num_users {
            if self.get_user_id_to_role(user_id) == role {
                if let Some(address) = self.user_mapper().get_user_address(user_id) {
                    result.push(address.managed_into());
                }
            }
        }
        result.into()
    }

    /// Used by board members to sign actions.
    #[endpoint]
    fn sign(&self, action_id: usize) -> SCResult<()> {
        require!(
            !self.action_mapper().item_is_empty_unchecked(action_id),
            "action does not exist"
        );

        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.get_user_id_to_role(caller_id);
        require!(caller_role.can_sign(), "only board members can sign");

        self.action_signer_ids(action_id).update(|signer_ids| {
            if !signer_ids.contains(&caller_id) {
                signer_ids.push(caller_id);
            }
        });

        Ok(())
    }

    /// Board members can withdraw their signatures if they no longer desire for the action to be executed.
    /// Actions that are left with no valid signatures can be then deleted to free up storage.
    #[endpoint]
    fn unsign(&self, action_id: usize) -> SCResult<()> {
        require!(
            !self.action_mapper().item_is_empty_unchecked(action_id),
            "action does not exist"
        );

        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.get_user_id_to_role(caller_id);
        require!(caller_role.can_sign(), "only board members can un-sign");

        self.action_signer_ids(action_id).update(|signer_ids| {
            if let Some(signer_pos) = signer_ids
                .iter()
                .position(|&signer_id| signer_id == caller_id)
            {
                // since we don't care about the order,
                // it is ok to call swap_remove, which is O(1)
                signer_ids.swap_remove(signer_pos);
            }
        });

        Ok(())
    }

    /// Clears storage pertaining to an action that is no longer supposed to be executed.
    /// Any signatures that the action received must first be removed, via `unsign`.
    /// Otherwise this endpoint would be prone to abuse.
    #[endpoint(discardAction)]
    fn discard_action(&self, action_id: usize) -> SCResult<()> {
        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.get_user_id_to_role(caller_id);
        require!(
            caller_role.can_discard_action(),
            "only board members and proposers can discard actions"
        );
        require!(
            self.get_action_valid_signer_count(action_id) == 0,
            "cannot discard action with valid signatures"
        );

        self.clear_action(action_id);
        Ok(())
    }
}
