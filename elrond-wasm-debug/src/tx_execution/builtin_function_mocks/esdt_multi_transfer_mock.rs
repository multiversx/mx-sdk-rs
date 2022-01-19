use elrond_wasm::{api::ESDT_MULTI_TRANSFER_FUNC_NAME, elrond_codec::TopDecode, types::Address};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxInputESDT, TxLog, TxResult},
};

pub fn execute_esdt_multi_transfer(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        let err_result =
            TxResult::from_vm_error("MultiESDTNFTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }
    assert!(
        tx_input.to == tx_input.from,
        "MultiESDTNFTTransfer expects that to == from"
    );

    let mut arg_index = 0;
    let destination = Address::top_decode(tx_input.args[arg_index].as_slice()).unwrap();
    arg_index += 1;
    let payments = usize::top_decode(tx_input.args[arg_index].as_slice()).unwrap();
    arg_index += 1;

    if tx_input.args.len() < 2 + payments * 3 {
        let err_result =
            TxResult::from_vm_error("MultiESDTNFTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }

    let mut esdt_values = Vec::new();
    let mut builtin_logs = Vec::new();
    for _ in 0..payments {
        let token_identifier = tx_input.args[arg_index].clone();
        arg_index += 1;
        let nonce_bytes = tx_input.args[arg_index].clone();
        let nonce = u64::top_decode(nonce_bytes.as_slice()).unwrap();
        arg_index += 1;
        let value_bytes = tx_input.args[arg_index].clone();
        let value = BigUint::from_bytes_be(value_bytes.as_slice());
        arg_index += 1;

        esdt_values.push(TxInputESDT {
            token_identifier: token_identifier.clone(),
            nonce,
            value: value.clone(),
        });

        builtin_logs.push(TxLog {
            address: tx_input.from.clone(),
            endpoint: ESDT_MULTI_TRANSFER_FUNC_NAME.to_vec(),
            topics: vec![token_identifier, nonce_bytes, value_bytes],
            data: vec![],
        });
    }

    let func_name = tx_input
        .args
        .get(arg_index)
        .map(Vec::clone)
        .unwrap_or_default();
    arg_index += 1;
    let args = if tx_input.args.len() > arg_index {
        tx_input.args[arg_index..].to_vec()
    } else {
        Vec::new()
    };

    let exec_input = TxInput {
        from: tx_input.from,
        to: destination,
        egld_value: BigUint::zero(),
        esdt_values,
        func_name,
        args,
        gas_limit: tx_input.gas_limit,
        gas_price: tx_input.gas_price,
        tx_hash: tx_input.tx_hash,
    };

    let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);

    // prepends esdt log
    tx_result.result_logs = [builtin_logs.as_slice(), tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
}
