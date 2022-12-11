use crate::{num_bigint::BigUint, tx_mock::TxFunctionName};
use elrond_wasm::{
    api::ESDT_MULTI_TRANSFER_FUNC_NAME, elrond_codec::TopDecode, types::heap::Address,
};
use num_traits::Zero;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxInputESDT, TxLog, TxResult},
};

use super::builtin_func_trait::BuiltinFunction;

pub struct ESDTMultiTransfer;

impl BuiltinFunction for ESDTMultiTransfer {
    fn name(&self) -> &str {
        ESDT_MULTI_TRANSFER_FUNC_NAME
    }

    fn extract_esdt_transfers(&self, tx_input: TxInput) -> Vec<TxInputESDT> {
        if let Ok(parsed_tx) = try_parse_input(&tx_input) {
            process_raw_esdt_transfers(parsed_tx.raw_esdt_transfers)
        } else {
            Vec::new()
        }
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => execute_esdt_multi_transfer(parsed_tx, tx_input, tx_cache),
            Err(message) => {
                let err_result = TxResult::from_vm_error(message.to_string());
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

struct ParsedMultiTransfer {
    destination: Address,
    raw_esdt_transfers: Vec<RawEsdtTransfer>,
    func_name: TxFunctionName,
    args: Vec<Vec<u8>>,
}
struct RawEsdtTransfer {
    token_identifier: Vec<u8>,
    nonce_bytes: Vec<u8>,
    value_bytes: Vec<u8>,
}

fn process_raw_esdt_transfer(raw_esdt_transfer: RawEsdtTransfer) -> TxInputESDT {
    TxInputESDT {
        token_identifier: raw_esdt_transfer.token_identifier,
        nonce: u64::top_decode(raw_esdt_transfer.nonce_bytes.as_slice()).unwrap(),
        value: BigUint::from_bytes_be(raw_esdt_transfer.value_bytes.as_slice()),
    }
}

fn process_raw_esdt_transfers(raw_esdt_transfers: Vec<RawEsdtTransfer>) -> Vec<TxInputESDT> {
    raw_esdt_transfers
        .into_iter()
        .map(process_raw_esdt_transfer)
        .collect()
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedMultiTransfer, &'static str> {
    if tx_input.args.len() < 2 {
        return Err("MultiESDTNFTTransfer too few arguments");
    }
    if tx_input.to != tx_input.from {
        // TODO: not sure what the real error message would be, certainly not this
        return Err("MultiESDTNFTTransfer expects that to == from");
    }

    let mut arg_index = 0;
    let destination_bytes = tx_input.args[arg_index].as_slice();
    let destination = Address::top_decode(destination_bytes).unwrap();
    arg_index += 1;
    let payments = usize::top_decode(tx_input.args[arg_index].as_slice()).unwrap();
    arg_index += 1;

    if tx_input.args.len() < 2 + payments * 3 {
        return Err("MultiESDTNFTTransfer too few arguments");
    }

    let mut raw_esdt_transfers = Vec::new();
    for _ in 0..payments {
        let token_identifier = tx_input.args[arg_index].clone();
        arg_index += 1;
        let nonce_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;
        let value_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;

        raw_esdt_transfers.push(RawEsdtTransfer {
            token_identifier: token_identifier.clone(),
            nonce_bytes,
            value_bytes,
        });
    }

    let func_name = tx_input.func_name_from_arg_index(arg_index);
    arg_index += 1;
    let args = if tx_input.args.len() > arg_index {
        tx_input.args[arg_index..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedMultiTransfer {
        destination,
        raw_esdt_transfers,
        func_name,
        args,
    })
}

fn execute_esdt_multi_transfer(
    parsed_tx: ParsedMultiTransfer,
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    let mut builtin_logs = Vec::new();
    for raw_esdt_transfer in &parsed_tx.raw_esdt_transfers {
        builtin_logs.push(TxLog {
            address: tx_input.from.clone(),
            endpoint: ESDT_MULTI_TRANSFER_FUNC_NAME.into(),
            topics: vec![
                raw_esdt_transfer.token_identifier.clone(),
                raw_esdt_transfer.nonce_bytes.clone(),
                raw_esdt_transfer.value_bytes.clone(),
                parsed_tx.destination.to_vec(),
            ],
            data: vec![],
        });
    }

    let exec_input = TxInput {
        from: tx_input.from,
        to: parsed_tx.destination,
        egld_value: BigUint::zero(),
        esdt_values: process_raw_esdt_transfers(parsed_tx.raw_esdt_transfers),
        func_name: parsed_tx.func_name,
        args: parsed_tx.args,
        gas_limit: tx_input.gas_limit,
        gas_price: tx_input.gas_price,
        tx_hash: tx_input.tx_hash,
        promise_callback_closure_data: Vec::new(),
    };

    let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);

    // prepends esdt log
    tx_result.result_logs = [builtin_logs.as_slice(), tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
}
