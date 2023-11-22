use num_traits::Zero;

use crate::{
    tx_execution::execute_system_sc,
    tx_mock::{
        BlockchainUpdate, CallType, TxCache, TxContext, TxContextStack, TxFunctionName, TxInput,
        TxLog, TxResult,
    },
    types::{top_encode_big_uint, VMAddress},
    with_shared::Shareable,
};

use super::{is_system_sc_address, BlockchainVMRef};

fn should_execute_sc_call(tx_input: &TxInput) -> bool {
    // execute whitebox calls no matter what
    if tx_input.func_name == TxFunctionName::WHITEBOX_CALL {
        return true;
    }

    // don't execute anything for an EOA
    if !tx_input.to.is_smart_contract_address() {
        return false;
    }

    // calls with empty func name are simple transfers
    !tx_input.func_name.is_empty()
}

fn should_add_transfer_value_log(tx_input: &TxInput) -> bool {
    if tx_input.call_type == CallType::AsyncCallback
        && !tx_input.callback_payments.esdt_values.is_empty()
    {
        // edge case in the VM
        return false;
    }

    if tx_input.call_type == CallType::UpgradeFromSource {
        // already handled in upgradeContract builtin function
        return false;
    }

    if tx_input.call_type != CallType::DirectCall {
        return true;
    }

    // skip for transactions coming directly from scenario json, which should all be coming from user wallets
    tx_input.from.is_smart_contract_address() && !tx_input.egld_value.is_zero()
}

pub(crate) fn create_transfer_value_log(tx_input: &TxInput, call_type: CallType) -> TxLog {
    let mut data = vec![call_type.to_log_bytes(), tx_input.func_name.to_bytes()];
    data.append(&mut tx_input.args.clone());

    let egld_value = if tx_input.call_type == CallType::AsyncCallback {
        &tx_input.callback_payments.egld_value
    } else {
        &tx_input.egld_value
    };

    TxLog {
        address: tx_input.from.clone(),
        endpoint: "transferValueOnly".into(),
        topics: vec![top_encode_big_uint(egld_value), tx_input.to.to_vec()],
        data,
    }
}

impl BlockchainVMRef {
    /// Executes without builtin functions, directly on the contract or the given lambda closure.
    pub fn default_execution<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        if let Err(err) =
            tx_cache.transfer_egld_balance(&tx_input.from, &tx_input.to, &tx_input.egld_value)
        {
            return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
        }

        let transfer_value_log = if should_add_transfer_value_log(&tx_input) {
            Some(create_transfer_value_log(&tx_input, tx_input.call_type))
        } else {
            None
        };

        // TODO: temporary, will convert to explicit builtin function first
        for esdt_transfer in tx_input.esdt_values.iter() {
            let transfer_result = tx_cache.transfer_esdt_balance(
                &tx_input.from,
                &tx_input.to,
                &esdt_transfer.token_identifier,
                esdt_transfer.nonce,
                &esdt_transfer.value,
            );
            if let Err(err) = transfer_result {
                return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
            }
        }

        let (mut tx_result, blockchain_updates) = if is_system_sc_address(&tx_input.to) {
            execute_system_sc(tx_input, tx_cache)
        } else if should_execute_sc_call(&tx_input) {
            let tx_context = TxContext::new(self.clone(), tx_input, tx_cache);
            let mut tx_context_sh = Shareable::new(tx_context);

            TxContextStack::execute_on_vm_stack(&mut tx_context_sh, f);

            tx_context_sh.into_inner().into_results()
        } else {
            // no execution
            (TxResult::empty(), tx_cache.into_blockchain_updates())
        };

        if let Some(tv_log) = transfer_value_log {
            tx_result.result_logs.insert(0, tv_log);
        }

        (tx_result, blockchain_updates)
    }

    pub fn deploy_contract<F>(
        &self,
        mut tx_input: TxInput,
        contract_path: Vec<u8>,
        tx_cache: TxCache,
        f: F,
    ) -> (TxResult, VMAddress, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        let new_address = tx_cache.get_new_address(&tx_input.from);
        tx_input.to = new_address.clone();
        tx_input.func_name = TxFunctionName::INIT;
        let tx_context = TxContext::new(self.clone(), tx_input, tx_cache);
        let mut tx_context_sh = Shareable::new(tx_context);
        let tx_input_ref = tx_context_sh.input_ref();

        if let Err(err) = tx_context_sh
            .tx_cache
            .subtract_egld_balance(&tx_input_ref.from, &tx_input_ref.egld_value)
        {
            return (
                TxResult::from_panic_obj(&err),
                VMAddress::zero(),
                BlockchainUpdate::empty(),
            );
        }
        tx_context_sh.create_new_contract(&new_address, contract_path, tx_input_ref.from.clone());
        tx_context_sh
            .tx_cache
            .increase_egld_balance(&new_address, &tx_input_ref.egld_value);

        TxContextStack::execute_on_vm_stack(&mut tx_context_sh, f);

        let (tx_result, blockchain_updates) = tx_context_sh.into_inner().into_results();
        (tx_result, new_address, blockchain_updates)
    }
}
