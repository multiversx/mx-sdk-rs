multiversx_sc::derive_imports!();

use super::color::*;
use random::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, TypeAbi, Default)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8, // the higher the value, the louder the cat
}

impl Randomizeable for KittyGenes {
    fn get_random(random: &mut Random) -> Self {
        KittyGenes {
            fur_color: Color::get_random(random),
            eye_color: Color::get_random(random),
            meow_power: random.next_u8(),
        }
    }
}
