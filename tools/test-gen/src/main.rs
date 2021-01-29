// Just a quick and dirty way to auto-generate some mandos tests on the Rust side.
// Will get converted to a proper tool at some point.

fn print_mandos_tests(names: &[&str]) {
	for name in names.iter() {
		print!(
			"
#[test]
fn {}() {{
    parse_execute_mandos(\"mandos/{}.scen.json\", &contract_map());
}}
",
			name.replace('-', "_").to_lowercase(),
			name
		);
	}
}

fn main() {
	print_mandos_tests(&[
		"block_info",
		"boxed_bytes_zeros",
		"count_ones",
		"crypto",
		"echo_array_u8",
		"echo_async_result_empty",
		"echo_big_int",
		"echo_big_uint",
		"echo_boxed_bytes",
		"echo_i32",
		"echo_i64",
		"echo_multi_i32",
		"echo_nothing",
		"echo_slice_u8",
		"echo_str_box",
		"echo_string",
		"echo_str",
		"echo_u64",
		"echo_usize",
		"echo_varags_tuples",
		"echo_varargs_u32",
		"echo_vec_u8",
		"eventA1",
		"eventA2",
		"eventB1",
		"out_of_gas",
		"panic",
		"payable_any_1",
		"payable_any_2",
		"payable_any_3",
		"payable_any_4",
		"payable_egld_0",
		"payable_egld_1",
		"payable_egld_2",
		"payable_egld_3",
		"payable_egld_4",
		"payable_token_1",
		"payable_token_2",
		"payable_token_3",
		"payable_token_4",
		"return_error",
		"send_tx",
		"storage_addr",
		"storage_big_int",
		"storage_big_uint",
		"storage_bool",
		"storage_clear",
		"storage_i64_bad",
		"storage_i64",
		"storage_load_cumulated_validator_reward",
		"storage_map1",
		"storage_map2",
		"storage_map3",
		"storage_mapper_single_value",
		"storage_mapper_vec",
		"storage_opt_addr",
		"storage_reserved",
		"storage_u64_bad",
		"storage_u64",
		"storage_usize_bad",
		"storage_usize",
		"storage_vec_u8",
	]);
}
