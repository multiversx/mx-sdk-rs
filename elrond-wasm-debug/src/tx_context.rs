use crate::{blockchain_mock::*, world_mock::AccountEsdt, TxInput, TxOutput};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use elrond_wasm::types::Address;

#[derive(Debug)]
pub struct TxContext {
    pub blockchain_info_box: Box<BlockchainTxInfo>,
    pub tx_input_box: Box<TxInput>,
    pub tx_output_cell: Rc<RefCell<TxOutput>>,
}

impl TxContext {
    pub fn new(blockchain_info: BlockchainTxInfo, tx_input: TxInput, tx_output: TxOutput) -> Self {
        TxContext {
            blockchain_info_box: Box::new(blockchain_info),
            tx_input_box: Box::new(tx_input),
            tx_output_cell: Rc::new(RefCell::new(tx_output)),
        }
    }

    pub fn into_output(self) -> TxOutput {
        let ref_cell = Rc::try_unwrap(self.tx_output_cell).unwrap();
        ref_cell.replace(TxOutput::default())
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
            tx_output_cell: Rc::new(RefCell::new(TxOutput::default())),
        }
    }
}

impl Clone for TxContext {
    fn clone(&self) -> Self {
        TxContext {
            blockchain_info_box: self.blockchain_info_box.clone(),
            tx_input_box: self.tx_input_box.clone(),
            tx_output_cell: Rc::clone(&self.tx_output_cell),
        }
    }
}
