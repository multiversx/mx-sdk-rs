#![no_std]

use elrond_wasm::elrond_codec::*;

const SECONDS_PER_MINUTE: u64 = 60;
const MAX_COOLDOWN: u64 = 60 * 60 * 24 * 7; // 7 days
const MAX_TIREDNESS: u16 = 20;

// TO DO: fixing encoding/decoding

// TO DO: move Color and KittyGenes to separate files each

pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8
}
pub struct KittyGenes {
	pub fur_color: Color,
	pub eye_color: Color,
	pub meow_power: u8 // the higher the value, the louder the cat
}

pub struct Kitty {
    pub genes: KittyGenes, 
    pub birth_time: u64, // timestamp
    pub cooldown_end: u64, // timestamp, used for pregnancy timer and siring cooldown
    pub matron_id: u32,
    pub sire_id: u32,
    pub siring_with_id: u32, // for pregnant cats, 0 otherwise
    pub nr_children: u16, // cooldown period increases exponentially with every breeding/siring
    pub generation: u16 // max(sire_gen, matron_gen) + 1. Generation also influences cooldown.
}

impl Color {
	// ratios are integers, 0 < ratio < 100, ratioFirst + ratioSecond = 100
	// checks should be done in the caller
	pub fn mix_with(&self, other_color: &Color, ratio_first: u8, ratio_second: u8) -> Color {
		let mut result = Color::default();

		result.r = ((self.r as u16 * ratio_first as u16 + 
			other_color.r as u16 * ratio_second as u16) / 100) as u8;
		
		result.g = ((self.g as u16 * ratio_first as u16 + 
			other_color.g as u16 * ratio_second as u16) / 100) as u8;

		result.b = ((self.b as u16 * ratio_first as u16 + 
			other_color.b as u16 * ratio_second as u16) / 100) as u8;

		result
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
            return MAX_COOLDOWN;
        }
        else {
            return cooldown;
        }
	}

	pub fn get_fur_color(&self) -> Color {
		self.genes.fur_color
	}
	
	pub fn get_eye_color(&self) -> Color {
		self.genes.eye_color
	}

	pub fn get_meow_power(&self) -> u8 {
		self.genes.meow_power
	}
}

// Default trait impl for the types

impl Default for Color {
	fn default() -> Self {
		Color {
			r: 0,
			g: 0,
			b: 0
		}
	}
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
			generation: 0
		}
	}
}

impl NestedEncode for Kitty {
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.genes.dep_encode(dest)?;
		self.birth_time.dep_encode(dest)?;
		self.cooldown_end.dep_encode(dest)?;
		self.matron_id.dep_encode(dest)?;
		self.sire_id.dep_encode(dest)?;
		self.siring_with_id.dep_encode(dest)?;
		self.nr_children.dep_encode(dest)?;
		self.generation.dep_encode(dest)?;

		Ok(())
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
        self.genes.dep_encode_or_exit(dest, c.clone(), exit);
		self.birth_time.dep_encode_or_exit(dest, c.clone(), exit);
		self.cooldown_end.dep_encode_or_exit(dest, c.clone(), exit);
		self.matron_id.dep_encode_or_exit(dest, c.clone(), exit);
		self.sire_id.dep_encode_or_exit(dest, c.clone(), exit);
		self.siring_with_id.dep_encode_or_exit(dest, c.clone(), exit);
		self.nr_children.dep_encode_or_exit(dest, c.clone(), exit);
		self.generation.dep_encode_or_exit(dest, c.clone(), exit);
	}
}

impl TopEncode for Kitty {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		top_encode_from_nested(self, output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		top_encode_from_nested_or_exit(self, output, c, exit);
	}
}

impl NestedDecode for Kitty {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Kitty {
            genes: KittyGenes::dep_decode(input)?,
            birth_time: u64::dep_decode(input)?,
            cooldown_end: u64::dep_decode(input)?,
            matron_id: u32::dep_decode(input)?,
            sire_id: u32::dep_decode(input)?,
            siring_with_id: u32::dep_decode(input)?,
            nr_children: u16::dep_decode(input)?,
            generation: u16::dep_decode(input)?,
		})
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Kitty {
            genes: KittyGenes::dep_decode_or_exit(input, c.clone(), exit),
            birth_time: u64::dep_decode_or_exit(input, c.clone(), exit),
            cooldown_end: u64::dep_decode_or_exit(input, c.clone(), exit),
            matron_id: u32::dep_decode_or_exit(input, c.clone(), exit),
            sire_id: u32::dep_decode_or_exit(input, c.clone(), exit),
            siring_with_id: u32::dep_decode_or_exit(input, c.clone(), exit),
            nr_children: u16::dep_decode_or_exit(input, c.clone(), exit),
            generation: u16::dep_decode_or_exit(input, c.clone(), exit),
		}
	}
}

impl TopDecode for Kitty {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		top_decode_from_nested(input)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		top_decode_from_nested_or_exit(input, c, exit)
	}
}
