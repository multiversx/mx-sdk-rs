#![allow(unused_variables)] // for now

use crate::*;
use elrond_wasm::types::*;
use mandos::*;
use num_bigint::BigUint;
use num_traits::Zero;
use std::path::Path;

pub fn parse_execute_mandos<P: AsRef<Path>>(
	relative_path: P,
	contract_map: &ContractMap<TxContext>,
) {
	let mut absolute_path = std::env::current_dir().unwrap();
	absolute_path.push(relative_path);
	let mut state = BlockchainMock::new();
	parse_execute_mandos_steps(absolute_path.as_ref(), &mut state, contract_map);
}

fn parse_execute_mandos_steps(
	steps_path: &Path,
	state: &mut BlockchainMock,
	contract_map: &ContractMap<TxContext>,
) {
	let scenario = mandos::parse_scenario(steps_path);

	for step in scenario.steps.iter() {
		match step {
			Step::ExternalSteps { path } => {
				let parent_path = steps_path.parent().unwrap();
				let new_path = parent_path.join(path);
				parse_execute_mandos_steps(&new_path.as_path(), state, contract_map);
			},
			Step::SetState {
				comment,
				accounts,
				new_addresses,
				block_hashes,
				previous_block_info,
				current_block_info,
			} => {
				for (address, account) in accounts.iter() {
					let storage = account
						.storage
						.iter()
						.map(|(k, v)| (k.value.clone(), v.value.clone()))
						.collect();
					let esdt = if let Some(esdt_map) = &account.esdt {
						esdt_map
							.iter()
							.map(|(k, v)| (k.value.clone(), v.value.clone()))
							.collect()
					} else {
						HashMap::new()
					};
					state.add_account(AccountData {
						address: address.value.into(),
						nonce: account.nonce.value,
						balance: account.balance.value.clone(),
						esdt,
						username: account
							.username
							.as_ref()
							.map(|bytes_value| bytes_value.value.clone())
							.unwrap_or_default(),
						storage,
						contract_path: account
							.code
							.as_ref()
							.map(|bytes_value| bytes_value.value.clone()),
						contract_owner: account
							.owner
							.as_ref()
							.map(|address_value| address_value.value.into()),
					});
				}
				for new_address in new_addresses.iter() {
					state.put_new_address(
						new_address.creator_address.value.into(),
						new_address.creator_nonce.value,
						new_address.new_address.value.into(),
					)
				}
				if let Some(block_info_obj) = &**previous_block_info {
					update_block_info(&mut state.previous_block_info, block_info_obj);
				}
				if let Some(block_info_obj) = &**current_block_info {
					update_block_info(&mut state.current_block_info, block_info_obj);
				}
			},
			Step::ScCall {
				tx_id,
				comment,
				tx,
				expect,
			} => {
				println!("Executing {}", tx_id);

				let tx_input = TxInput {
					from: tx.from.value.into(),
					to: tx.to.value.into(),
					call_value: tx.call_value.value.clone(),
					esdt_value: if let Some(esdt) = &tx.esdt_value {
						// TODO: clean this up
						esdt.esdt_value.value.clone()
					} else {
						BigUint::zero()
					},
					esdt_token_identifier: if let Some(esdt) = &tx.esdt_value {
						// TODO: clean this up
						esdt.esdt_token_name.value.clone()
					} else {
						Vec::new()
					},
					func_name: tx.function.as_bytes().to_vec(),
					args: tx
						.arguments
						.iter()
						.map(|scen_arg| scen_arg.value.clone())
						.collect(),
					gas_limit: tx.gas_limit.value,
					gas_price: tx.gas_price.value,
					tx_hash: generate_tx_hash_dummy(tx_id.as_str()),
				};
				state.increase_nonce(&tx_input.from);
				let tx_result =
					execute_sc_call_with_async_and_callback(tx_input, state, contract_map).unwrap();
				if let Some(tx_expect) = expect {
					check_tx_output(tx_id.as_str(), &tx_expect, &tx_result);
				}
			},
			Step::ScQuery {
				tx_id,
				comment,
				tx,
				expect,
			} => {
				let tx_input = TxInput {
					from: tx.to.value.into(),
					to: tx.to.value.into(),
					call_value: BigUint::from(0u32),
					esdt_value: BigUint::from(0u32),
					esdt_token_identifier: Vec::new(),
					func_name: tx.function.as_bytes().to_vec(),
					args: tx
						.arguments
						.iter()
						.map(|scen_arg| scen_arg.value.clone())
						.collect(),
					gas_limit: u64::MAX,
					gas_price: 0u64,
					tx_hash: generate_tx_hash_dummy(tx_id.as_str()),
				};

				let (tx_result, opt_async_data) =
					execute_sc_call(tx_input, state, contract_map).unwrap();
				if tx_result.result_status == 0 && opt_async_data.is_some() {
					panic!("Can't query a view function that performs an async call");
				}
				if let Some(tx_expect) = expect {
					check_tx_output(tx_id.as_str(), &tx_expect, &tx_result);
				}
			},
			Step::ScDeploy {
				tx_id,
				comment,
				tx,
				expect,
			} => {
				let tx_input = TxInput {
					from: tx.from.value.into(),
					to: Address::zero(),
					call_value: tx.call_value.value.clone(),
					esdt_value: tx.esdt_value.value.clone(),
					esdt_token_identifier: tx.esdt_token_name.value.clone(),
					func_name: b"init".to_vec(),
					args: tx
						.arguments
						.iter()
						.map(|scen_arg| scen_arg.value.clone())
						.collect(),
					gas_limit: tx.gas_limit.value,
					gas_price: tx.gas_price.value,
					tx_hash: generate_tx_hash_dummy(tx_id.as_str()),
				};
				state.increase_nonce(&tx_input.from);
				let (tx_result, _) =
					execute_sc_create(tx_input, &tx.contract_code.value, state, contract_map)
						.unwrap();
				if let Some(tx_expect) = expect {
					check_tx_output(tx_id.as_str(), &tx_expect, &tx_result);
				}
			},
			Step::Transfer { tx_id, comment, tx } => {
				let sender_address = &tx.from.value.into();
				state.increase_nonce(sender_address);
				state
					.subtract_tx_payment(sender_address, &tx.value.value)
					.unwrap();
				let recipient_address = &tx.to.value.into();
				state.increase_balance(recipient_address, &tx.value.value);
				let esdt_token_identifier = tx.esdt_token_name.value.clone();
				let esdt_value = tx.esdt_value.value.clone();

				if !esdt_token_identifier.is_empty() && esdt_value > 0u32.into() {
					state.substract_esdt_balance(
						sender_address,
						&esdt_token_identifier[..],
						&esdt_value,
					);
					state.increase_esdt_balance(
						recipient_address,
						&esdt_token_identifier[..],
						&esdt_value,
					);
				}
			},
			Step::ValidatorReward { tx_id, comment, tx } => {
				state.increase_validator_reward(&tx.to.value.into(), &tx.value.value);
			},
			Step::CheckState { comment, accounts } => {
				check_state(accounts, state);
			},
			Step::DumpState { .. } => {
				state.print_accounts();
			},
		}
	}
}

