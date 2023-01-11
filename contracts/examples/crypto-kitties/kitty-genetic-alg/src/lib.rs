#![no_std]

multiversx_sc::imports!();

use kitty::{kitty_genes::*, Kitty};
use random::Random;

#[multiversx_sc::contract]
pub trait KittyGeneticAlg {
    #[init]
    fn init(&self) {}

    // endpoints

    #[endpoint(generateKittyGenes)]
    fn generate_kitty_genes(&self, matron: Kitty, sire: Kitty) -> KittyGenes {
        let mut random = Random::new(
            self.blockchain().get_block_random_seed(),
            self.blockchain().get_tx_hash(),
        );

        let fur_color_percentage = 1 + random.next_u8() % 99; // val in [1, 100)
        let matron_fur_color = matron.get_fur_color();
        let sire_fur_color = sire.get_fur_color();
        let kitty_fur_color = matron_fur_color.mix_with(
            &sire_fur_color,
            fur_color_percentage,
            100 - fur_color_percentage,
        );

        let eye_color_percentage = 1 + random.next_u8() % 99; // val in [1, 100)
        let matron_eye_color = matron.get_eye_color();
        let sire_eye_color = sire.get_eye_color();
        let kitty_eye_color = matron_eye_color.mix_with(
            &sire_eye_color,
            eye_color_percentage,
            100 - eye_color_percentage,
        );

        let kitty_meow_power = matron.get_meow_power() / 2 + sire.get_meow_power() / 2;

        KittyGenes {
            fur_color: kitty_fur_color,
            eye_color: kitty_eye_color,
            meow_power: kitty_meow_power,
        }
    }
}
