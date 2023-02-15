multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::zombie::Zombie;

#[multiversx_sc::module]
pub trait Storage {
    #[view]
    #[storage_mapper("dna_digits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[view]
    #[storage_mapper("zombies_count")]
    fn zombies_count(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("zombies")]
    fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;

    #[view]
    #[storage_mapper("zombie_owner")]
    fn zombie_owner(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("crypto_kitties_sc_address")]
    fn crypto_kitties_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("cooldown_time")]
    fn cooldown_time(&self) -> SingleValueMapper<u64>;

    #[view]
    #[storage_mapper("owned_zombies")]
    fn owned_zombies(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;

    #[storage_mapper("attack_victory_probability")]
    fn attack_victory_probability(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("level_up_fee")]
    fn level_up_fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collected_fees")]
    fn collected_fees(&self) -> SingleValueMapper<BigUint>;
}
