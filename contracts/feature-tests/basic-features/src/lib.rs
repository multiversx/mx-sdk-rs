#![no_std]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::redundant_clone)]
#![feature(never_type)]

elrond_wasm::imports!();

// this is not part of the standard imports because we want to discourage its use
use elrond_wasm::String;

mod large_boxed_byte_array;
mod ser_ex1;
mod ser_ex2;
mod simple_enum;

use large_boxed_byte_array::LargeBoxedByteArray;
use ser_ex1::*;
use ser_ex2::*;
use simple_enum::*;

use core::num::NonZeroUsize;

#[elrond_wasm_derive::contract(BasicFeaturesImpl)]
pub trait BasicFeatures {
	#[init]
	fn init(&self) {}

	#[endpoint(panicWithMessage)]
	fn panic_with_message(&self) {
		panic!("example panic message");
	}

	// TEST ARGUMENT AND RETURN TYPE SERIALIZATION

	#[endpoint]
	fn echo_big_uint(&self, bi: BigUint) -> BigUint {
		bi
	}

	#[endpoint]
	fn echo_big_int(&self, bi: BigInt) -> BigInt {
		bi
	}

	#[endpoint]
	fn echo_u64(&self, i: u64) -> u64 {
		i
	}

	#[endpoint]
	fn echo_i64(&self, i: i64) -> i64 {
		i
	}

	#[endpoint]
	fn echo_i32(&self, i: i32) -> i32 {
		i
	}

	#[endpoint]
	fn echo_u32(&self, i: u32) -> u32 {
		i
	}

	#[endpoint]
	fn echo_isize(&self, i: isize) -> isize {
		i
	}

	#[endpoint]
	fn echo_usize(&self, i: usize) -> usize {
		i
	}

	#[endpoint]
	fn echo_i8(&self, i: i8) -> i8 {
		i
	}

	#[endpoint]
	fn echo_u8(&self, i: u8) -> u8 {
		i
	}

	#[endpoint]
	fn echo_bool(&self, i: bool) -> bool {
		i
	}

	#[endpoint]
	fn echo_opt_bool(&self, i: Option<bool>) -> Option<bool> {
		i
	}

	#[endpoint]
	fn echo_h256(&self, h: H256) -> H256 {
		h
	}

	#[endpoint]
	fn echo_nothing(&self, #[var_args] nothing: ()) -> () {
		nothing
	}

	#[endpoint]
	fn echo_array_u8(&self, s: [u8; 5]) -> [u8; 5] {
		s
	}

	#[endpoint]
	fn echo_boxed_array_u8(&self, s: Box<[u8; 128]>) -> Box<[u8; 128]> {
		s
	}

	#[endpoint]
	fn echo_boxed_bytes(&self, arg: BoxedBytes) -> MultiResult2<BoxedBytes, usize> {
		let l = arg.len();
		(arg, l).into()
	}

