multiversx_sc::imports!();
multiversx_sc::derive_imports!();

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

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8, // the higher the value, the louder the cat
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[multiversx_sc::proxy]
pub trait CryptoKitties {
    #[endpoint]
    fn get_kitty_by_id_endpoint(&self, kitty_id: u32) -> Kitty;
}

impl KittyGenes {
    pub fn get_as_u64(&self) -> u64 {
        (self.fur_color.as_u64() << 12 | self.eye_color.as_u64()) << 4
            | self.meow_power.to_be() as u64
    }
}

impl Color {
    pub fn as_u64(&self) -> u64 {
        ((self.r.to_be() as u64) << 4 | self.r.to_be() as u64) << 4 | self.r.to_be() as u64
    }
}
