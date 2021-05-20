use crate::*;

const ESDT_TRANSFER_FUNC: &[u8] = b"ESDTTransfer";
const SET_USERNAME_FUNC: &[u8] = b"SetUserName";

pub fn try_execute_builtin_function(
	tx_input: &TxInput,
	state: &mut BlockchainMock,
) -> Option<TxResult> {
	match tx_input.func_name.as_slice() {
		ESDT_TRANSFER_FUNC => Some(execute_esdt_transfer(tx_input, state)),
		SET_USERNAME_FUNC => Some(execute_set_username(tx_input, state)),
		_ => None,
	}
}

fn execute_esdt_transfer(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
	let from = tx_input.from.clone();
	let to = tx_input.to.clone();
	let esdt_token_identifier = tx_input.esdt_token_identifier.clone();
	let esdt_value = tx_input.esdt_value.clone();

	state.substract_esdt_balance(&from, &esdt_token_identifier, &esdt_value);
	state.increase_esdt_balance(&to, &esdt_token_identifier, &esdt_value);
	TxResult::empty()
}

fn execute_set_username(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
	assert_eq!(tx_input.args.len(), 1, "SetUserName expects 1 argument");
	if state.try_set_username(&tx_input.to, tx_input.args[0].as_slice()) {
		TxResult::empty()
	} else {
		TxResult {
			result_status: 10,
			result_message: b"username already set".to_vec(),
			result_values: Vec::new(),
			result_logs: Vec::new(),
		}
	}
}
