pub trait TxGas {}

impl TxGas for () {}

pub struct ExplicitGas(pub u64);

impl TxGas for ExplicitGas {}
