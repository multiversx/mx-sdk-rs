multiversx_sc::imports!();

pub const GAS_OVERHEAD: u64 = 7_000_000;

#[multiversx_sc::module]
pub trait ForwarderBlindCommon {
    fn tx_gas(&self) -> u64 {
        require!(
            self.blockchain().get_gas_left() > GAS_OVERHEAD,
            "not enough gas for forwarding"
        );

        self.blockchain().get_gas_left() - GAS_OVERHEAD
    }
}
