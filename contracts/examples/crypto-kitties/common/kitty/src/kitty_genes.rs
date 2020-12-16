use elrond_wasm::elrond_codec::*;

use super::color::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
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
