
use elrond_wasm::{H256, Address};
use crate::display_util::*;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use alloc::vec::Vec;


#[derive(Debug)]
pub struct AsyncCallTxData {
    pub to: Address,
    pub call_data: Vec<u8>,
    pub call_value: BigUint,
}

