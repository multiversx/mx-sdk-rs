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
		T: elrond_wasm::api::ContractHookApi<BigInt, BigUint>
		 + elrond_wasm::api::ErrorApi
		 + elrond_wasm::api::CallValueApi<BigUint>
		 + elrond_wasm::api::EndpointArgumentApi
		 + elrond_wasm::api::EndpointFinishApi
		 + elrond_wasm::api::StorageReadApi
		 + elrond_wasm::api::StorageWriteApi
		 + elrond_wasm::api::LogApi
		 + Clone
		 + 'static,
	}
}

pub fn contract_trait_api_impl(contract_struct: &syn::Path) -> proc_macro2::TokenStream {
	let api_where = api_where();
	quote! {
		impl <T, BigInt, BigUint> elrond_wasm::api::ContractHookApi<BigInt, BigUint> for #contract_struct<T, BigInt, BigUint>
		#api_where
		{
			type Storage = T::Storage;
			type CallValue = T::CallValue;

			#[inline]
			fn get_storage_raw(&self) -> Self::Storage {
				self.api.get_storage_raw()
			}

			#[inline]
			fn call_value(&self) -> Self::CallValue {
				self.api.call_value()
			}

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
			fn execute_on_dest_context(&self, gas: u64, address: &Address, value: &BigUint, function: &[u8], arg_buffer: &ArgBuffer) {
				self.api.execute_on_dest_context(gas, address, value, function, arg_buffer);
			}

			#[inline]
			fn execute_on_dest_context_by_caller(&self, gas: u64, address: &Address, value: &BigUint, function: &[u8], arg_buffer: &ArgBuffer) {
				self.api.execute_on_dest_context_by_caller(gas, address, value, function, arg_buffer);
			}

			#[inline]
			fn execute_on_same_context(&self, gas: u64, address: &Address, value: &BigUint, function: &[u8], arg_buffer: &ArgBuffer) {
				self.api.execute_on_same_context(gas, address, value, function, arg_buffer);
			}
		}

		impl <T, BigInt, BigUint> elrond_wasm::api::CryptoApi for #contract_struct<T, BigInt, BigUint>
		#api_where
		{
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
