multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie::Zombie};

#[multiversx_sc::module]
pub trait ZombieFactory: storage::Storage {
    fn create_zombie(&self, owner: ManagedAddress, name: ManagedBuffer, dna: u64) {
        self.zombies_count().update(|id| {
            self.new_zombie_event(*id, &name, dna);
            self.zombies(id).set(Zombie {
                name,
                dna,
                level: 1u16,
                ready_time: self.blockchain().get_block_timestamp(),
                win_count: 0usize,
                loss_count: 0usize,
            });
            self.owned_zombies(&owner).insert(*id);
            self.zombie_owner(id).set(owner);
            *id += 1;
        });
    }

    #[view]
    fn generate_random_dna(&self) -> u64 {
        let mut rand_source = RandomnessSource::new();
        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        rand_source.next_u64_in_range(0u64, max_dna_value)
    }

    #[endpoint]
    fn create_random_zombie(&self, name: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        require!(
            self.owned_zombies(&caller).is_empty(),
            "You already own a zombie"
        );
        let rand_dna = self.generate_random_dna();
        self.create_zombie(caller, name, rand_dna);
    }

    #[event("new_zombie_event")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        name: &ManagedBuffer,
        #[indexed] dna: u64,
    );
}
