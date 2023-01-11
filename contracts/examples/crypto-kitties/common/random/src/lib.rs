#![no_std]

use multiversx_sc::{api::ManagedTypeApi, types::ManagedByteArray};

const SEED_SIZE: usize = 48;
const SALT_SIZE: usize = 32;
const BYTE_MAX: u16 = u8::MAX as u16 + 1u16;

static mut SEED_STATIC_BUFFER: [u8; SEED_SIZE] = [0u8; SEED_SIZE];
static mut SALT_STATIC_BUFFER: [u8; SALT_SIZE] = [0u8; SALT_SIZE];

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
    pub fn new<M: ManagedTypeApi>(
        seed: ManagedByteArray<M, SEED_SIZE>,
        salt: ManagedByteArray<M, SALT_SIZE>,
    ) -> Self {
        unsafe {
            // it's more efficient to load all the data in static buffers
            // instead of reading byte by byte
            SEED_STATIC_BUFFER.copy_from_slice(&seed.to_byte_array());
            SALT_STATIC_BUFFER.copy_from_slice(&salt.to_byte_array());

            let mut rand_source = [0u8; SEED_SIZE];

            for i in 0..SEED_SIZE {
                let seed_byte = SEED_STATIC_BUFFER[i];
                let salt_byte = SALT_STATIC_BUFFER[i % SALT_SIZE];
                let sum = (seed_byte as u16) + (salt_byte as u16);

                rand_source[i] = (sum % BYTE_MAX) as u8;
            }

            Random {
                data: rand_source,
                current_index: 0,
            }
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let first_byte = self.next_u8() as u32;
        let second_byte = self.next_u8() as u32;
        let third_byte = self.next_u8() as u32;
        let fourth_byte = self.next_u8() as u32;

        // TODO: Fix, this only generates in u8 range
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
