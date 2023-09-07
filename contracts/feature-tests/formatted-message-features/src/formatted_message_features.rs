#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait FormattedMessageFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn static_message(&self) {
        sc_panic!("Static error");
    }

    #[endpoint]
    fn dynamic_message(&self, bytes: ManagedBuffer) {
        sc_panic!("Got this buffer: {}. I don't like it, ERROR!", &bytes);
    }

    #[endpoint]
    fn dynamic_message_hex(&self, bytes: ManagedBuffer) {
        sc_panic!("Got this buffer: {:x}. I don't like it, ERROR!", bytes);
    }

    #[payable("*")]
    #[endpoint]
    fn dynamic_message_multiple(&self) {
        let (token_id, nonce, amount) = self.call_value().egld_or_single_esdt().into_tuple();
        sc_panic!(
            "Got token {}, with nonce {}, amount {}. I prefer EGLD. ERROR!",
            &&token_id, // references are accepted
            nonce,
            &amount
        );
    }

    #[payable("*")]
    #[endpoint]
    fn dynamic_message_ascii(&self) {
        let (token_id, nonce, amount) = self.call_value().egld_or_single_esdt().into_tuple();
        sc_panic!(
            "Got token {}, with nonce {}, amount {}. I prefer EGLD. ERROR!",
            token_id,
            nonce,
            amount, // trailing comma allowed
        );
    }

    #[endpoint]
    fn decode_error_message(&self) {
        sc_panic!(DecodeError::UNSUPPORTED_OPERATION,);
    }

    /// TODO: figure out a way to test this.
    #[endpoint]
    fn print_message(&self, x: i32) {
        sc_print!("Printing x: {}", x,);
    }

    #[endpoint]
    fn print_message_hex(&self, x: i32) {
        sc_print!("Printing x: {:x}", x,);
    }

    #[endpoint]
    fn print_message_binary(&self, x: u32) {
        sc_print!("Printing x: {:b}", x);
    }

    #[endpoint]
    fn print_message_codec(&self, x: i32) {
        sc_print!("Printing x: {:c}", x);
    }

    #[endpoint]
    fn format_message_one_part(&self) -> ManagedBuffer {
        let message = sc_format!("Test");
        message
    }

    #[endpoint]
    fn format_message_multiple_parts(&self, x: i32) -> ManagedBuffer {
        let message = sc_format!("Hello {} world", x);
        message
    }

    #[endpoint]
    fn format_message_big_int(&self, x: BigInt) -> ManagedBuffer {
        let message = sc_format!("BigInt: {}", x);
        message
    }

    #[endpoint]
    fn format_message_i64(&self, x: i64) -> ManagedBuffer {
        let message = sc_format!("i64: {}", x);
        message
    }

    #[endpoint]
    fn format_message_managed_buffer(&self, x: ManagedBuffer) -> ManagedBuffer {
        let message = sc_format!("ManagedBuffer: {}", x);
        message
    }

    #[endpoint]
    fn format_message_managed_buffer_hex(&self, x: ManagedBuffer) -> ManagedBuffer {
        let message = sc_format!("ManagedBuffer hex: {:x}", x);
        message
    }
}
