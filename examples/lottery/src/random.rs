

const SEED_SIZE: usize = 48;

pub struct Random {
    data: [u8; SEED_SIZE],
    current_index: usize
}

impl Random {
    pub fn new(seed: [u8; SEED_SIZE]) -> Self {
        Random { data: seed, current_index: 0 }
    }

    pub fn next(&mut self) -> u32 {
        let first_byte = (self.data[self.current_index] as u32) << 24;
        let second_byte = (self.data[self.current_index + 1] as u32) << 16;
        let third_byte = (self.data[self.current_index + 2] as u32) << 8;
        let fourth_byte = self.data[self.current_index + 3] as u32;

        self.current_index += 4;

        if self.current_index == SEED_SIZE {
            self.shuffle();
            self.current_index = 0;
        }

        return first_byte | second_byte | third_byte | fourth_byte;
    }

    // Fake shuffle. Just add numbers to one another, ignoring overflow.
    fn shuffle(&mut self) {
        for i in 0..(self.data.len() - 1) {
            self.data[i] += self.data[i + 1] + 1;
        }
    }
}
