use crate::{
    tx_execution::{
        builtin_function_names::ESDT_NFT_TRANSFER_FUNC_NAME, BlockchainVMRef,
        BuiltinFunctionEsdtTransferInfo,
    },
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    types::VMAddress,
};

use super::{
    super::BuiltinFunction,
    transfer_common::{
        adjust_call_type, execute_transfer_builtin_func, extract_transfer_info,
        push_func_name_if_necessary, push_transfer_bytes, ParsedTransferBuiltinFunCall,
        RawEsdtTransfer,
    },
};

pub struct ESDTNftTransfer;

impl BuiltinFunction for ESDTNftTransfer {
    fn name(&self) -> &str {
        ESDT_NFT_TRANSFER_FUNC_NAME
    }

    fn extract_esdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionEsdtTransferInfo {
        if let Ok(parsed_tx) = try_parse_input(tx_input) {
            extract_transfer_info(parsed_tx)
        } else {
            BuiltinFunctionEsdtTransferInfo::empty(tx_input)
        }
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        vm: &BlockchainVMRef,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                let log = build_log(&tx_input, &parsed_tx);
                execute_transfer_builtin_func(vm, parsed_tx, tx_input, tx_cache, log, f)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message);
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn build_log(tx_input: &TxInput, call: &ParsedTransferBuiltinFunCall) -> TxLog {
    let call_type = adjust_call_type(tx_input.call_type, call);
    let mut topics = Vec::new();
    push_transfer_bytes(&call.raw_esdt_transfers, &mut topics);
    topics.push(call.destination.to_vec());

    let mut data = Vec::new();

    data.push(call_type.to_log_bytes());
    data.push(ESDT_NFT_TRANSFER_FUNC_NAME.into());
    push_transfer_bytes(&call.raw_esdt_transfers, &mut data);
    data.push(call.destination.to_vec());
    push_func_name_if_necessary(call_type, &call.func_name, &mut data);

    TxLog {
        address: tx_input.from.clone(),
        endpoint: ESDT_NFT_TRANSFER_FUNC_NAME.into(),
        topics,
        data,
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 4 {
        return Err("ESDTNFTTransfer too few arguments");
    }
    if tx_input.to != tx_input.from {
        // TODO: not sure what the real error message would be, certainly not this
        return Err("ESDTNFTTransfer expects that to == from");
    }

    let token_identifier = tx_input.args[0].clone();
    let nonce_bytes = tx_input.args[1].clone();
    let value_bytes = tx_input.args[2].clone();
    let destination = VMAddress::from_slice(&tx_input.args[3]);

    let func_name = tx_input.func_name_from_arg_index(4);
    let args = if tx_input.args.len() > 5 {
        tx_input.args[5..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination,
        raw_esdt_transfers: vec![RawEsdtTransfer {
            token_identifier,
            nonce_bytes,
            value_bytes,
        }],
        func_name,
        args,
    })
}
