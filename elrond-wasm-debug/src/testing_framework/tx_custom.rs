use crate::tx_mock::TxInputESDT;
use elrond_wasm::types::Address;

pub(crate) struct TxCallCustom {
    pub from: Address,
    pub to: Address,
    pub egld_value: num_bigint::BigUint,
    pub esdt: Vec<TxInputESDT>,
    pub function: String,
    pub arguments: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
}

pub(crate) struct TxQueryCustom {
    pub to: Address,
    pub function: String,
    pub arguments: Vec<Vec<u8>>,
}

pub(crate) struct TxExpectCustom {
    pub out: Vec<Vec<u8>>,
    pub status: u64,
    pub message: String,
    // TODO: Add logs?
}
