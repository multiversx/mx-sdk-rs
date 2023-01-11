#![no_std]

pub mod action;
pub mod multisig_events;
pub mod multisig_perform;
pub mod multisig_propose;
pub mod multisig_state;
pub mod user_role;

use action::ActionFullInfo;
use user_role::UserRole;

multiversx_sc::imports!();

/// Multi-signature smart contract implementation.
/// Acts like a wallet that needs multiple signers for any action performed.
/// See the readme file for more detailed documentation.
#[multiversx_sc::contract]
pub trait Multisig:
    multisig_state::MultisigStateModule
    + multisig_propose::MultisigProposeModule
    + multisig_perform::MultisigPerformModule
    + multisig_events::MultisigEventsModule
    + multiversx_sc_modules::dns::DnsModule
{
    #[init]
    fn init(&self, quorum: usize, board: MultiValueEncoded<ManagedAddress>) {
        let board_vec = board.to_vec();
        let new_num_board_members = self.add_multiple_board_members(board_vec);

        let num_proposers = self.num_proposers().get();
        require!(
            new_num_board_members + num_proposers > 0,
            "board cannot be empty on init, no-one would be able to propose"
        );

        require!(
            quorum <= new_num_board_members,
            "quorum cannot exceed board size"
        );
        self.quorum().set(quorum);
    }

    /// Allows the contract to receive funds even if it is marked as unpayable in the protocol.
    #[payable("*")]
    #[endpoint]
    fn deposit(&self) {}

    /// Iterates through all actions and retrieves those that are still pending.
    /// Serialized full action data:
    /// - the action id
    /// - the serialized action data
    /// - (number of signers followed by) list of signer addresses.
    #[label("multisig-external-view")]
    #[view(getPendingActionFullInfo)]
    fn get_pending_action_full_info(&self) -> MultiValueEncoded<ActionFullInfo<Self::Api>> {
        let mut result = MultiValueEncoded::new();
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
        result
    }

    /// Returns `true` (`1`) if the user has signed the action.
    /// Does not check whether or not the user is still a board member and the signature valid.
    #[view]
    fn signed(&self, user: ManagedAddress, action_id: usize) -> bool {
        let user_id = self.user_mapper().get_user_id(&user);
        if user_id == 0 {
            false
        } else {
            self.action_signer_ids(action_id).contains(&user_id)
        }
    }

    /// Indicates user rights.
    /// `0` = no rights,
    /// `1` = can propose, but not sign,
    /// `2` = can propose and sign.
    #[label("multisig-external-view")]
    #[view(userRole)]
    fn user_role(&self, user: ManagedAddress) -> UserRole {
        let user_id = self.user_mapper().get_user_id(&user);
        if user_id == 0 {
            UserRole::None
        } else {
            self.user_id_to_role(user_id).get()
        }
    }

    /// Lists all users that can sign actions.
    #[label("multisig-external-view")]
    #[view(getAllBoardMembers)]
    fn get_all_board_members(&self) -> MultiValueEncoded<ManagedAddress> {
        self.get_all_users_with_role(UserRole::BoardMember)
    }

    /// Lists all proposers that are not board members.
    #[label("multisig-external-view")]
    #[view(getAllProposers)]
    fn get_all_proposers(&self) -> MultiValueEncoded<ManagedAddress> {
        self.get_all_users_with_role(UserRole::Proposer)
    }

    fn get_all_users_with_role(&self, role: UserRole) -> MultiValueEncoded<ManagedAddress> {
        let mut result = MultiValueEncoded::new();
        let num_users = self.user_mapper().get_user_count();
        for user_id in 1..=num_users {
            if self.user_id_to_role(user_id).get() == role {
                if let Some(address) = self.user_mapper().get_user_address(user_id) {
                    result.push(address);
                }
            }
        }
        result
    }

    /// Used by board members to sign actions.
    #[endpoint]
    fn sign(&self, action_id: usize) {
        require!(
            !self.action_mapper().item_is_empty_unchecked(action_id),
            "action does not exist"
        );

        let (caller_id, caller_role) = self.get_caller_id_and_role();
        require!(caller_role.can_sign(), "only board members can sign");

        if !self.action_signer_ids(action_id).contains(&caller_id) {
            self.action_signer_ids(action_id).insert(caller_id);
        }
    }

    /// Board members can withdraw their signatures if they no longer desire for the action to be executed.
    /// Actions that are left with no valid signatures can be then deleted to free up storage.
    #[endpoint]
    fn unsign(&self, action_id: usize) {
        require!(
            !self.action_mapper().item_is_empty_unchecked(action_id),
            "action does not exist"
        );

        let (caller_id, caller_role) = self.get_caller_id_and_role();
        require!(caller_role.can_sign(), "only board members can un-sign");

        self.action_signer_ids(action_id).swap_remove(&caller_id);
    }

    /// Clears storage pertaining to an action that is no longer supposed to be executed.
    /// Any signatures that the action received must first be removed, via `unsign`.
    /// Otherwise this endpoint would be prone to abuse.
    #[endpoint(discardAction)]
    fn discard_action(&self, action_id: usize) {
        let (_, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_discard_action(),
            "only board members and proposers can discard actions"
        );
        require!(
            self.get_action_valid_signer_count(action_id) == 0,
            "cannot discard action with valid signatures"
        );

        self.clear_action(action_id);
    }
}