fn execute_sc_call(
	tx_input: TxInput,
	state: &mut BlockchainMock,
	contract_map: &ContractMap<TxContext>,
) -> Result<(TxResult, Option<AsyncCallTxData>), BlockchainMockError> {
	if let Some(tx_result) = try_execute_builtin_function(&tx_input, state) {
		return Ok((tx_result, None));
	}

	let from = tx_input.from.clone();
	let to = tx_input.to.clone();
	let call_value = tx_input.call_value.clone();
	let blockchain_info = state.create_tx_info(&to);

	state.subtract_tx_payment(&from, &call_value)?;
	state.subtract_tx_gas(&from, tx_input.gas_limit, tx_input.gas_price);

	let esdt_token_identifier = tx_input.esdt_token_identifier.clone();
	let esdt_value = tx_input.esdt_value.clone();
	let esdt_used = !esdt_token_identifier.is_empty() && esdt_value > 0u32.into();

	if esdt_used {
		state.substract_esdt_balance(&from, &esdt_token_identifier, &esdt_value)
	}

	let contract_account = state
		.accounts
		.get_mut(&to)
		.unwrap_or_else(|| panic!("Recipient account not found: {}", address_hex(&to)));

	let contract_path = &contract_account
		.contract_path
		.clone()
		.unwrap_or_else(|| panic!("Recipient account is not a smart contract"));

	let tx_context = TxContext::new(
		blockchain_info,
		tx_input,
		TxOutput {
			contract_storage: contract_account.storage.clone(),
			result: TxResult::empty(),
			send_balance_list: Vec::new(),
			async_call: None,
		},
	);

	let tx_output = execute_tx(tx_context, contract_path, contract_map);
	let tx_result = tx_output.result;

	if tx_result.result_status == 0 {
		// replace storage with new one
		let _ = std::mem::replace(&mut contract_account.storage, tx_output.contract_storage);

		state.increase_balance(&to, &call_value);
		if esdt_used {
			state.increase_esdt_balance(&to, &esdt_token_identifier, &esdt_value);
		}

		state.send_balance(&to, tx_output.send_balance_list.as_slice())?;
	} else {
		state.increase_balance(&from, &call_value);

		if esdt_used {
			state.increase_esdt_balance(&from, &esdt_token_identifier, &esdt_value);
		}
	}

	Ok((tx_result, tx_output.async_call))
}

