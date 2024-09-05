use multiversx_sc::derive_imports::*;

use super::color::*;
use random::*;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, Default)]
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

impl KittyGenes {
    pub fn get_as_u64(&self) -> u64 {
        (self.fur_color.as_u64() << 12 | self.eye_color.as_u64()) << 4
            | self.meow_power.to_be() as u64
    }
}
