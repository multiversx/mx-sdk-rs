multiversx_sc::imports!();

const GAS_OVERHEAD: u64 = 700_000;

#[multiversx_sc::module]
pub trait ForwarderBlindCommon {
    fn tx_gas(&self) -> u64 {
        self.blockchain().get_gas_left() - GAS_OVERHEAD
    }
}
