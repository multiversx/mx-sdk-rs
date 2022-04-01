#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait FormattedMessageFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn static_message(&self) {
        sc_panic!("Static error");
    }

    #[endpoint]
    fn dynamic_message(&self, bytes: ManagedBuffer) {
        sc_panic!("Got this buffer: {}. I don't like it, ERROR!", bytes);
    }

    #[payable("*")]
    #[endpoint]
    fn dynamic_message_multiple(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: BigUint,
    ) {
        sc_panic!(
            "Got token {}, with nonce {}, amount {}. I prefer EGLD. ERROR!",
            token_id,
            nonce,
            amount
        );
    }

    #[payable("*")]
    #[endpoint]
    fn dynamic_message_ascii(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: BigUint,
    ) {
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
    fn format_message_one_argument(&self) -> ManagedBuffer {
        let message = sc_format!("Test");
        message
    }

    #[endpoint]
    fn format_message_multiple_arguments(&self, x: i32) -> ManagedBuffer {
        let message = sc_format!("Hello {} world", x);
        message
    }
}
