multiversx_sc::derive_imports!();

use random::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, TypeAbi, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // ratios are integers, 0 < ratio < 100, ratioFirst + ratioSecond = 100
    // checks should be done in the caller
    #[must_use]
    pub fn mix_with(&self, other_color: &Color, ratio_first: u8, ratio_second: u8) -> Color {
        let r = ((self.r as u16 * ratio_first as u16 + other_color.r as u16 * ratio_second as u16)
            / 100) as u8;

        let g = ((self.g as u16 * ratio_first as u16 + other_color.g as u16 * ratio_second as u16)
            / 100) as u8;

        let b = ((self.b as u16 * ratio_first as u16 + other_color.b as u16 * ratio_second as u16)
            / 100) as u8;

        Color { r, g, b }
    }
}

impl Randomizeable for Color {
    fn get_random(random: &mut Random) -> Self {
        Color {
            r: random.next_u8(),
            g: random.next_u8(),
            b: random.next_u8(),
        }
    }
}