fn execute_sc_call_with_async_and_callback(
	tx_input: TxInput,
	state: &mut BlockchainMock,
	contract_map: &ContractMap<TxContext>,
) -> Result<TxResult, BlockchainMockError> {
	let contract_address = tx_input.to.clone();
	let (mut tx_result, opt_async_data) = execute_sc_call(tx_input, state, contract_map)?;
	if tx_result.result_status == 0 {
		if let Some(async_data) = opt_async_data {
			if state.accounts.contains_key(&async_data.to) {
				let async_input = async_call_tx_input(&async_data, &contract_address);

				let async_result =
					execute_sc_call_with_async_and_callback(async_input, state, contract_map)?;

				tx_result = merge_results(tx_result, async_result.clone());

				let callback_input =
					async_callback_tx_input(&async_data, &contract_address, &async_result);
				let (callback_result, opt_more_async) =
					execute_sc_call(callback_input, state, contract_map)?;
				assert!(
					opt_more_async.is_none(),
					"successive asyncs currently not supported"
				);
				tx_result = merge_results(tx_result, callback_result);
			} else {
				state
					.subtract_tx_payment(&contract_address, &async_data.call_value)
					.unwrap();
				state.add_account(AccountData {
					address: async_data.to.clone(),
					nonce: 0,
					balance: async_data.call_value,
					esdt: HashMap::new(),
					username: Vec::new(),
					storage: HashMap::new(),
					contract_path: None,
					contract_owner: None,
				});
			}
		}
	}
	Ok(tx_result)
}

fn execute_sc_create(
	tx_input: TxInput,
	contract_path: &[u8],
	state: &mut BlockchainMock,
	contract_map: &ContractMap<TxContext>,
) -> Result<(TxResult, Option<AsyncCallTxData>), BlockchainMockError> {
	let from = tx_input.from.clone();
	let to = tx_input.to.clone();
	let call_value = tx_input.call_value.clone();
	let blockchain_info = state.create_tx_info(&to);

	state.subtract_tx_payment(&from, &call_value)?;
	state.subtract_tx_gas(&from, tx_input.gas_limit, tx_input.gas_price);

	let esdt_token_identifier = tx_input.esdt_token_identifier.clone();
	let esdt_value = tx_input.esdt_value.clone();
	let esdt_used = !esdt_token_identifier.is_empty() && esdt_value > 0u32.into();

	if esdt_used {
		state.substract_esdt_balance(&from, &esdt_token_identifier, &esdt_value)
	}

	let tx_context = TxContext::new(blockchain_info, tx_input.clone(), TxOutput::default());
	let tx_output = execute_tx(tx_context, contract_path, contract_map);

	if tx_output.result.result_status == 0 {
		let new_address = state.create_account_after_deploy(
			&tx_input,
			tx_output.contract_storage,
			contract_path.to_vec(),
		);
		state.send_balance(&new_address, tx_output.send_balance_list.as_slice())?;
	} else {
		state.increase_balance(&from, &call_value);

		if esdt_used {
			state.increase_esdt_balance(&from, &esdt_token_identifier, &esdt_value);
		}
	}

	Ok((tx_output.result, tx_output.async_call))
}

