use elrond_wasm::elrond_codec::*;

use super::color::*;
use random::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone)]
pub struct KittyGenes {
	pub fur_color: Color,
	pub eye_color: Color,
	pub meow_power: u8 // the higher the value, the louder the cat
}

impl Default for KittyGenes {
	fn default() -> Self {
		KittyGenes {
			fur_color: Color::default(),
			eye_color: Color::default(),
			meow_power: 0
		}
	}
}

impl Randomizeable for KittyGenes {
    fn get_random(random: &mut Random) -> Self {
        KittyGenes {
			fur_color: Color::get_random(random),
			eye_color: Color::get_random(random),
			meow_power: random.next_u8()
		}
    }
}
