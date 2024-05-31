use multiversx_sc::derive_imports::*;
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
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

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8, // the higher the value, the louder the cat
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl KittyGenes {
    pub fn get_as_u64(&self) -> u64 {
        (self.fur_color.as_u64() << 24 | self.eye_color.as_u64()) << 8
            | self.meow_power.to_be() as u64
    }
}

impl Color {
    pub fn as_u64(&self) -> u64 {
        ((self.r.to_be() as u64) << 8 | self.r.to_be() as u64) << 8 | self.r.to_be() as u64
    }
}
