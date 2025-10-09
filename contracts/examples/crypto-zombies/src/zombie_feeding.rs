use multiversx_sc::imports::*;

use crate::{kitty_obj::Kitty, kitty_ownership_proxy, storage, zombie_factory, zombie_helper};

#[multiversx_sc::module]
pub trait ZombieFeeding:
    storage::Storage + zombie_factory::ZombieFactory + zombie_helper::ZombieHelper
{
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64, species: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        self.check_zombie_belongs_to_caller(zombie_id, &caller);
        require!(self.is_ready(zombie_id), "Zombie is not ready");
        let my_zombie = self.zombies(&zombie_id).get();
        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        let verified_target_dna = target_dna % max_dna_value;
        let mut new_dna = (my_zombie.dna + verified_target_dna) / 2;
        if species == b"kitty" {
            new_dna = new_dna - new_dna % 100 + 99
        }
        self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);
        self.trigger_cooldown(zombie_id);
    }

    fn trigger_cooldown(&self, zombie_id: usize) {
        let cooldown_time = self.cooldown_time().get();
        self.zombies(&zombie_id).update(|my_zombie| {
            my_zombie.ready_time = self.blockchain().get_block_timestamp() + cooldown_time
        });
    }

    #[view]
    fn is_ready(&self, zombie_id: usize) -> bool {
        let my_zombie = self.zombies(&zombie_id).get();
        my_zombie.ready_time <= self.blockchain().get_block_timestamp()
    }

    #[callback]
    fn get_kitty_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<Kitty>,
        zombie_id: usize,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(kitty) => {
                let kitty_dna = kitty.genes.get_as_u64();
                self.feed_and_multiply(zombie_id, kitty_dna, ManagedBuffer::from(b"kitty"));
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }

    #[endpoint]
    fn feed_on_kitty(&self, zombie_id: usize, kitty_id: u32) {
        let crypto_kitties_sc_address = self.crypto_kitties_sc_address().get();
        self.tx()
            .to(&crypto_kitties_sc_address)
            .typed(kitty_ownership_proxy::KittyOwnershipProxy)
            .get_kitty_by_id_endpoint(kitty_id)
            .callback(self.callbacks().get_kitty_callback(zombie_id))
            .async_call_and_exit();
    }
}