	#[endpoint]
	fn echo_slice_u8<'s>(&self, slice: &'s [u8]) -> MultiResult2<&'s [u8], usize> {
		let l = slice.len();
		(slice, l).into()
	}

	#[endpoint]
	fn echo_vec_u8(&self, arg: Vec<u8>) -> MultiResult2<Vec<u8>, usize> {
		let l = arg.len();
		(arg, l).into()
	}

	#[endpoint]
	fn echo_string(&self, s: String) -> MultiResult2<String, usize> {
		let l = s.len();
		(s, l).into()
	}

	#[endpoint]
	fn echo_str<'s>(&self, s: &'s str) -> MultiResult2<&'s str, usize> {
		let l = s.len();
		(s, l).into()
	}

	#[endpoint]
	fn echo_str_box(&self, s: Box<str>) -> MultiResult2<Box<str>, usize> {
		let l = s.len();
		(s, l).into()
	}

	#[endpoint]
	fn echo_varags_u32(
		&self,
		#[var_args] m: VarArgs<u32>,
	) -> MultiResult2<usize, MultiResultVec<u32>> {
		let v = m.into_vec();
		(v.len(), v.into()).into()
	}

	#[endpoint]
	fn take_varags_u32(&self, #[var_args] m: VarArgs<u32>) -> usize {
		let v = m.into_vec();
		v.len()
	}

	#[endpoint]
	fn echo_varags_big_uint(&self, #[var_args] m: VarArgs<BigUint>) -> MultiResultVec<BigUint> {
		m.into_vec().into()
	}

	#[endpoint]
	fn echo_varags_tuples(
		&self,
		#[var_args] m: VarArgs<MultiArg2<isize, Vec<u8>>>,
	) -> MultiResultVec<MultiResult2<isize, Vec<u8>>> {
		let mut result: Vec<MultiResult2<isize, Vec<u8>>> = Vec::new();
		for m_arg in m.into_vec().into_iter() {
			result.push(m_arg.into_tuple().into())
		}
		result.into()
	}

	#[endpoint]
	fn echo_async_result_empty(&self, #[var_args] a: AsyncCallResult<()>) -> SCResult<()> {
		match a {
			AsyncCallResult::Ok(()) => Ok(()),
			AsyncCallResult::Err(msg) => Err(msg.err_msg.into()),
		}
	}

	#[endpoint]
	fn echo_large_boxed_byte_array(&self, lbba: LargeBoxedByteArray) -> LargeBoxedByteArray {
		lbba
	}

	#[endpoint]
	fn echo_ser_example_1(&self, se: SerExample1) -> SerExample1 {
		se
	}

	#[endpoint]
	fn echo_boxed_ser_example_1(&self, se: Box<SerExample1>) -> Box<SerExample1> {
		se
	}

	#[endpoint]
	fn echo_ser_example_2(&self, se: SerExample2) -> SerExample2 {
		se
	}

	#[endpoint]
	fn echo_boxed_ser_example_2(&self, se: Box<SerExample2>) -> Box<SerExample2> {
		se
	}

	#[view]
	fn echo_simple_enum(&self, se: SimpleEnum) -> SimpleEnum {
		se
	}

	#[view]
	fn finish_simple_enum_variant_1(&self) -> SimpleEnum {
		SimpleEnum::Variant1
	}

	#[view]
	fn echo_non_zero_usize(&self, nz: NonZeroUsize) -> NonZeroUsize {
		nz
	}

	// OPERATIONS THAT HAVE CAUSED ISSUES IN THE PAST

	#[endpoint]
	fn count_ones(&self, arg: u64) -> u32 {
		arg.count_ones()
	}

	// STORAGE STORE

	#[endpoint]
	#[storage_set("big_uint")]
	fn store_big_uint(&self, bi: BigUint);

	#[endpoint]
	#[storage_set("big_int")]
	fn store_big_int(&self, bi: BigInt);

	#[endpoint]
	#[storage_set("usize")]
	fn store_usize(&self, i: usize);

	#[endpoint]
	#[storage_set("i32")]
	fn store_i32(&self, i: i32);

	#[endpoint]
	#[storage_set("u64")]
	fn store_u64(&self, i: u64);

	#[endpoint]
	#[storage_set("i64")]
	fn store_i64(&self, i: i64);

	#[endpoint]
	#[storage_set("bool")]
	fn store_bool(&self, i: bool);

	#[endpoint]
	#[storage_set("vec_u8")]
	fn store_vec_u8(&self, arg: Vec<u8>);

	#[endpoint]
	#[storage_set("addr")]
	fn store_addr(&self, arg: Address);

	#[storage_set("opt_addr")]
	fn _set_opt_addr(&self, opt_addr: Option<Address>);

	#[endpoint]
	fn store_opt_addr(&self, #[var_args] opt_addr: OptionalArg<Address>) {
		self._set_opt_addr(opt_addr.into_option());
	}

	#[endpoint]
	#[storage_set("ser_1")]
	fn store_ser_1(&self, arg: SerExample1);

	#[endpoint]
	#[storage_set("ser_2")]
	fn store_ser_2(&self, arg: SerExample2);

	#[endpoint]
	#[storage_set("map1")]
	fn store_map1(&self, addr: Address, bi: BigUint);

	#[endpoint]
	#[storage_set("map2")]
	fn store_map2(&self, addr1: &Address, addr2: &Address, bi: &BigUint);

	#[endpoint]
	#[storage_set("map3")]
	fn store_map3(&self, x: usize, b: bool);

	#[storage_set("slice1")]
	fn store_slice1(&self, slice: &[BigUint]);

	#[endpoint]
	#[storage_set("ELRONDi64")]
	fn store_reserved_i64(&self, i: i64);

	#[endpoint]
	#[storage_set("ELRONDBigUint")]
	fn store_reserved_big_uint(&self, i: BigUint);

	#[endpoint]
	#[storage_set("ELRONDreserved")]
	fn store_reserved_vec_u8(&self, i: Vec<u8>);

	// STORAGE LOAD

	#[endpoint]
	#[storage_get("big_uint")]
	fn load_big_uint(&self) -> BigUint;

	#[endpoint]
	#[storage_get("big_int")]
	fn load_big_int(&self) -> BigInt;

	#[endpoint]
	#[storage_get("u64")]
	fn load_u64(&self) -> u64;

	#[endpoint]
	#[storage_get("usize")]
	fn load_usize(&self) -> usize;

	#[endpoint]
	#[storage_get("i64")]
	fn load_i64(&self) -> i64;

	#[endpoint]
	#[storage_get("bool")]
	fn load_bool(&self) -> bool;

	#[endpoint]
	#[storage_get("vec_u8")]
	fn load_vec_u8(&self) -> Vec<u8>;

	#[endpoint]
	#[storage_get("addr")]
	fn load_addr(&self) -> Address;

	#[storage_get("opt_addr")]
	fn _get_opt_addr(&self) -> Option<Address>;

	#[endpoint]
	fn load_opt_addr(&self) -> OptionalResult<Address> {
		self._get_opt_addr().into()
	}

	#[endpoint(storage_load_cumulated_validator_reward)]
	fn storage_load_cumulated_validator_reward_endpoint(&self) -> BigUint {
		self.storage_load_cumulated_validator_reward()
	}

	#[view]
	#[storage_is_empty("opt_addr")]
	fn is_empty_opt_addr(&self) -> bool;

	#[endpoint]
	#[storage_get("nr_to_clear")]
	fn get_nr_to_clear(&self) -> u32;

	#[endpoint]
	#[storage_clear("nr_to_clear")]
	fn clear_storage_value(&self);

	#[endpoint]
	#[storage_get("ser_1")]
	fn load_ser_1(&self) -> SerExample1;

	#[endpoint]
	#[storage_get("ser_2")]
	fn load_ser_2(&self) -> SerExample2;

	#[endpoint]
	#[storage_get("map1")]
	fn load_map1(&self, addr: Address) -> BigUint;

	#[endpoint]
	#[storage_get("map2")]
	fn load_map2(&self, addr1: &Address, addr2: &Address) -> BigUint;

	#[endpoint]
	#[storage_get("map3")]
	fn load_map3(&self, x: usize) -> bool;

	// STORAGE MAPPERS

	#[view]
	#[storage_mapper("my_single_value_mapper")]
	fn map_my_single_value_mapper(&self) -> SingleValueMapper<Self::Storage, BigInt>;

	#[endpoint]
	fn my_single_value_mapper_increment_1(&self, amount: BigInt) {
		let my_single_value_mapper = self.map_my_single_value_mapper();
		my_single_value_mapper.set(&(my_single_value_mapper.get() + amount));
	}

	/// Same as my_single_value_mapper_increment_1, but expressed more compactly.
	#[endpoint]
	fn my_single_value_mapper_increment_2(&self, amount: &BigInt) {
		self.map_my_single_value_mapper()
			.update(|value| *value += amount);
	}

	#[endpoint]
	fn clear_single_value_mapper(&self) {
		self.map_my_single_value_mapper().clear();
	}

	#[endpoint]
	fn is_empty_single_value_mapper(&self) -> bool {
		self.map_my_single_value_mapper().is_empty()
	}

	// VecMapper

	#[view]
	#[storage_mapper("vec_mapper")]
	fn vec_mapper(&self) -> VecMapper<Self::Storage, u32>;

	#[endpoint]
	fn vec_mapper_push(&self, item: u32) {
		let mut vec_mapper = self.vec_mapper();
		let _ = vec_mapper.push(&item);
	}

	#[view]
	fn vec_mapper_get(&self, index: usize) -> u32 {
		self.vec_mapper().get(index)
	}

	#[view]
	fn vec_mapper_len(&self) -> usize {
		self.vec_mapper().len()
	}

	// LinkedListMapper

	#[view]
	#[storage_mapper("list_mapper")]
	fn list_mapper(&self) -> LinkedListMapper<Self::Storage, u32>;

	#[endpoint]
	fn list_mapper_push_back(&self, item: u32) {
		let mut list_mapper = self.list_mapper();
		list_mapper.push_back(item);
	}

	#[endpoint]
	fn list_mapper_pop_front(&self) -> Option<u32> {
		let mut list_mapper = self.list_mapper();
		list_mapper.pop_front()
	}

	#[endpoint]
	fn list_mapper_front(&self) -> SCResult<u32> {
		if let Some(front) = self.list_mapper().front() {
			return Ok(front);
		}
		sc_error!("List empty!")
	}

	// SetMapper

	#[view]
	#[storage_mapper("set_mapper")]
	fn set_mapper(&self) -> SetMapper<Self::Storage, u32>;

	#[endpoint]
	fn set_mapper_insert(&self, item: u32) -> bool {
		let mut set_mapper = self.set_mapper();
		set_mapper.insert(item)
	}

	#[endpoint]
	fn set_mapper_contains(&self, item: u32) -> bool {
		let set_mapper = self.set_mapper();
		set_mapper.contains(&item)
	}

	#[endpoint]
	fn set_mapper_remove(&self, item: u32) -> bool {
		let mut set_mapper = self.set_mapper();
		set_mapper.remove(&item)
	}

	// MapMapper

	#[storage_mapper("map_mapper")]
	fn map_mapper(&self) -> MapMapper<Self::Storage, u32, u32>;

	#[view]
	fn map_mapper_keys(&self) -> MultiResultVec<u32> {
		self.map_mapper().keys().collect()
	}

	#[view]
	fn map_mapper_values(&self) -> MultiResultVec<u32> {
		self.map_mapper().values().collect()
	}

	#[endpoint]
	fn map_mapper_insert(&self, item: u32, value: u32) -> Option<u32> {
		let mut map_mapper = self.map_mapper();
		map_mapper.insert(item, value)
	}

	#[endpoint]
	fn map_mapper_contains_key(&self, item: u32) -> bool {
		let map_mapper = self.map_mapper();
		map_mapper.contains_key(&item)
	}

	#[endpoint]
	fn map_mapper_get(&self, item: u32) -> Option<u32> {
		let map_mapper = self.map_mapper();
		map_mapper.get(&item)
	}

	#[endpoint]
	fn map_mapper_remove(&self, item: u32) -> Option<u32> {
		let mut map_mapper = self.map_mapper();
		map_mapper.remove(&item)
	}

	// MapStorageMapper

	#[storage_mapper("map_storage_mapper")]
	fn map_storage_mapper(
		&self,
	) -> MapStorageMapper<Self::Storage, u32, MapMapper<Self::Storage, u32, u32>>;

	#[view]
	fn map_storage_mapper_view(&self) -> MultiResultVec<u32> {
		let mut vec: Vec<u32> = Vec::new();
		for (key1, map) in self.map_storage_mapper().iter() {
			for (key2, value) in map.iter() {
				vec.push(key1);
				vec.push(key2);
				vec.push(value);
			}
		}
		MultiResultVec::from(vec)
	}

	#[endpoint]
	fn map_storage_mapper_insert_default(&self, item: u32) -> bool {
		let mut map_storage_mapper = self.map_storage_mapper();
		map_storage_mapper.insert_default(item)
	}

	#[endpoint]
	fn map_storage_mapper_contains_key(&self, item: u32) -> bool {
		let map_storage_mapper = self.map_storage_mapper();
		map_storage_mapper.contains_key(&item)
	}

	#[endpoint]
	fn map_storage_mapper_get(&self, item: u32) -> SCResult<MultiResultVec<u32>> {
		let map_storage_mapper = self.map_storage_mapper();
		if let Some(map) = map_storage_mapper.get(&item) {
			let mut vec = Vec::new();
			for (key, value) in map.iter() {
				vec.push(key);
				vec.push(value);
			}
			return Ok(MultiResultVec::from(vec));
		}
		sc_error!("No storage!")
	}

	#[endpoint]
	fn map_storage_mapper_insert_value(
		&self,
		item: u32,
		key: u32,
		value: u32,
	) -> SCResult<Option<u32>> {
		let map_storage_mapper = self.map_storage_mapper();
		if let Some(mut map) = map_storage_mapper.get(&item) {
			return Ok(map.insert(key, value));
		}
		sc_error!("No storage!")
	}

	#[endpoint]
	fn map_storage_mapper_get_value(&self, item: u32, key: u32) -> SCResult<Option<u32>> {
		let map_storage_mapper = self.map_storage_mapper();
		if let Some(map) = map_storage_mapper.get(&item) {
			return Ok(map.get(&key));
		}
		sc_error!("No storage!")
	}

	#[endpoint]
	fn map_storage_mapper_remove(&self, item: u32) -> bool {
		let mut map_storage_mapper = self.map_storage_mapper();
		map_storage_mapper.remove(&item)
	}

	#[endpoint]
	fn map_storage_mapper_clear(&self) {
		let mut map_storage_mapper = self.map_storage_mapper();
		map_storage_mapper.clear();
	}

	// BASIC API
	#[endpoint(get_caller)]
	fn get_caller_endpoint(&self) -> Address {
		self.get_caller()
	}

	#[endpoint(get_shard_of_address)]
	fn get_shard_of_address_endpoint(&self, address: &Address) -> u32 {
		self.get_shard_of_address(address)
	}

	#[endpoint(is_smart_contract)]
	fn is_smart_contract_endpoint(&self, address: &Address) -> bool {
		self.is_smart_contract(address)
	}

	#[endpoint(get_gas_left)]
	fn get_gas_left_endpoint(&self) -> u64 {
		self.get_gas_left()
	}

	// EVENTS

	#[endpoint(logEventA)]
	fn log_event_a(&self, data: &BigUint) {
		self.event_a(data);
	}

	#[event("event_a")]
	fn event_a(&self, data: &BigUint);

	#[endpoint(logEventB)]
	fn log_event_b(&self, arg1: &BigUint, arg2: &Address, #[var_args] data: VarArgs<BoxedBytes>) {
		self.event_b(arg1, arg2, data.as_slice());
	}

	#[event("event_b")]
	fn event_b(&self, #[indexed] arg1: &BigUint, #[indexed] arg2: &Address, data: &[BoxedBytes]);

	// EVENTS (LEGACY)

	#[endpoint(logLegacyEventA)]
	fn log_legacy_event_a(&self, data: &BigUint) {
		self.legacy_event_a(data);
	}

	#[endpoint(logLegacyEventB)]
	fn log_legacy_event_b(&self, arg1: &BigUint, arg2: &Address, data: &BigUint) {
		self.legacy_event_b(arg1, arg2, data);
	}

	#[legacy_event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000a")]
	fn legacy_event_a(&self, data: &BigUint);

	#[legacy_event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000b")]
	fn legacy_event_b(&self, arg1: &BigUint, arg2: &Address, data: &BigUint);

	// BLOCK INFO

	#[view(get_block_timestamp)]
	fn get_block_timestamp_view(&self) -> u64 {
		self.get_block_timestamp()
	}

	#[view(get_block_nonce)]
	fn get_block_nonce_view(&self) -> u64 {
		self.get_block_nonce()
	}

	#[view(get_block_round)]
	fn get_block_round_view(&self) -> u64 {
		self.get_block_round()
	}

	#[view(get_block_epoch)]
	fn get_block_epoch_view(&self) -> u64 {
		self.get_block_epoch()
	}

	#[view(get_block_random_seed)]
	fn get_block_random_seed_view(&self) -> Box<[u8; 48]> {
		self.get_block_random_seed()
	}

	#[view(get_prev_block_timestamp)]
	fn get_prev_block_timestamp_view(&self) -> u64 {
		self.get_prev_block_timestamp()
	}

	#[view(get_prev_block_nonce)]
	fn get_prev_block_nonce_view(&self) -> u64 {
		self.get_prev_block_nonce()
	}

	#[view(get_prev_block_round)]
	fn get_prev_block_round_view(&self) -> u64 {
		self.get_prev_block_round()
	}

	#[view(get_prev_block_epoch)]
	fn get_prev_block_epoch_view(&self) -> u64 {
		self.get_prev_block_epoch()
	}

	#[view(get_prev_block_random_seed)]
	fn get_prev_block_random_seed_view(&self) -> Box<[u8; 48]> {
		self.get_prev_block_random_seed()
	}

	// BIG INT OPERATIONS

	// arithmetic ooperators: + - * / %
	#[endpoint]
	fn add_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		a + b
	}
	#[endpoint]
	fn add_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		a + b
	}
	#[endpoint]
	fn add_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a + b
	}
	#[endpoint]
	fn add_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a + b
	}
	#[endpoint]
	fn sub_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		a - b
	}
	#[endpoint]
	fn sub_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		a - b
	}
	#[endpoint]
	fn sub_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a - b
	}
	#[endpoint]
	fn sub_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a - b
	}
	#[endpoint]
	fn mul_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		a * b
	}
	#[endpoint]
	fn mul_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		a * b
	}
	#[endpoint]
	fn mul_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a * b
	}
	#[endpoint]
	fn mul_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a * b
	}
	#[endpoint]
	fn div_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		a / b
	}
	#[endpoint]
	fn div_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		a / b
	}
	#[endpoint]
	fn div_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a / b
	}
	#[endpoint]
	fn div_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a / b
	}
	#[endpoint]
	fn rem_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		a % b
	}
	#[endpoint]
	fn rem_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		a % b
	}
	#[endpoint]
	fn rem_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a % b
	}
	#[endpoint]
	fn rem_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a % b
	}

	// assign version of all operators above
	#[endpoint]
	fn add_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		let mut r = a.clone();
		r += b;
		r
	}
	#[endpoint]
	fn add_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		let mut r = a.clone();
		r += b;
		r
	}
	#[endpoint]
	fn add_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r += b;
		r
	}
	#[endpoint]
	fn add_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r += b;
		r
	}
	#[endpoint]
	fn sub_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		let mut r = a.clone();
		r -= b;
		r
	}
	#[endpoint]
	fn sub_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		let mut r = a.clone();
		r -= b;
		r
	}
	#[endpoint]
	fn sub_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r -= b;
		r
	}
	#[endpoint]
	fn sub_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r -= b;
		r
	}
	#[endpoint]
	fn mul_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		let mut r = a.clone();
		r *= b;
		r
	}
	#[endpoint]
	fn mul_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		let mut r = a.clone();
		r *= b;
		r
	}
	#[endpoint]
	fn mul_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r *= b;
		r
	}
	#[endpoint]
	fn mul_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r *= b;
		r
	}
	#[endpoint]
	fn div_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		let mut r = a.clone();
		r /= b;
		r
	}
	#[endpoint]
	fn div_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		let mut r = a.clone();
		r /= b;
		r
	}
	#[endpoint]
	fn div_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r /= b;
		r
	}
	#[endpoint]
	fn div_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r /= b;
		r
	}
	#[endpoint]
	fn rem_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
		let mut r = a.clone();
		r %= b;
		r
	}
	#[endpoint]
	fn rem_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
		let mut r = a.clone();
		r %= b;
		r
	}
	#[endpoint]
	fn rem_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r %= b;
		r
	}
	#[endpoint]
	fn rem_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r %= b;
		r
	}

	#[endpoint]
	fn bit_and_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a & b
	}
	#[endpoint]
	fn bit_and_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a & b
	}
	#[endpoint]
	fn bit_or_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a | b
	}
	#[endpoint]
	fn bit_or_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a | b
	}
	#[endpoint]
	fn bit_xor_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		a ^ b
	}
	#[endpoint]
	fn bit_xor_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		a ^ b
	}

	#[endpoint]
	fn bit_and_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r &= b;
		r
	}
	#[endpoint]
	fn bit_and_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r &= b;
		r
	}
	#[endpoint]
	fn bit_or_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r |= b;
		r
	}
	#[endpoint]
	fn bit_or_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r |= b;
		r
	}
	#[endpoint]
	fn bit_xor_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
		let mut r = a.clone();
		r ^= b;
		r
	}
	#[endpoint]
	fn bit_xor_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
		let mut r = a.clone();
		r ^= b;
		r
	}

	#[endpoint]
	fn shr_big_uint(&self, a: BigUint, b: usize) -> BigUint {
		a >> b
	}
	#[endpoint]
	fn shr_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
		a >> b
	}
	#[endpoint]
	fn shl_big_uint(&self, a: BigUint, b: usize) -> BigUint {
		a << b
	}
	#[endpoint]
	fn shl_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
		a << b
	}

	#[endpoint]
	fn shr_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint {
		let mut r = a.clone();
		r >>= b;
		r
	}
	#[endpoint]
	fn shr_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
		let mut r = a.clone();
		r >>= b;
		r
	}
	#[endpoint]
	fn shl_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint {
		let mut r = a.clone();
		r <<= b;
		r
	}
	#[endpoint]
	fn shl_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
		let mut r = a.clone();
		r <<= b;
		r
	}

	// MORE H256

	#[endpoint]
	fn compare_h256(&self, h1: H256, h2: H256) -> bool {
		h1 == h2
	}

	#[endpoint]
	fn h256_is_zero(&self, h: H256) -> bool {
		h.is_zero()
	}

	// BOXED BYTES

	#[endpoint]
	fn boxed_bytes_zeros(&self, len: usize) -> BoxedBytes {
		BoxedBytes::zeros(len)
	}

	#[endpoint]
	fn boxed_bytes_concat_2(&self, slice1: &[u8], slice2: &[u8]) -> BoxedBytes {
		BoxedBytes::from_concat(&[slice1, slice2][..])
	}

	#[endpoint]
	fn boxed_bytes_split(&self, bb: BoxedBytes, at: usize) -> MultiResult2<BoxedBytes, BoxedBytes> {
		bb.split(at).into()
	}

	// VEC OPERATIONS

	#[view]
	fn vec_concat_const(&self) -> Vec<u8> {
		let mut result = b"part1".to_vec();
		result.extend_from_slice(&[0u8; 100][..]);
		result
	}

	// NON ZERO EXTRA

	#[view]
	fn non_zero_usize_iter(&self, how_many: usize) -> MultiResultVec<NonZeroUsize> {
		let mut result = Vec::<NonZeroUsize>::new();
		for nz in NonZeroUsizeIterator::from_1_to_n(how_many) {
			result.push(nz);
		}
		result.into()
	}

	#[view]
	fn non_zero_usize_macro(&self, number: usize) -> SCResult<NonZeroUsize> {
		let nz = non_zero_usize!(number, "wans non-zero");
		Ok(nz)
	}

	// CRYPTO FUNCTIONS

	#[endpoint(computeSha256)]
	fn compute_sha256(&self, input: Vec<u8>) -> H256 {
		self.sha256(&input)
	}

	#[endpoint(computeKeccak256)]
	fn compute_keccak256(&self, input: Vec<u8>) -> H256 {
		self.keccak256(&input)
	}

	// Not called, they currently just panic with "Not implemented yet!"

	#[endpoint]
	fn verify_bls_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		self.verify_bls(key, message, signature)
	}

	#[endpoint]
	fn verify_ed25519_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		self.verify_ed25519(key, message, signature)
	}

	#[endpoint]
	fn verify_secp256k1_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		self.verify_secp256k1(key, message, signature)
	}

	// MACROS

	#[view]
	fn only_owner(&self) -> SCResult<()> {
		only_owner!(self, "Caller must be owner");
		Ok(())
	}

	#[view]
	fn require_equals(&self, a: u32, b: u32) -> SCResult<()> {
		require!(a == b, "a must equal b");
		Ok(())
	}

	#[view]
	fn return_sc_error(&self) -> SCResult<()> {
		sc_error!("return_sc_error")
	}

	#[view]
	fn result_ok(&self) -> Result<(), !> {
		Result::Ok(())
	}

	#[view]
	fn result_err_from_bytes_1(&self, e: BoxedBytes) -> Result<(), BoxedBytes> {
		Result::Err(e)
	}

	#[view]
	fn result_err_from_bytes_2<'a>(&self, e: &'a [u8]) -> Result<(), &'a [u8]> {
		Result::Err(e)
	}

	#[view]
	fn result_err_from_bytes_3(&self, e: Vec<u8>) -> Result<(), Vec<u8>> {
		Result::Err(e)
	}

	#[view]
	fn result_err_from_string(&self, e: String) -> Result<(), String> {
		Result::Err(e)
	}

	#[view]
	fn result_err_from_str<'a>(&self, e: &'a str) -> Result<(), &'a str> {
		Result::Err(e)
	}
}
