#![no_std]

const SEED_SIZE: usize = 48;

pub struct Random {
    data: [u8; SEED_SIZE],
    current_index: usize,
}

// usually, types should create their own `random` instance,
// but because standalone types can't create a random seed
// (due to no access to blockhain functions),
// the method will use a provided `random` instance
pub trait Randomizeable {
    fn get_random(random: &mut Random) -> Self;
}

impl Random {
    /// block random seed + salt creates a stronger randomness source
    pub fn new(seed: [u8; SEED_SIZE], salt: &[u8]) -> Self {
        let mut rand_source = [0u8; SEED_SIZE];
        let salt_len = salt.len();

        for i in 0..SEED_SIZE {
            rand_source[i] =
                (((seed[i] as u16) + (salt[i % salt_len] as u16)) % (u8::MAX as u16 + 1u16)) as u8;
        }

        Random {
            data: rand_source,
            current_index: 0,
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let first_byte = self.next_u8() as u32;
        let second_byte = self.next_u8() as u32;
        let third_byte = self.next_u8() as u32;
        let fourth_byte = self.next_u8() as u32;

        first_byte | second_byte | third_byte | fourth_byte
    }

    pub fn next_u8(&mut self) -> u8 {
        let val = self.data[self.current_index];

        self.current_index += 1;

        if self.current_index == SEED_SIZE {
            self.shuffle();
            self.current_index = 0;
        }

        val
    }

    // Fake shuffle. Just add numbers to one another, accounting for overflow.
    fn shuffle(&mut self) {
        for i in 0..(self.data.len() - 1) {
            let res: u16 = (self.data[i] as u16) + (self.data[i + 1] as u16) + 1;

            self.data[i] = (res % (u8::MAX as u16 + 1)) as u8;
        }
    }
}
