use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    display_util::*,
    types::{VMAddress, H256},
};
use std::fmt;

use super::TxFunctionName;

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: VMAddress,
    pub to: VMAddress,
    pub egld_value: BigUint,
    pub esdt_values: Vec<TxTokenTransfer>,
    pub func_name: TxFunctionName,
    pub args: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
    pub promise_callback_closure_data: Vec<u8>,
    pub callback_payments: CallbackPayments,
}

impl Default for TxInput {
    fn default() -> Self {
        TxInput {
            from: VMAddress::zero(),
            to: VMAddress::zero(),
            egld_value: BigUint::zero(),
            esdt_values: Vec::new(),
            func_name: TxFunctionName::EMPTY,
            args: Vec::new(),
            gas_limit: 0,
            gas_price: 0,
            tx_hash: H256::zero(),
            promise_callback_closure_data: Vec::new(),
            callback_payments: Default::default(),
        }
    }
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, esdt_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            self.func_name.as_str(),
            self.args,
            self.egld_value,
            self.esdt_values,
            address_hex(&self.from),
            address_hex(&self.to))
    }
}

impl TxInput {
    pub fn add_arg(&mut self, arg: Vec<u8>) {
        self.args.push(arg);
    }

    pub fn func_name_from_arg_index(&self, arg_index: usize) -> TxFunctionName {
        if let Some(arg) = self.args.get(arg_index) {
            arg.into()
        } else {
            TxFunctionName::EMPTY
        }
    }
}

/// Models ESDT transfers between accounts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxTokenTransfer {
    pub token_identifier: Vec<u8>,
    pub nonce: u64,
    pub value: BigUint,
}

/// Signals to the callback that funds have been returned to it, without performing any transfer.
#[derive(Default, Clone, Debug)]
pub struct CallbackPayments {
    pub egld_value: BigUint,
    pub esdt_values: Vec<TxTokenTransfer>,
}

impl TxInput {
    /// The received EGLD can come either from the original caller, or from an async call, during callback.
    pub fn received_egld(&self) -> &BigUint {
        if !self.callback_payments.egld_value.is_zero() {
            &self.callback_payments.egld_value
        } else {
            &self.egld_value
        }
    }

    /// The received ESDT tokens can come either from the original caller, or from an async call, during callback.
    pub fn received_esdt(&self) -> &[TxTokenTransfer] {
        if !self.callback_payments.esdt_values.is_empty() {
            self.callback_payments.esdt_values.as_slice()
        } else {
            self.esdt_values.as_slice()
        }
    }

    pub fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8> {
        let arg_idx_usize = arg_index as usize;
        assert!(arg_idx_usize < self.args.len(), "Tx arg index out of range");
        self.args[arg_idx_usize].clone()
    }
}
