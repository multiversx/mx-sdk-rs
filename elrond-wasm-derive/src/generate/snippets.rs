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
		T: elrond_wasm::api::ContractSelfApi<BigInt, BigUint>
		 + elrond_wasm::api::ErrorApi
		 + elrond_wasm::api::BlockchainApi<BigUint>
		 + elrond_wasm::api::CallValueApi<BigUint>
		 + elrond_wasm::api::SendApi<BigUint>
		 + elrond_wasm::api::EndpointArgumentApi
		 + elrond_wasm::api::EndpointFinishApi
		 + elrond_wasm::api::StorageReadApi
		 + elrond_wasm::api::StorageWriteApi
		 + elrond_wasm::api::CryptoApi
		 + elrond_wasm::api::LogApi
		 + Clone
		 + 'static,
	}
}

pub fn contract_trait_api_impl(contract_struct: &syn::Path) -> proc_macro2::TokenStream {
	let api_where = api_where();
	quote! {
		impl <T, BigInt, BigUint> elrond_wasm::api::ContractSelfApi<BigInt, BigUint> for #contract_struct<T, BigInt, BigUint>
		#api_where
		{
			type Storage = T::Storage;
			type CallValue = T::CallValue;
			type SendApi = T::SendApi;
			type BlockchainApi = T::BlockchainApi;
			type CryptoApi = T::CryptoApi;

			#[inline]
			fn get_storage_raw(&self) -> Self::Storage {
				self.api.get_storage_raw()
			}

			#[inline]
			fn call_value(&self) -> Self::CallValue {
				self.api.call_value()
			}

			#[inline]
			fn send(&self) -> Self::SendApi {
				self.api.send()
			}

			#[inline]
			fn blockchain(&self) -> Self::BlockchainApi {
				self.api.blockchain()
			}

			#[inline]
			fn crypto(&self) -> Self::CryptoApi<BigInt, BigUint> {
				self.api.crypto()
			}
		}
	}
}
