use multiversx_sc::imports::*;

use crate::zombie::Zombie;

#[multiversx_sc::module]
pub trait Storage {
    #[view]
    #[storage_mapper("dnaDigits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[view]
    #[storage_mapper("zombieLastIndex")]
    fn zombie_last_index(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("zombies")]
    fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;

    #[view]
    #[storage_mapper("zombieOwner")]
    fn zombie_owner(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("cryptoKittiesScAddress")]
    fn crypto_kitties_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("cooldownTime")]
    fn cooldown_time(&self) -> SingleValueMapper<u64>;

    #[view]
    #[storage_mapper("ownedZombies")]
    fn owned_zombies(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;

    #[storage_mapper("attackVictoryProbability")]
    fn attack_victory_probability(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("levelUpFee")]
    fn level_up_fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collectedFees")]
    fn collected_fees(&self) -> SingleValueMapper<BigUint>;
}
