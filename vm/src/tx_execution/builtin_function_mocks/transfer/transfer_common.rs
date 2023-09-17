use crate::{
    tx_execution::{
        builtin_function_mocks::builtin_func_trait::BuiltinFunctionEsdtTransferInfo,
        BlockchainVMRef,
    },
    tx_mock::{
        BlockchainUpdate, CallType, TxCache, TxFunctionName, TxInput, TxLog, TxResult,
        TxTokenTransfer,
    },
    types::{top_decode_u64, VMAddress},
};
use num_bigint::BigUint;
use num_traits::Zero;

pub(super) struct ParsedTransferBuiltinFunCall {
    pub destination: VMAddress,
    pub raw_esdt_transfers: Vec<RawEsdtTransfer>,
    pub func_name: TxFunctionName,
    pub args: Vec<Vec<u8>>,
}

pub(super) struct RawEsdtTransfer {
    pub token_identifier: Vec<u8>,
    pub nonce_bytes: Vec<u8>,
    pub value_bytes: Vec<u8>,
}

/// Convenience function for populating log topics and data with transfer fields as bytes.
pub(super) fn push_transfer_bytes(transfers: &[RawEsdtTransfer], dest: &mut Vec<Vec<u8>>) {
    for transfer in transfers {
        dest.push(transfer.token_identifier.clone());
        dest.push(transfer.nonce_bytes.clone());
        dest.push(transfer.value_bytes.clone());
    }
}

pub(super) fn process_raw_esdt_transfer(raw_esdt_transfer: RawEsdtTransfer) -> TxTokenTransfer {
    TxTokenTransfer {
        token_identifier: raw_esdt_transfer.token_identifier,
        nonce: top_decode_u64(raw_esdt_transfer.nonce_bytes.as_slice()),
        value: BigUint::from_bytes_be(raw_esdt_transfer.value_bytes.as_slice()),
    }
}

fn process_raw_esdt_transfers(raw_esdt_transfers: Vec<RawEsdtTransfer>) -> Vec<TxTokenTransfer> {
    raw_esdt_transfers
        .into_iter()
        .map(process_raw_esdt_transfer)
        .collect()
}

pub(super) fn extract_transfer_info(
    parsed_tx: ParsedTransferBuiltinFunCall,
) -> BuiltinFunctionEsdtTransferInfo {
    BuiltinFunctionEsdtTransferInfo {
        real_recipient: parsed_tx.destination,
        transfers: process_raw_esdt_transfers(parsed_tx.raw_esdt_transfers),
    }
}

pub(super) fn execute_transfer_builtin_func<F>(
    vm: &BlockchainVMRef,
    parsed_tx: ParsedTransferBuiltinFunCall,
    tx_input: TxInput,
    tx_cache: TxCache,
    log: TxLog,
    f: F,
) -> (TxResult, BlockchainUpdate)
where
    F: FnOnce(),
{
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
        ..Default::default()
    };

    let (mut tx_result, blockchain_updates) = vm.default_execution(exec_input, tx_cache, f);

    // prepends esdt log
    // tx_result.result_logs = [builtin_logs.as_slice(), tx_result.result_logs.as_slice()].concat();
    tx_result.result_logs.insert(0, log);

    (tx_result, blockchain_updates)
}

pub(super) fn adjust_call_type(
    call_type: CallType,
    call: &ParsedTransferBuiltinFunCall,
) -> CallType {
    if call_type == CallType::TransferExecute && call.func_name.is_empty() {
        CallType::DirectCall
    } else {
        call_type
    }
}

pub(super) fn push_func_name_if_necessary(
    call_type: CallType,
    func_name: &TxFunctionName,
    data: &mut Vec<Vec<u8>>,
) {
    if call_type == CallType::DirectCall {
        return;
    }

    if func_name.is_empty() {
        return;
    }

    data.push(func_name.clone().into_bytes());
}
