use crate::{display_util::*, num_bigint::BigUint};
use alloc::vec::Vec;
use elrond_wasm::{
    api::{ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME},
    elrond_codec::TopDecode,
    types::heap::{Address, H256},
};
use num_traits::Zero;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: Address,
    pub to: Address,
    pub egld_value: BigUint,
    pub esdt_values: Vec<TxInputESDT>,
    pub func_name: Vec<u8>,
    pub args: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
}

impl TxInput {
    pub fn convert_to_token_transfer(self) -> Self {
        match &self.func_name[..] {
            ESDT_TRANSFER_FUNC_NAME => self.convert_to_esdt_transfer(),
            ESDT_NFT_TRANSFER_FUNC_NAME => self.convert_to_nft_transfer(),
            ESDT_MULTI_TRANSFER_FUNC_NAME => self.convert_to_esdt_multi_transfer(),
            _ => self,
        }
    }

    fn convert_to_esdt_transfer(self) -> Self {
        let token_identifier = self.args[0].clone();
        let value = BigUint::from_bytes_be(self.args[1].as_slice());

        let esdt_values = vec![TxInputESDT {
            token_identifier: token_identifier.clone(),
            nonce: 0,
            value: value.clone(),
        }];

        let func_name = self.args.get(2).map(Vec::clone).unwrap_or_default();
        let args = if self.args.len() > 2 {
            self.args[3..].to_vec()
        } else {
            Vec::new()
        };

        TxInput {
            from: self.from,
            to: self.to,
            egld_value: BigUint::zero(),
            esdt_values,
            func_name,
            args,
            gas_limit: self.gas_limit,
            gas_price: self.gas_price,
            tx_hash: self.tx_hash,
        }
    }

    fn convert_to_nft_transfer(self) -> Self {
        let token_identifier = self.args[0].clone();
        let nonce = u64::top_decode(self.args[1].as_slice()).unwrap();
        let value = BigUint::from_bytes_be(self.args[2].as_slice());
        let destination = Address::top_decode(self.args[3].as_slice()).unwrap();

        let esdt_values = vec![TxInputESDT {
            token_identifier,
            nonce,
            value,
        }];

        let func_name = self.args.get(4).map(Vec::clone).unwrap_or_default();
        let args = if self.args.len() > 5 {
            self.args[5..].to_vec()
        } else {
            Vec::new()
        };

        TxInput {
            from: self.from,
            to: destination,
            egld_value: BigUint::zero(),
            esdt_values,
            func_name,
            args,
            gas_limit: self.gas_limit,
            gas_price: self.gas_price,
            tx_hash: self.tx_hash,
        }
    }

    fn convert_to_esdt_multi_transfer(self) -> Self {
        let mut arg_index = 0;
        let destination_bytes = self.args[arg_index].as_slice();
        let destination = Address::top_decode(destination_bytes).unwrap();
        arg_index += 1;
        let payments = usize::top_decode(self.args[arg_index].as_slice()).unwrap();
        arg_index += 1;

        let mut esdt_values = Vec::new();
        for _ in 0..payments {
            let token_identifier = self.args[arg_index].clone();
            arg_index += 1;
            let nonce_bytes = self.args[arg_index].clone();
            let nonce = u64::top_decode(nonce_bytes.as_slice()).unwrap();
            arg_index += 1;
            let value_bytes = self.args[arg_index].clone();
            let value = BigUint::from_bytes_be(value_bytes.as_slice());
            arg_index += 1;

            esdt_values.push(TxInputESDT {
                token_identifier: token_identifier.clone(),
                nonce,
                value: value.clone(),
            });
        }

        let func_name = self.args.get(arg_index).map(Vec::clone).unwrap_or_default();
        arg_index += 1;
        let args = if self.args.len() > arg_index {
            self.args[arg_index..].to_vec()
        } else {
            Vec::new()
        };

        TxInput {
            from: self.from,
            to: destination,
            egld_value: BigUint::zero(),
            esdt_values,
            func_name,
            args,
            gas_limit: self.gas_limit,
            gas_price: self.gas_price,
            tx_hash: self.tx_hash,
        }
    }
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, esdt_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            String::from_utf8(self.func_name.clone()).unwrap(),
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

    pub fn dummy() -> Self {
        TxInput {
            from: Address::zero(),
            to: Address::zero(),
            egld_value: BigUint::zero(),
            esdt_values: Vec::new(),
            func_name: Vec::new(),
            args: Vec::new(),
            gas_limit: 0,
            gas_price: 0,
            tx_hash: H256::zero(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TxInputESDT {
    pub token_identifier: Vec<u8>,
    pub nonce: u64,
    pub value: BigUint,
}
