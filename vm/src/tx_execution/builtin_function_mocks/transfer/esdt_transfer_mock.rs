use crate::{
    tx_execution::{
        builtin_function_names::ESDT_TRANSFER_FUNC_NAME, BlockchainVMRef,
        BuiltinFunctionEsdtTransferInfo,
    },
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
};

use super::{
    super::builtin_func_trait::BuiltinFunction,
    transfer_common::{
        adjust_call_type, execute_transfer_builtin_func, extract_transfer_info,
        push_func_name_if_necessary, push_transfer_bytes, ParsedTransferBuiltinFunCall,
        RawEsdtTransfer,
    },
};

pub struct ESDTTransfer;

impl BuiltinFunction for ESDTTransfer {
    fn name(&self) -> &str {
        ESDT_TRANSFER_FUNC_NAME
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

    let mut data = vec![
        call_type.to_log_bytes(),
        ESDT_TRANSFER_FUNC_NAME.into(),
        call.raw_esdt_transfers[0].token_identifier.clone(),
        call.raw_esdt_transfers[0].value_bytes.clone(),
    ];
    push_func_name_if_necessary(call_type, &call.func_name, &mut data);

    TxLog {
        address: tx_input.from.clone(),
        endpoint: ESDT_TRANSFER_FUNC_NAME.into(),
        topics,
        data,
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 2 {
        return Err("ESDTTransfer too few arguments");
    }

    let token_identifier = tx_input.args[0].clone();
    let value_bytes = tx_input.args[1].clone();

    let func_name = tx_input.func_name_from_arg_index(2);
    let args = if tx_input.args.len() > 2 {
        tx_input.args[3..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination: tx_input.to.clone(),
        raw_esdt_transfers: vec![RawEsdtTransfer {
            token_identifier,
            nonce_bytes: Vec::new(),
            value_bytes,
        }],
        func_name,
        args,
    })
}