fn check_tx_output(tx_id: &str, tx_expect: &TxExpect, tx_result: &TxResult) {
	assert_eq!(
		tx_expect.out.len(),
		tx_result.result_values.len(),
		"bad out value. Tx id: {}. Want: {:?}. Have: {:?}",
		tx_id,
		tx_expect.out,
		tx_result.result_values
	);
	for (i, expected_out) in tx_expect.out.iter().enumerate() {
		let actual_value = &tx_result.result_values[i];
		assert!(
			expected_out.check(actual_value.as_slice()),
			"bad out value. Tx id: {}. Want: {}. Have: {}",
			tx_id,
			expected_out,
			verbose_hex(actual_value.as_slice())
		);
	}

	let have_str = std::str::from_utf8(tx_result.result_message.as_slice()).unwrap();
	assert!(
		tx_expect.status.check(tx_result.result_status),
		"result code mismatch. Tx id: {}. Want: {}. Have: {}. Message: {}",
		tx_id,
		tx_expect.status,
		tx_result.result_status,
		have_str,
	);

	assert!(
		tx_expect.message.check(&tx_result.result_message),
		"result message mismatch. Tx id: {}. Want: {}. Have: {}.",
		tx_id,
		&tx_expect.message,
		have_str,
	);

	match &tx_expect.logs {
		CheckLogs::Star => {},
		CheckLogs::List(expected_logs) => {
			assert!(
				expected_logs.len() == tx_result.result_logs.len(),
				"Log amounts do not match. Tx id: {}. Want: {}. Have: {}",
				tx_id,
				expected_logs.len(),
				tx_result.result_logs.len()
			);

			for (expected_log, actual_log) in expected_logs.iter().zip(tx_result.result_logs.iter())
			{
				assert!(
					actual_log.equals(&expected_log),
					"Logs do not match. Tx id: {}.\nWant: Address: {}, Identifier: {}, Topics: {:?}, Data: {}\nHave: Address: {}, Identifier: {}, Topics: {:?}, Data: {}",
					tx_id,
					verbose_hex(&expected_log.address.value),
					bytes_to_string(&expected_log.identifier.value),
					expected_log.topics.iter().map(|topic| verbose_hex(&topic.value)).collect::<String>(),
					verbose_hex(&expected_log.data.value),
					address_hex(&actual_log.address),
					bytes_to_string(&actual_log.identifier),
					actual_log.topics.iter().map(|topic| verbose_hex(&topic)).collect::<String>(),
					verbose_hex(&actual_log.data),
				);
			}
		},
	}
}

