pub fn where_self_big_int() -> proc_macro2::TokenStream {
	quote! {
		where
			for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
			for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
			for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
			for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
			for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
			for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	}
}

pub fn where_api_big_int() -> proc_macro2::TokenStream {
	quote! {
		where
			for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
			for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
			for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
			for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
			for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
			for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
	}
}

pub fn api_where() -> proc_macro2::TokenStream {
	let where_self_big_int = where_self_big_int();

	quote! {
	  #where_self_big_int
		T: elrond_wasm::api::ContractBase
		 + elrond_wasm::api::ErrorApi
		 + elrond_wasm::api::BlockchainApi
		 + elrond_wasm::api::CallValueApi
		 + elrond_wasm::api::SendApi
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

pub fn contract_object_def() -> proc_macro2::TokenStream {
	quote! {
		pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
			api: A,
		}
	}
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
	quote! {
		impl<A>elrond_wasm::api::ContractBase for ContractObj<A>
		where
			A:elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
			type BigUint = A::BigUint;
			type BigInt = A::BigInt;
			type Storage = A::Storage;
			type CallValue = A::CallValue;
			type SendApi = A::SendApi;
			type BlockchainApi = A::BlockchainApi;
			type CryptoApi = A::CryptoApi;

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
			fn crypto(&self) -> Self::CryptoApi {
				self.api.crypto()
			}
		}
	}
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
	let where_api_big_int = where_api_big_int();
	quote! {
		pub fn contract_obj<A>(api: A) -> ContractObj<A>
		#where_api_big_int
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
			ContractObj { api }
		}
	}
}

pub fn impl_auto_impl() -> proc_macro2::TokenStream {
	quote! {
		impl<A> AutoImpl for ContractObj<A> where
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static
		{
		}
	}
}
pub fn impl_private_api() -> proc_macro2::TokenStream {
	quote! {
	impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
	where
		A: elrond_wasm::api::ContractBase
			+ elrond_wasm::api::ErrorApi
			+ elrond_wasm::api::EndpointArgumentApi
			+ elrond_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
	{
		type ArgumentApi = A;

		type FinishApi = A;

		#[inline]
		fn argument_api(&self) -> Self::ArgumentApi {
			self.api.clone()
		}

		#[inline]
		fn finish_api(&self) -> Self::FinishApi {
			self.api.clone()
		}
	}
	}
}

pub fn impl_endpoint_wrappers() -> proc_macro2::TokenStream {
	let where_self_big_int = where_self_big_int();
	quote! {
		impl<A> EndpointWrappers for ContractObj<A>
		#where_self_big_int
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
		}
	}
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
	let where_api_big_int = where_api_big_int();
	quote! {
		impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
		#where_api_big_int
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
			fn call(&self, fn_name: &[u8]) -> bool {
				EndpointWrappers::call(self, fn_name)
			}
			fn into_api(self: Box<Self>) -> A {
				self.api
			}
		}
	}
}
