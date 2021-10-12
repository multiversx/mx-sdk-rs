use crate::world_mock::{AccountEsdt, BlockInfo, BlockchainTxInfo};
use alloc::vec::Vec;
use core::cell::RefCell;
use elrond_wasm::types::Address;

use super::{TxInput, TxManagedTypes, TxOutput};

#[derive(Debug)]
pub struct TxContext {
    pub blockchain_info_box: Box<BlockchainTxInfo>,
    pub tx_input_box: Box<TxInput>,
    pub managed_types: RefCell<TxManagedTypes>,
    pub tx_output_cell: RefCell<TxOutput>,
}

impl TxContext {
    pub fn new(blockchain_info: BlockchainTxInfo, tx_input: TxInput, tx_output: TxOutput) -> Self {
        TxContext {
            blockchain_info_box: Box::new(blockchain_info),
            tx_input_box: Box::new(tx_input),
            managed_types: RefCell::new(TxManagedTypes::new()),
            tx_output_cell: RefCell::new(tx_output),
        }
    }

    pub fn dummy() -> Self {
        TxContext {
            blockchain_info_box: Box::new(BlockchainTxInfo {
                previous_block_info: BlockInfo::new(),
                current_block_info: BlockInfo::new(),
                contract_balance: 0u32.into(),
                contract_esdt: AccountEsdt::default(),
                contract_owner: None,
            }),
            tx_input_box: Box::new(TxInput {
                from: Address::zero(),
                to: Address::zero(),
                egld_value: 0u32.into(),
                esdt_values: Vec::new(),
                func_name: Vec::new(),
                args: Vec::new(),
                gas_limit: 0,
                gas_price: 0,
                tx_hash: b"dummy...........................".into(),
            }),
            managed_types: RefCell::new(TxManagedTypes::new()),
            tx_output_cell: RefCell::new(TxOutput::default()),
        }
    }
}
