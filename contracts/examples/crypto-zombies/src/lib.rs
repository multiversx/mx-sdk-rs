#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
mod zombie;
mod zombie_attack;
mod zombie_factory;
mod zombie_feeding;
mod zombie_helper;

#[multiversx_sc::contract]
pub trait CryptoZombies:
    zombie_factory::ZombieFactory
    + zombie_feeding::ZombieFeeding
    + storage::Storage
    + zombie_helper::ZombieHelper
    + zombie_attack::ZombieAttack
{
    #[init]
    fn init(&self) {
        self.dna_digits().set(16u8);
        self.attack_victory_probability().set(70u8);
        self.level_up_fee().set(BigUint::from(1000000000000000u64));
        self.cooldown_time().set(86400u64);
    }

    #[only_owner]
    #[endpoint]
    fn set_crypto_kitties_sc_address(&self, address: ManagedAddress) {
        self.crypto_kitties_sc_address().set(address);
    }
}