fn check_state(accounts: &mandos::CheckAccounts, state: &mut BlockchainMock) {
	for (expected_address, expected_account) in accounts.accounts.iter() {
		if let Some(account) = state.accounts.get(&expected_address.value.into()) {
			assert!(
				expected_account.nonce.check(account.nonce),
				"bad account nonce. Address: {}. Want: {}. Have: {}",
				expected_address,
				expected_account.nonce,
				account.nonce
			);

			assert!(
				expected_account.balance.check(&account.balance),
				"bad account balance. Address: {}. Want: {}. Have: {}",
				expected_address,
				expected_account.balance,
				account.balance
			);

			assert!(
				expected_account.username.check(&account.username),
				"bad account username. Address: {}. Want: {}. Have: {}",
				expected_address,
				expected_account.username,
				std::str::from_utf8(account.username.as_slice()).unwrap()
			);

			if let CheckStorage::Equal(eq) = &expected_account.storage {
				let default_value = &Vec::new();
				for (expected_key, expected_value) in eq.iter() {
					let actual_value = account
						.storage
						.get(&expected_key.value)
						.unwrap_or(default_value);
					assert!(
						expected_value.check(actual_value),
						"bad storage value. Address: {}. Key: {}. Want: {}. Have: {}",
						expected_address,
						expected_key,
						expected_value,
						verbose_hex(actual_value)
					);
				}

				let default_check_value = CheckValue::Equal(BytesValue::empty());
				for (actual_key, actual_value) in account.storage.iter() {
					let expected_value = eq
						.get(&actual_key.clone().into())
						.unwrap_or(&default_check_value);
					assert!(
						expected_value.check(actual_value),
						"bad storage value. Address: {}. Key: {}. Want: {}. Have: {}",
						expected_address,
						verbose_hex(actual_key),
						expected_value,
						verbose_hex(actual_value)
					);
				}
			}

			match &expected_account.esdt {
				CheckEsdt::Equal(eq) => {
					let default_value = &BigUint::from(0u32);
					for (expected_key, expected_value) in eq.iter() {
						let actual_value = account
							.esdt
							.get(&expected_key.value)
							.unwrap_or(default_value);
						assert!(
							expected_value.check(actual_value),
							"bad esdt value. Address: {}. Token Name: {}. Want: {}. Have: {}",
							expected_address,
							expected_key,
							expected_value,
							actual_value
						);
					}

					let default_check_value = CheckValue::Equal(BigUintValue::default());

					for (actual_key, actual_value) in account.esdt.iter() {
						let expected_value = eq
							.get(&actual_key.clone().into())
							.unwrap_or(&default_check_value);
						assert!(
							expected_value.check(actual_value),
							"bad esdt value. Address: {}. Token: {}. Want: {}. Have: {}",
							expected_address,
							verbose_hex(actual_key),
							expected_value,
							actual_value
						);
					}
				},
				CheckEsdt::Star => {
					// nothing to be done for *
				},
			}

			if let CheckEsdt::Equal(eq) = &expected_account.esdt {
				let default_value = &BigUint::from(0u32);
				for (expected_key, expected_value) in eq.iter() {
					let actual_value = account
						.esdt
						.get(&expected_key.value)
						.unwrap_or(default_value);
					assert!(
						expected_value.check(actual_value),
						"bad esdt value. Address: {}. Token Name: {}. Want: {}. Have: {}",
						expected_address,
						expected_key,
						expected_value,
						actual_value
					);
				}

				let default_check_value = CheckValue::Equal(BigUintValue::default());

				for (actual_key, actual_value) in account.esdt.iter() {
					let expected_value = eq
						.get(&actual_key.clone().into())
						.unwrap_or(&default_check_value);
					assert!(
						expected_value.check(actual_value),
						"bad esdt value. Address: {}. Token: {}. Want: {}. Have: {}",
						expected_address,
						verbose_hex(actual_key),
						expected_value,
						actual_value
					);
				}
			}
		} else if !accounts.other_accounts_allowed {
			panic!("Expected account not found");
		}
	}
}

fn generate_tx_hash_dummy(tx_id: &str) -> H256 {
	let bytes = tx_id.as_bytes();
	let mut result = [b'.'; 32];
	if bytes.len() > 32 {
		result[..].copy_from_slice(&bytes[..32]);
	} else {
		result[..bytes.len()].copy_from_slice(bytes);
	}
	result.into()
}

fn update_block_info(block_info: &mut super::BlockInfo, mandos_block_info: &mandos::BlockInfo) {
	if let Some(u64_value) = &mandos_block_info.block_timestamp {
		block_info.block_timestamp = u64_value.value;
	}
	if let Some(u64_value) = &mandos_block_info.block_nonce {
		block_info.block_nonce = u64_value.value;
	}
	if let Some(u64_value) = &mandos_block_info.block_epoch {
		block_info.block_epoch = u64_value.value;
	}
	if let Some(u64_value) = &mandos_block_info.block_round {
		block_info.block_round = u64_value.value;
	}
	if let Some(bytes_value) = &mandos_block_info.block_random_seed {
		const SEED_LEN: usize = 48;
		let val = &bytes_value.value;

		assert!(
			val.len() == SEED_LEN,
			"block random seed input value must be exactly 48 bytes long"
		);

		let mut seed = [0u8; SEED_LEN];
		seed[..].copy_from_slice(val.as_slice());
		block_info.block_random_seed = Box::from(seed);
	}
}
