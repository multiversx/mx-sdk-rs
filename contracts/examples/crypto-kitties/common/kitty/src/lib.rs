#![no_std]

multiversx_sc::derive_imports!();

const SECONDS_PER_MINUTE: u64 = 60;
const MAX_COOLDOWN: u64 = 60 * 60 * 24 * 7; // 7 days
const MAX_TIREDNESS: u16 = 20;

pub mod color;
pub mod kitty_genes;

use color::*;
use kitty_genes::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Kitty {
    pub genes: KittyGenes,
    pub birth_time: u64,   // timestamp
    pub cooldown_end: u64, // timestamp, used for pregnancy timer and siring cooldown
    pub matron_id: u32,
    pub sire_id: u32,
    pub siring_with_id: u32, // for pregnant cats, 0 otherwise
    pub nr_children: u16,    // cooldown period increases exponentially with every breeding/siring
    pub generation: u16,     // max(sire_gen, matron_gen) + 1. Generation also influences cooldown.
}

impl Kitty {
    pub fn new(
        genes: KittyGenes,
        birth_time: u64,
        matron_id: u32,
        sire_id: u32,
        generation: u16,
    ) -> Self {
        Kitty {
            genes,
            birth_time,
            cooldown_end: 0,
            matron_id,
            sire_id,
            siring_with_id: 0,
            nr_children: 0,
            generation,
        }
    }
}

impl Kitty {
    pub fn get_next_cooldown_time(&self) -> u64 {
        let tiredness = self.nr_children + self.generation / 2;
        if tiredness > MAX_TIREDNESS {
            return MAX_COOLDOWN;
        }

        let cooldown = SECONDS_PER_MINUTE << tiredness; // 2^(tiredness) minutes
        if cooldown > MAX_COOLDOWN {
            MAX_COOLDOWN
        } else {
            cooldown
        }
    }

    pub fn get_fur_color(&self) -> Color {
        self.genes.fur_color.clone()
    }

    pub fn get_eye_color(&self) -> Color {
        self.genes.eye_color.clone()
    }

    pub fn get_meow_power(&self) -> u8 {
        self.genes.meow_power
    }

    pub fn is_pregnant(&self) -> bool {
        self.siring_with_id != 0
    }
}

// The default Kitty, which is not a valid kitty. Used for Kitty with ID 0
impl Default for Kitty {
    fn default() -> Self {
        Kitty {
            genes: KittyGenes::default(),
            birth_time: 0,
            cooldown_end: u64::MAX,
            matron_id: 0,
            sire_id: 0,
            siring_with_id: 0,
            nr_children: 0,
            generation: 0,
        }
    }
}
