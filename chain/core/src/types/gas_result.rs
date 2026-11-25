use alloc::string::String;

/// Outcome of a gas simulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GasResult {
    /// The simulated transaction succeeded, consuming the given amount of gas.
    GasUsed(u64),

    /// The simulated transaction ran out of gas, i.e. it needs more gas than allowed.
    OutOfGas,

    /// The simulated transaction failed for a reason other than running out of gas.
    ///
    /// Contains the error message returned by the simulation.
    Failed(String),
}

impl GasResult {
    pub fn is_out_of_gas(&self) -> bool {
        matches!(self, GasResult::OutOfGas)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, GasResult::Failed(_))
    }

    /// The gas used, or `None` if the simulation ran out of gas or failed.
    pub fn gas_used(&self) -> Option<u64> {
        match self {
            GasResult::GasUsed(gas) => Some(*gas),
            GasResult::OutOfGas | GasResult::Failed(_) => None,
        }
    }
}

impl core::fmt::Display for GasResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GasResult::GasUsed(gas) => write!(f, "{gas} units"),
            GasResult::OutOfGas => write!(f, "out of gas"),
            GasResult::Failed(message) => write!(f, "failed: {message}"),
        }
    }
}
