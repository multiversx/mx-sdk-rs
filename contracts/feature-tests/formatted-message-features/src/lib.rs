#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait FormattedMessageFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn dynamic_message(&self, bytes: ManagedBuffer) {
        signal_error!("Got this buffer: {:x}. I don't like it, ERROR!", bytes);
    }

    #[payable("*")]
    #[endpoint]
    fn dynamic_message_multiple(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: BigUint,
    ) {
        signal_error!(
            "Got token {:x}, with nonce {:x}, amount {:x}. I prefer EGLD. ERROR!",
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
        signal_error!(
            "Got token {}, with nonce {:x}, amount {:x}. I prefer EGLD. ERROR!",
            token_id,
            nonce,
            amount
        );
    }
}
