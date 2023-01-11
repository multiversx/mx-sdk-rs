use crate::{action::Action, user_role::UserRole};

multiversx_sc::imports!();

/// Contains all events that can be emitted by the contract.
#[multiversx_sc::module]
pub trait MultisigStateModule {
    /// Minimum number of signatures needed to perform any action.
    #[view(getQuorum)]
    #[storage_mapper("quorum")]
    fn quorum(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("user")]
    fn user_mapper(&self) -> UserMapper;

    #[storage_mapper("user_role")]
    fn user_id_to_role(&self, user_id: usize) -> SingleValueMapper<UserRole>;

    fn get_caller_id_and_role(&self) -> (usize, UserRole) {
        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.user_id_to_role(caller_id).get();
        (caller_id, caller_role)
    }

    /// Denormalized board member count.
    /// It is kept in sync with the user list by the contract.
    #[view(getNumBoardMembers)]
    #[storage_mapper("num_board_members")]
    fn num_board_members(&self) -> SingleValueMapper<usize>;

    /// Denormalized proposer count.
    /// It is kept in sync with the user list by the contract.
    #[view(getNumProposers)]
    #[storage_mapper("num_proposers")]
    fn num_proposers(&self) -> SingleValueMapper<usize>;

    fn add_multiple_board_members(&self, new_board_members: ManagedVec<ManagedAddress>) -> usize {
        let mut duplicates = false;
        self.user_mapper().get_or_create_users(
            new_board_members.into_iter(),
            |user_id, new_user| {
                if !new_user {
                    duplicates = true;
                }
                self.user_id_to_role(user_id).set(UserRole::BoardMember);
            },
        );
        require!(!duplicates, "duplicate board member");

        let num_board_members_mapper = self.num_board_members();
        let new_num_board_members = num_board_members_mapper.get() + new_board_members.len();
        num_board_members_mapper.set(new_num_board_members);

        new_num_board_members
    }

    #[storage_mapper("action_data")]
    fn action_mapper(&self) -> VecMapper<Action<Self::Api>>;

    /// The index of the last proposed action.
    /// 0 means that no action was ever proposed yet.
    #[view(getActionLastIndex)]
    fn get_action_last_index(&self) -> usize {
        self.action_mapper().len()
    }

    /// Serialized action data of an action with index.
    #[label("multisig-external-view")]
    #[view(getActionData)]
    fn get_action_data(&self, action_id: usize) -> Action<Self::Api> {
        self.action_mapper().get(action_id)
    }

    #[storage_mapper("action_signer_ids")]
    fn action_signer_ids(&self, action_id: usize) -> UnorderedSetMapper<usize>;

    /// Gets addresses of all users who signed an action.
    /// Does not check if those users are still board members or not,
    /// so the result may contain invalid signers.
    #[label("multisig-external-view")]
    #[view(getActionSigners)]
    fn get_action_signers(&self, action_id: usize) -> ManagedVec<ManagedAddress> {
        let signer_ids = self.action_signer_ids(action_id);
        let mut signers = ManagedVec::new();
        for signer_id in signer_ids.iter() {
            signers.push(self.user_mapper().get_user_address_unchecked(signer_id));
        }
        signers
    }

    /// Gets addresses of all users who signed an action and are still board members.
    /// All these signatures are currently valid.
    #[label("multisig-external-view")]
    #[view(getActionSignerCount)]
    fn get_action_signer_count(&self, action_id: usize) -> usize {
        self.action_signer_ids(action_id).len()
    }

    /// It is possible for board members to lose their role.
    /// They are not automatically removed from all actions when doing so,
    /// therefore the contract needs to re-check every time when actions are performed.
    /// This function is used to validate the signers before performing an action.
    /// It also makes it easy to check before performing an action.
    #[label("multisig-external-view")]
    #[view(getActionValidSignerCount)]
    fn get_action_valid_signer_count(&self, action_id: usize) -> usize {
        let signer_ids = self.action_signer_ids(action_id);
        signer_ids
            .iter()
            .filter(|signer_id| {
                let signer_role = self.user_id_to_role(*signer_id).get();
                signer_role.can_sign()
            })
            .count()
    }
}
