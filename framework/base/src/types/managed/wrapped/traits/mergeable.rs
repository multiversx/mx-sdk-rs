use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{EsdtTokenPayment, ManagedVec, ManagedVecItem},
};

pub static CANNOT_MERGE_ERR_MSG: &[u8] = b"Cannot merge";

/// Used for types that can be merged locally.
pub trait Mergeable<M: ManagedTypeApi> {
    fn error_if_not_mergeable(&self, other: &Self) {
        if !self.can_merge_with(other) {
            throw_not_mergeable_error::<M>();
        }
    }

    fn can_merge_with(&self, other: &Self) -> bool;

    fn merge_with(&mut self, other: Self);

    fn merge_with_multiple(&mut self, others: ManagedVec<M, Self>)
    where
        Self: Sized + ManagedVecItem,
    {
        for item in others {
            self.merge_with(item);
        }
    }
}

/// Used when merging is done through an external SC call.
/// Generally, these only need to have the same token ID, with different nonces.
pub trait ExternallyMergeable<M: ManagedTypeApi> {
    fn error_if_not_externally_mergeable(&self, other: &Self) {
        if !self.can_be_merged_externally_with(other) {
            throw_not_mergeable_error::<M>();
        }
    }

    fn can_be_merged_externally_with(&self, other: &Self) -> bool;
}

pub fn throw_not_mergeable_error<M: ManagedTypeApi>() -> ! {
    M::error_api_impl().signal_error(CANNOT_MERGE_ERR_MSG);
}

impl<M: ManagedTypeApi> Mergeable<M> for EsdtTokenPayment<M> {
    fn can_merge_with(&self, other: &Self) -> bool {
        let same_token_id = self.token_identifier == other.token_identifier;
        let same_token_nonce = self.token_nonce == other.token_nonce;

        same_token_id && same_token_nonce
    }

    fn merge_with(&mut self, other: Self) {
        self.error_if_not_mergeable(&other);

        self.amount += other.amount;
    }
}

impl<M: ManagedTypeApi> ExternallyMergeable<M> for EsdtTokenPayment<M> {
    fn can_be_merged_externally_with(&self, other: &Self) -> bool {
        self.token_identifier == other.token_identifier
    }
}
