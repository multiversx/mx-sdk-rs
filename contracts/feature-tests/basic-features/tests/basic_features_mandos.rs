use basic_features::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/basic-features.wasm",
		Box::new(|context| Box::new(BasicFeaturesImpl::new(context))),
	);
	contract_map
}

#[test]
fn block_info() {
	parse_execute_mandos("mandos/block_info.scen.json", &contract_map());
}

#[test]
fn boxed_bytes_zeros() {
	parse_execute_mandos("mandos/boxed_bytes_zeros.scen.json", &contract_map());
}

#[test]
fn count_ones() {
	parse_execute_mandos("mandos/count_ones.scen.json", &contract_map());
}

#[test]
fn crypto_keccak256() {
	parse_execute_mandos("mandos/crypto_keccak256.scen.json", &contract_map());
}

#[test]
fn crypto_sha256() {
	parse_execute_mandos("mandos/crypto_sha256.scen.json", &contract_map());
}

#[test]
fn echo_array_u8() {
	parse_execute_mandos("mandos/echo_array_u8.scen.json", &contract_map());
}

#[test]
fn echo_async_result_empty() {
	parse_execute_mandos("mandos/echo_async_result_empty.scen.json", &contract_map());
}

#[test]
fn echo_big_int() {
	parse_execute_mandos("mandos/echo_big_int.scen.json", &contract_map());
}

#[test]
fn echo_big_uint() {
	parse_execute_mandos("mandos/echo_big_uint.scen.json", &contract_map());
}

#[test]
fn echo_boxed_bytes() {
	parse_execute_mandos("mandos/echo_boxed_bytes.scen.json", &contract_map());
}

#[test]
fn echo_i32() {
	parse_execute_mandos("mandos/echo_i32.scen.json", &contract_map());
}

#[test]
fn echo_i64() {
	parse_execute_mandos("mandos/echo_i64.scen.json", &contract_map());
}

#[test]
fn echo_nothing() {
	parse_execute_mandos("mandos/echo_nothing.scen.json", &contract_map());
}

#[test]
fn echo_ser_ex_1() {
	parse_execute_mandos("mandos/echo_ser_ex_1.scen.json", &contract_map());
}

#[test]
fn echo_slice_u8() {
	parse_execute_mandos("mandos/echo_slice_u8.scen.json", &contract_map());
}

#[test]
fn echo_str() {
	parse_execute_mandos("mandos/echo_str.scen.json", &contract_map());
}

#[test]
fn echo_str_box() {
	parse_execute_mandos("mandos/echo_str_box.scen.json", &contract_map());
}

#[test]
fn echo_string() {
	parse_execute_mandos("mandos/echo_string.scen.json", &contract_map());
}

#[test]
fn echo_u64() {
	parse_execute_mandos("mandos/echo_u64.scen.json", &contract_map());
}

#[test]
fn echo_usize() {
	parse_execute_mandos("mandos/echo_usize.scen.json", &contract_map());
}

#[test]
fn echo_varags_tuples() {
	parse_execute_mandos("mandos/echo_varags_tuples.scen.json", &contract_map());
}

#[test]
fn echo_varargs_u32() {
	parse_execute_mandos("mandos/echo_varargs_u32.scen.json", &contract_map());
}

#[test]
fn echo_vec_u8() {
	parse_execute_mandos("mandos/echo_vec_u8.scen.json", &contract_map());
}

#[test]
fn events() {
	parse_execute_mandos("mandos/events.scen.json", &contract_map());
}

#[test]
fn events_legacy() {
	parse_execute_mandos("mandos/events_legacy.scen.json", &contract_map());
}

// TODO: fix, by first implementing scQuery
// #[test]
// fn get_caller() {
// 	parse_execute_mandos("mandos/get_caller.scen.json", &contract_map());
// }

// TODO: fix, by first implementing scQuery
// #[test]
// fn is_smart_contract() {
// 	parse_execute_mandos("mandos/is_smart_contract.scen.json", &contract_map());
// }

#[test]
fn panic() {
	parse_execute_mandos("mandos/panic.scen.json", &contract_map());
}

#[test]
fn return_codes() {
	parse_execute_mandos("mandos/return_codes.scen.json", &contract_map());
}

#[test]
fn return_error() {
	parse_execute_mandos("mandos/return_error.scen.json", &contract_map());
}

#[test]
fn storage_addr() {
	parse_execute_mandos("mandos/storage_addr.scen.json", &contract_map());
}

#[test]
fn storage_big_int() {
	parse_execute_mandos("mandos/storage_big_int.scen.json", &contract_map());
}

#[test]
fn storage_big_uint() {
	parse_execute_mandos("mandos/storage_big_uint.scen.json", &contract_map());
}

#[test]
fn storage_bool() {
	parse_execute_mandos("mandos/storage_bool.scen.json", &contract_map());
}

#[test]
fn storage_clear() {
	parse_execute_mandos("mandos/storage_clear.scen.json", &contract_map());
}

#[test]
fn storage_i64() {
	parse_execute_mandos("mandos/storage_i64.scen.json", &contract_map());
}

#[test]
fn storage_i64_bad() {
	parse_execute_mandos("mandos/storage_i64_bad.scen.json", &contract_map());
}

#[test]
fn storage_load_cumulated_validator_reward() {
	parse_execute_mandos(
		"mandos/storage_load_cumulated_validator_reward.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_map1() {
	parse_execute_mandos("mandos/storage_map1.scen.json", &contract_map());
}

#[test]
fn storage_map2() {
	parse_execute_mandos("mandos/storage_map2.scen.json", &contract_map());
}

#[test]
fn storage_map3() {
	parse_execute_mandos("mandos/storage_map3.scen.json", &contract_map());
}

#[test]
fn storage_mapper_linked_list() {
	parse_execute_mandos(
		"mandos/storage_mapper_linked_list.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_map() {
	parse_execute_mandos("mandos/storage_mapper_map.scen.json", &contract_map());
}

#[test]
fn storage_mapper_set() {
	parse_execute_mandos("mandos/storage_mapper_set.scen.json", &contract_map());
}

#[test]
fn storage_mapper_single_value() {
	parse_execute_mandos(
		"mandos/storage_mapper_single_value.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_vec() {
	parse_execute_mandos("mandos/storage_mapper_vec.scen.json", &contract_map());
}

#[test]
fn storage_opt_addr() {
	parse_execute_mandos("mandos/storage_opt_addr.scen.json", &contract_map());
}

#[test]
fn storage_reserved() {
	parse_execute_mandos("mandos/storage_reserved.scen.json", &contract_map());
}

#[test]
fn storage_u64() {
	parse_execute_mandos("mandos/storage_u64.scen.json", &contract_map());
}

#[test]
fn storage_u64_bad() {
	parse_execute_mandos("mandos/storage_u64_bad.scen.json", &contract_map());
}

#[test]
fn storage_usize() {
	parse_execute_mandos("mandos/storage_usize.scen.json", &contract_map());
}

#[test]
fn storage_usize_bad() {
	parse_execute_mandos("mandos/storage_usize_bad.scen.json", &contract_map());
}

#[test]
fn storage_vec_u8() {
	parse_execute_mandos("mandos/storage_vec_u8.scen.json", &contract_map());
}
