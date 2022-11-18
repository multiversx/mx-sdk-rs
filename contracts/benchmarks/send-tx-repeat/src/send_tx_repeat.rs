#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait SendTxRepeat {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint]
    fn repeat(&self, to: ManagedAddress, amount: BigUint, times: usize) {
        for _ in 0..times {
            self.send().direct_egld(&to, &amount);
        }
    }
}
