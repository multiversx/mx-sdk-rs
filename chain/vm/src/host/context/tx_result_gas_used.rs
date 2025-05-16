#[derive(Clone, Default, Debug, PartialEq)]
pub enum GasUsed {
    #[default]
    Unknown,
    SomeGas(u64),
    AllGas(u64),
}

impl GasUsed {
    pub fn as_u64(&self) -> u64 {
        match self {
            GasUsed::Unknown => 0,
            GasUsed::SomeGas(gas) => *gas,
            GasUsed::AllGas(gas) => *gas,
        }
    }
}
