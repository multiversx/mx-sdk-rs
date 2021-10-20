use elrond_wasm::types::Address;
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxInputESDT, TxLog, TxResult},
};

use super::builtin_func_exec::ESDT_TRANSFER_FUNC;

pub fn execute_esdt_transfer(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        let err_result = TxResult::from_vm_error("ESDTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }

    let token_identifier = tx_input.args[0].clone();
    let value = BigUint::from_bytes_be(tx_input.args[1].as_slice());

    // TODO: uncomment, after removing esdt transfer from `default_execution`
    // tx_cache.subtract_esdt_balance(&tx_input.from, &token_identifier, 0, &value);
    // tx_cache.increase_esdt_balance(&tx_input.to, &token_identifier, 0, &value);

    let esdt_values = vec![TxInputESDT {
        token_identifier: token_identifier.clone(),
        nonce: 0,
        value: value.clone(),
    }];

    let esdt_transfer_log = esdt_transfer_event_log(
        tx_input.from.clone(),
        tx_input.to.clone(),
        token_identifier,
        &value,
    );

    let func_name = tx_input.args.get(2).map(Vec::clone).unwrap_or(Vec::new());
    let args = if tx_input.args.len() > 2 {
        tx_input.args[3..].to_vec()
    } else {
        Vec::new()
    };

    let exec_input = TxInput {
        from: tx_input.from,
        to: tx_input.to,
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
    tx_result.result_logs = [&[esdt_transfer_log][..], tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
}

pub fn esdt_transfer_event_log(
    from: Address,
    to: Address,
    esdt_token_identifier: Vec<u8>,
    esdt_value: &BigUint,
) -> TxLog {
    let nonce_topic = Vec::<u8>::new();
    TxLog {
        address: from,
        endpoint: ESDT_TRANSFER_FUNC.to_vec(),
        topics: vec![
            esdt_token_identifier,
            nonce_topic,
            esdt_value.to_bytes_be(),
            to.to_vec(),
        ],
        data: vec![],
    }
}
