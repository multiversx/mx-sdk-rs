use crate::tx_execution::{
    builtin_function_names::UPGRADE_CONTRACT_FUNC_NAME, create_transfer_value_log, BlockchainVMRef,
};

use crate::tx_mock::{BlockchainUpdate, CallType, TxCache, TxFunctionName, TxInput, TxResult};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct UpgradeContract;

impl BuiltinFunction for UpgradeContract {
    fn name(&self) -> &str {
        UPGRADE_CONTRACT_FUNC_NAME
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
        if tx_input.args.len() < 2 {
            return (
                TxResult::from_vm_error("upgradeContract expects at least 2 arguments"),
                BlockchainUpdate::empty(),
            );
        }

        let new_code = tx_input.args[0].clone();

        // tx_input.args[1] is the code metadata, we ignore for now
        // TODO: model code metadata in Mandos

        let args = if tx_input.args.len() > 2 {
            tx_input.args[2..].to_vec()
        } else {
            Vec::new()
        };

        tx_cache.with_account_mut(&tx_input.to, |account| {
            account.contract_path = Some(new_code);
        });

        let transfer_value_log = create_transfer_value_log(&tx_input, CallType::UpgradeFromSource);

        let exec_input = TxInput {
            from: tx_input.from,
            to: tx_input.to,
            egld_value: tx_input.egld_value,
            esdt_values: Vec::new(),
            func_name: TxFunctionName::UPGRADE,
            args,
            gas_limit: tx_input.gas_limit,
            gas_price: tx_input.gas_price,
            tx_hash: tx_input.tx_hash,
            call_type: CallType::UpgradeFromSource,
            ..Default::default()
        };

        let (mut tx_result, blockchain_updates) = vm.default_execution(exec_input, tx_cache, f);
        adjust_upgrade_log_endpoint(&mut tx_result);
        tx_result.result_logs.insert(0, transfer_value_log);
        (tx_result, blockchain_updates)
    }
}

/// A small anomaly in the VM: logs from the "upgrade" function appear under "upgradeContract".
fn adjust_upgrade_log_endpoint(tx_result: &mut TxResult) {
    for log in &mut tx_result.result_logs {
        if log.endpoint == TxFunctionName::UPGRADE {
            log.endpoint = UPGRADE_CONTRACT_FUNC_NAME.into();
        }
    }
}
