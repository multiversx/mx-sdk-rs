use multiversx_sc::derive_imports::*;
use random::{Random, Randomizeable};

const SECONDS_PER_MINUTE: u64 = 60;
const MAX_COOLDOWN: u64 = 60 * 60 * 24 * 7; // 7 days
const MAX_TIREDNESS: u16 = 20;

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

impl Randomizeable for KittyGenes {
    fn get_random(random: &mut Random) -> Self {
        KittyGenes {
            fur_color: Color::get_random(random),
            eye_color: Color::get_random(random),
            meow_power: random.next_u8(),
        }
    }
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

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, Default)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8, // the higher the value, the louder the cat
}

impl KittyGenes {
    pub fn get_as_u64(&self) -> u64 {
        (self.fur_color.as_u64() << 12 | self.eye_color.as_u64()) << 4
            | self.meow_power.to_be() as u64
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, Default)]
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

    pub fn as_u64(&self) -> u64 {
        ((self.r.to_be() as u64) << 4 | self.r.to_be() as u64) << 4 | self.r.to_be() as u64
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
