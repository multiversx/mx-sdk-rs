use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UserRole {
    None,
    Proposer,
    BoardMember,
}

impl UserRole {
    pub fn can_propose(&self) -> bool {
        matches!(*self, UserRole::BoardMember | UserRole::Proposer)
    }

    pub fn can_perform_action(&self) -> bool {
        self.can_propose()
    }

    pub fn can_discard_action(&self) -> bool {
        self.can_propose()
    }

    pub fn can_sign(&self) -> bool {
        matches!(*self, UserRole::BoardMember)
    }
}
