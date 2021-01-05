pub fn big_int_where() -> proc_macro2::TokenStream {
	quote! {
		where
			BigUint: BigUintApi + 'static,
			for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output=BigUint>,
			for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output=BigUint>,
			for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output=BigUint>,
			for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output=BigUint>,
			for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output=BigUint>,
			for<'b> BigUint: AddAssign<&'b BigUint>,
			for<'b> BigUint: SubAssign<&'b BigUint>,
			for<'b> BigUint: MulAssign<&'b BigUint>,
			for<'b> BigUint: DivAssign<&'b BigUint>,
			for<'b> BigUint: RemAssign<&'b BigUint>,
			for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output=BigUint>,
			for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output=BigUint>,
			for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output=BigUint>,
			for<'b> BigUint: BitAndAssign<&'b BigUint>,
			for<'b> BigUint: BitOrAssign<&'b BigUint>,
			for<'b> BigUint: BitXorAssign<&'b BigUint>,
			for<'a> &'a BigUint: Shr<usize, Output=BigUint>,
			for<'a> &'a BigUint: Shl<usize, Output=BigUint>,

			BigInt: BigIntApi<BigUint> + 'static,
			for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output=BigInt>,
			for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output=BigInt>,
			for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output=BigInt>,
			for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output=BigInt>,
			for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output=BigInt>,
			for<'b> BigInt: AddAssign<&'b BigInt>,
			for<'b> BigInt: SubAssign<&'b BigInt>,
			for<'b> BigInt: MulAssign<&'b BigInt>,
			for<'b> BigInt: DivAssign<&'b BigInt>,
			for<'b> BigInt: RemAssign<&'b BigInt>,
	}
}

pub fn api_where() -> proc_macro2::TokenStream {
	let bi_where = big_int_where();

	quote! {
	  #bi_where
		T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + Clone + 'static,
	}
}

