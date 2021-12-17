use crate::{api::ManagedTypeApi, types::ManagedBuffer};

const U8_BYTES: usize = 1;
const U16_BYTES: usize = 2;
const U32_BYTES: usize = 4;
const U64_BYTES: usize = 8;

pub struct RandomnessSource<M: ManagedTypeApi> {
    buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> Default for RandomnessSource<M> {
    fn default() -> Self {
        Self {
            buffer: ManagedBuffer::new(),
        }
    }
}

impl<M: ManagedTypeApi> RandomnessSource<M> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_u8(&mut self) -> u8 {
        self.buffer.set_random(U8_BYTES);

        let mut bytes = [0u8; U8_BYTES];
        let _ = self.buffer.load_slice(0, &mut bytes[..]);

        u8::from_be_bytes(bytes)
    }

    pub fn next_u16(&mut self) -> u16 {
        self.buffer.set_random(U16_BYTES);

        let mut bytes = [0u8; U16_BYTES];
        let _ = self.buffer.load_slice(0, &mut bytes[..]);

        u16::from_be_bytes(bytes)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.buffer.set_random(U32_BYTES);

        let mut bytes = [0u8; U32_BYTES];
        let _ = self.buffer.load_slice(0, &mut bytes[..]);

        u32::from_be_bytes(bytes)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.buffer.set_random(U64_BYTES);

        let mut bytes = [0u8; U64_BYTES];
        let _ = self.buffer.load_slice(0, &mut bytes[..]);

        u64::from_be_bytes(bytes)
    }

    pub fn next_bytes(&mut self, len: usize) -> ManagedBuffer<M> {
        self.buffer.set_random(len);

        self.buffer.clone()
    }
}
