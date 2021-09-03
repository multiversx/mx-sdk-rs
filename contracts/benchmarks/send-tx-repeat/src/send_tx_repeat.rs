#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait SendTxRepeat {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint]
    fn repeat(
        &self,
        to: ManagedAddress,
        amount: BigUint,
        times: usize,
        #[var_args] opt_data: OptionalArg<Vec<u8>>,
    ) {
        let data = match opt_data {
            OptionalArg::Some(d) => d,
            OptionalArg::None => Vec::new(),
        };
        for _ in 0..times {
            self.send().direct_egld(&to, &amount, &data);
        }
    }
}