pub fn contract_trait_api_impl(contract_struct: &syn::Path) -> proc_macro2::TokenStream {
	let api_where = api_where();
	quote! {
	  impl <T, BigInt, BigUint> ContractHookApi<BigInt, BigUint> for #contract_struct<T, BigInt, BigUint>
	  #api_where
	  {
		#[inline]
		fn get_sc_address(&self) -> Address {
		  self.api.get_sc_address()
		}

		#[inline]
		fn get_owner_address(&self) -> Address {
		  self.api.get_owner_address()
		}

		#[inline]
		fn get_caller(&self) -> Address {
		  self.api.get_caller()
		}

		#[inline]
		fn get_balance(&self, address: &Address) -> BigUint {
		  self.api.get_balance(address)
		}

		#[inline]
		fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]) {
		  self.api.storage_store_slice_u8(key, value);
		}

		#[inline]
		fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8> {
		  self.api.storage_load_vec_u8(key)
		}

		#[inline]
		fn storage_load_len(&self, key: &[u8]) -> usize {
		  self.api.storage_load_len(key)
		}

		#[inline]
		fn storage_store_bytes32(&self, key: &[u8], value: &[u8; 32]) {
		  self.api.storage_store_bytes32(key, value);
		}

		#[inline]
		fn storage_load_bytes32(&self, key: &[u8]) -> [u8; 32] {
		  self.api.storage_load_bytes32(key)
		}

		#[inline]
		fn storage_store_big_uint(&self, key: &[u8], value: &BigUint) {
		  self.api.storage_store_big_uint(key, value);
		}

		#[inline]
		fn storage_load_big_uint(&self, key: &[u8]) -> BigUint {
		  self.api.storage_load_big_uint(key)
		}

		#[inline]
		fn storage_store_big_uint_raw(&self, key: &[u8], handle: i32) {
		  self.api.storage_store_big_uint_raw(key, handle);
		}

		#[inline]
		fn storage_load_big_uint_raw(&self, key: &[u8]) -> i32 {
		  self.api.storage_load_big_uint_raw(key)
		}

		#[inline]
		fn storage_store_big_int(&self, key: &[u8], value: &BigInt) {
		  self.api.storage_store_big_int(key, value);
		}

		#[inline]
		fn storage_load_big_int(&self, key: &[u8]) -> BigInt {
		  self.api.storage_load_big_int(key)
		}

		#[inline]
		fn storage_store_i64(&self, key: &[u8], value: i64) {
		  self.api.storage_store_i64(key, value);
		}

		#[inline]
		fn storage_store_u64(&self, key: &[u8], value: u64) {
		  self.api.storage_store_u64(key, value);
		}

		#[inline]
		fn storage_load_i64(&self, key: &[u8]) -> i64 {
		  self.api.storage_load_i64(key)
		}

		#[inline]
		fn storage_load_u64(&self, key: &[u8]) -> u64 {
		  self.api.storage_load_u64(key)
		}

		#[inline]
		fn get_call_value_big_uint(&self) -> BigUint {
		  self.api.get_call_value_big_uint()
		}

		#[inline]
		fn get_esdt_value_big_uint(&self) -> BigUint {
			self.api.get_esdt_value_big_uint()
		}

		#[inline]
		fn get_esdt_token_name(&self) -> Vec<u8> {
			self.api.get_esdt_token_name()
		}

		#[inline]
		fn send_tx(&self, to: &Address, amount: &BigUint, data: &[u8]) {
		  self.api.send_tx(to, amount, data);
		}

		#[inline]
		fn async_call(&self, to: &Address, amount: &BigUint, data: &[u8]) {
		  self.api.async_call(to, amount, data);
		}

		#[inline]
		fn deploy_contract(&self, gas: u64, amount: &BigUint, code: &BoxedBytes, code_metadata: CodeMetadata, arg_buffer: &ArgBuffer) -> Address {
			self.api.deploy_contract(gas, amount, code, code_metadata, arg_buffer)
		}

		#[inline]
		fn get_tx_hash(&self) -> H256 {
		  self.api.get_tx_hash()
		}

		#[inline]
		fn get_gas_left(&self) -> u64 {
		  self.api.get_gas_left()
		}

		#[inline]
		fn get_block_timestamp(&self) -> u64 {
		  self.api.get_block_timestamp()
		}

		#[inline]
		fn get_block_nonce(&self) -> u64 {
		  self.api.get_block_nonce()
		}

		#[inline]
		fn get_block_round(&self) -> u64 {
		  self.api.get_block_round()
		}

		#[inline]
		fn get_block_epoch(&self) -> u64 {
		  self.api.get_block_epoch()
		}

		#[inline]
		fn get_block_random_seed(&self) -> Box<[u8; 48]> {
		  self.api.get_block_random_seed()
		}

		#[inline]
		fn get_prev_block_timestamp(&self) -> u64 {
		  self.api.get_prev_block_timestamp()
		}

		#[inline]
		fn get_prev_block_nonce(&self) -> u64 {
		  self.api.get_prev_block_nonce()
		}

		#[inline]
		fn get_prev_block_round(&self) -> u64 {
		  self.api.get_prev_block_round()
		}

		#[inline]
		fn get_prev_block_epoch(&self) -> u64 {
		  self.api.get_prev_block_epoch()
		}

		#[inline]
		fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
		  self.api.get_prev_block_random_seed()
		}

		#[inline]
		fn sha256(&self, data: &[u8]) -> H256 {
		  self.api.sha256(data)
		}

		#[inline]
		fn keccak256(&self, data: &[u8]) -> H256 {
		  self.api.keccak256(data)
		}

		#[inline]
		fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
			self.api.verify_bls(key, message, signature)
		}

		#[inline]
		fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
			self.api.verify_ed25519(key, message, signature)
		}

		#[inline]
		fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
			self.api.verify_secp256k1(key, message, signature)
		}
	  }
	}
}
