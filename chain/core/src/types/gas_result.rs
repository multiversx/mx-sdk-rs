/// Outcome of a gas simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GasResult {
    /// The simulated transaction ran out of gas, i.e. it needs more gas than allowed.
    OutOfGas,

    /// The simulated transaction succeeded, consuming the given amount of gas.
    GasUsed(u64),
}

impl GasResult {
    pub fn is_out_of_gas(&self) -> bool {
        matches!(self, GasResult::OutOfGas)
    }

    /// The gas used, or `None` if the simulation ran out of gas.
    pub fn gas_used(&self) -> Option<u64> {
        match self {
            GasResult::OutOfGas => None,
            GasResult::GasUsed(gas) => Some(*gas),
        }
    }
}

impl core::fmt::Display for GasResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GasResult::OutOfGas => write!(f, "out of gas"),
            GasResult::GasUsed(gas) => write!(f, "{gas} units"),
        }
    }
}
