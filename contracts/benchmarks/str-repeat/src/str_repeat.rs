#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait StrRepeat {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn repeat(&self, string: &[u8], num_repeats: usize) {
        let mut byte_array = self.byte_array().get();
        for _ in 0..num_repeats {
            byte_array.extend_from_slice(string);
        }

        self.byte_array().set(byte_array);
    }

    #[view(getByteArrayLength)]
    fn get_byte_array_length(&self) -> usize {
        self.byte_array().raw_byte_length()
    }

    #[view(getByteArray)]
    #[storage_mapper("byteArray")]
    fn byte_array(&self) -> SingleValueMapper<Vec<u8>>;
}
