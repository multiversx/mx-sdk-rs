#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
mod zombie;
mod zombieattack;
mod zombiefactory;
mod zombiefeeding;
mod zombiehelper;

#[multiversx_sc::contract]
pub trait CryptoZombies:
    zombiefactory::ZombieFactory
    + zombiefeeding::ZombieFeeding
    + storage::Storage
    + zombiehelper::ZombieHelper
    + zombieattack::ZombieAttack
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
