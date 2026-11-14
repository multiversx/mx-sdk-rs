#![no_std]

use multiversx_sc::imports::*;

pub mod validator_rewards_feature_proxy;

/// Contract that only tests the validator rewards feature,
/// i.e. the `getCumulatedValidatorRewards` blockchain API.
///
/// It behaves like a minimal reward-splitting contract: the protocol (or a test) credits
/// validator rewards to the contract, and the owner can claim whatever has accumulated
/// since the last claim.
///
/// It also holds an unrelated storage value, to check that reward bookkeeping done outside
/// the contract leaves the contract's own storage alone.
#[multiversx_sc::contract]
pub trait ValidatorRewardsFeature {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Total validator rewards ever credited to this contract, as reported by the protocol.
    #[view(getCumulatedRewards)]
    fn get_cumulated_rewards(&self) -> BigUint {
        self.blockchain().get_cumulated_validator_rewards()
    }

    /// Part of the cumulated rewards that was already claimed.
    #[view(getClaimedRewards)]
    #[storage_mapper("claimedRewards")]
    fn claimed_rewards(&self) -> SingleValueMapper<BigUint>;

    /// Cumulated rewards that were not claimed yet.
    #[view(getUnclaimedRewards)]
    fn get_unclaimed_rewards(&self) -> BigUint {
        self.get_cumulated_rewards() - self.claimed_rewards().get()
    }

    /// Sends all unclaimed rewards to the owner. Returns the amount sent.
    #[only_owner]
    #[endpoint(claimRewards)]
    fn claim_rewards(&self) -> BigUint {
        let unclaimed = self.get_unclaimed_rewards();
        require!(unclaimed > 0u32, "no rewards to claim");

        self.claimed_rewards()
            .update(|claimed| *claimed += &unclaimed);
        self.tx().to(ToCaller).egld(&unclaimed).transfer();

        unclaimed
    }

    /// Unrelated storage, written by the owner of the test.
    #[endpoint(storeNote)]
    fn store_note(&self, note: ManagedBuffer) {
        self.note().set(note);
    }

    #[view(getNote)]
    #[storage_mapper("note")]
    fn note(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getCodeMetadata)]
    fn get_code_metadata(&self, address: ManagedAddress) -> CodeMetadata {
        self.blockchain().get_code_metadata(&address)
    }
}
