// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

use elrond_wasm::types::Address;

use crate::module_1::VersionModule;

mod module_1 {
	use elrond_wasm::api::{
		ContractPrivateApi, EndpointArgumentApi, EndpointFinishApi, ProxyObjApi,
	};

	elrond_wasm::imports!();

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait VersionModule: ContractSelfApi + Sized
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
	{
		fn version(&self) -> Self::BigInt;
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait AutoImpl: ContractSelfApi {}

	impl<C> VersionModule for C
	where
		C: AutoImpl,
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
	{
		fn version(&self) -> Self::BigInt {
			Self::BigInt::from(100)
		}
	}

	pub trait CallMethods: VersionModule + ContractPrivateApi
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
	{
		#[inline]
		fn call_version(&self) {
			self.call_value().check_not_payable();
			let result = self.version();
			EndpointResult::<Self::FinishApi>::finish(&result, self.finish_api());
		}

		fn call(&self, fn_name: &[u8]) -> bool {
			if match fn_name {
				// b"callBack" => {
				//     self.callback();
				//     return true;
				// }
				b"version" => {
					self.call_version();
					true
				},
				_other => false,
			} {
				return true;
			}
			false
		}
	}

	pub trait CallProxy: ProxyObjApi + Sized {
		fn version(self) -> ContractCall<Self::PaymentType, Self::BigInt> {
			let (___api___, ___address___, ___token___, ___payment___) = self.into_fields();
			let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
				___address___,
				___token___,
				___payment___,
				elrond_wasm::types::BoxedBytes::from(&b"version"[..]),
			);
			___contract_call___
		}
	}
}

mod sample_adder {
	use elrond_wasm::api::{
		ContractPrivateApi, EndpointArgumentApi, EndpointFinishApi, ProxyObjApi,
	};

	elrond_wasm::imports!();

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait Adder: super::module_1::VersionModule + ContractSelfApi + Sized
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
	{
		fn init(&self, initial_value: &Self::BigInt) {
			self.set_sum(initial_value);
		}
		fn add(&self, value: &Self::BigInt) -> SCResult<()> {
			let mut sum = self.get_sum();
			sum += value;
			self.set_sum(&sum);
			Ok(())
		}
		fn get_sum(&self) -> Self::BigInt;
		fn set_sum(&self, sum: &Self::BigInt);
		fn add_version(&self) -> SCResult<()> {
			self.add(&self.version())
		}
		fn callback(&self);
		// fn callbacks(&self) -> callback_proxy::CallbackProxies<T, BigInt, BigUint>;
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub trait AutoImpl: ContractSelfApi {}

	impl<C> Adder for C
	where
		C: AutoImpl + super::module_1::VersionModule,
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
	{
		fn get_sum(&self) -> Self::BigInt {
			let key: &'static [u8] = b"sum";
			elrond_wasm::storage_get(self.get_storage_raw(), &key[..])
		}
		fn set_sum(&self, sum: &Self::BigInt) {
			let key: &'static [u8] = b"sum";
			elrond_wasm::storage_set(self.get_storage_raw(), &key[..], &sum);
		}
		fn callback(&self) {}
	}

	pub trait CallMethods: Adder + ContractPrivateApi + super::module_1::CallMethods
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
	{
		#[inline]
		fn call_get_sum(&self) {
			self.call_value().check_not_payable();
			self.argument_api().check_num_arguments(0i32);
			let result = self.get_sum();
			EndpointResult::<Self::FinishApi>::finish(&result, self.finish_api());
		}
		#[inline]
		fn call_init(&self) {
			self.call_value().check_not_payable();
			self.argument_api().check_num_arguments(1i32);
			let initial_value = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigInt>(
				self.argument_api(),
				0i32,
				ArgId::from(&b"initial_value"[..]),
			);
			self.init(&initial_value);
		}
		#[inline]
		fn call_add(&self) {
			self.call_value().check_not_payable();
			self.argument_api().check_num_arguments(1i32);
			let value = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigInt>(
				self.argument_api(),
				0i32,
				ArgId::from(&b"value"[..]),
			);
			let result = self.add(&value);
			EndpointResult::<Self::FinishApi>::finish(&result, self.finish_api());
		}

		fn call(&self, fn_name: &[u8]) -> bool {
			if match fn_name {
				// b"callBack" => {
				//     self.callback();
				//     return true;
				// }
				[103u8, 101u8, 116u8, 83u8, 117u8, 109u8] => {
					self.call_get_sum();
					true
				},
				[105u8, 110u8, 105u8, 116u8] => {
					self.call_init();
					true
				},
				[97u8, 100u8, 100u8] => {
					self.call_add();
					true
				},
				_other => false,
			} {
				return true;
			}
			if super::module_1::CallMethods::call(self, fn_name) {
				return true;
			}
			false
		}
	}

	pub trait CallProxy: ProxyObjApi + super::module_1::CallProxy {
		fn get_sum(self) -> ContractCall<Self::PaymentType, Self::BigInt> {
			let (___api___, ___address___, ___token___, ___payment___) = self.into_fields();
			let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
				___address___,
				___token___,
				___payment___,
				elrond_wasm::types::BoxedBytes::from(&b"get_sum"[..]),
			);
			___contract_call___
		}
		fn add(self, amount: &Self::BigInt) -> ContractCall<Self::PaymentType, SCResult<()>> {
			let (___api___, ___address___, ___token___, ___payment___) = self.into_fields();
			let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
				___address___,
				___token___,
				___payment___,
				elrond_wasm::types::BoxedBytes::from(&b"get_sum"[..]),
			);
			elrond_wasm::io::serialize_contract_call_arg(
				amount,
				___contract_call___.get_mut_arg_buffer(),
				___api___.clone(),
			);
			___contract_call___
		}
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	pub struct ContractObj<A: ContractSelfApi> {
		api: A,
	}

	impl<A> ContractObj<A>
	where
		A: ContractSelfApi,
	{
		pub fn new_contract_obj(api: A) -> Self {
			ContractObj { api }
		}
	}

	/////////////////////////////////////////////////////////////////////////////////////////////////
	//////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////////////////////////////////////////
	impl<A> ContractSelfApi for ContractObj<A>
	where
		A: ContractSelfApi
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

	impl<A> super::module_1::AutoImpl for ContractObj<A> where
		A: ContractSelfApi
			+ elrond_wasm::api::ErrorApi
			+ elrond_wasm::api::EndpointArgumentApi
			+ elrond_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static
	{
	}

	impl<A> AutoImpl for ContractObj<A> where
		A: ContractSelfApi
			+ elrond_wasm::api::ErrorApi
			+ elrond_wasm::api::EndpointArgumentApi
			+ elrond_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static
	{
	}

	impl<A> ContractPrivateApi for ContractObj<A>
	where
		A: ContractSelfApi
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

	impl<A> super::module_1::CallMethods for ContractObj<A>
	where
		A: ContractSelfApi
			+ elrond_wasm::api::ErrorApi
			+ elrond_wasm::api::EndpointArgumentApi
			+ elrond_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
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
	{
	}

	impl<A> CallMethods for ContractObj<A>
	where
		A: ContractSelfApi
			+ elrond_wasm::api::ErrorApi
			+ elrond_wasm::api::EndpointArgumentApi
			+ elrond_wasm::api::EndpointFinishApi
			+ Clone
			+ 'static,
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
	{
	}

	pub struct ProxyObj<SA>
	where
		SA: SendApi + 'static,
	{
		pub api: SA,
		pub address: Address,
		pub token: elrond_wasm::types::TokenIdentifier,
		pub payment: SA::AmountType,
	}

	impl<SA> ProxyObj<SA>
	where
		SA: SendApi + 'static,
	{
		pub fn new_proxy_obj(api: SA, address: Address) -> Self {
			ProxyObj {
				api,
				address,
				token: elrond_wasm::types::TokenIdentifier::egld(),
				payment: SA::AmountType::zero(),
			}
		}
	}

	impl<SA> ProxyObjApi for ProxyObj<SA>
	where
		SA: SendApi + 'static,
	{
		type BigUint = SA::ProxyBigUint;

		type BigInt = SA::ProxyBigInt;

		type PaymentType = SA::AmountType;

		type ProxySendApi = SA;

		fn with_token_transfer(
			mut self,
			token: TokenIdentifier,
			payment: Self::PaymentType,
		) -> Self {
			self.token = token;
			self.payment = payment;
			self
		}

		fn into_fields(
			self,
		) -> (
			Self::ProxySendApi,
			Address,
			TokenIdentifier,
			Self::PaymentType,
		) {
			(self.api, self.address, self.token, self.payment)
		}
	}

	impl<SA> super::module_1::CallProxy for ProxyObj<SA> where SA: SendApi {}

	impl<SA> CallProxy for ProxyObj<SA> where SA: SendApi {}
}

#[test]
fn test_add() {
	use elrond_wasm::api::ContractSelfApi;
	use elrond_wasm_debug::api::RustBigInt;
	use elrond_wasm_debug::TxContext;
	use sample_adder::{Adder, CallMethods, CallProxy};
	// use module_1::{VersionModule, CallMethods};

	let tx_context = TxContext::dummy();

	let adder = sample_adder::ContractObj::new_contract_obj(tx_context.clone());

	adder.init(&RustBigInt::from(5));
	assert_eq!(RustBigInt::from(5), adder.get_sum());

	let _ = adder.add(&RustBigInt::from(7));
	assert_eq!(RustBigInt::from(12), adder.get_sum());

	let _ = adder.add(&RustBigInt::from(-1));
	assert_eq!(RustBigInt::from(11), adder.get_sum());

	assert_eq!(RustBigInt::from(100), adder.version());

	let _ = adder.add_version();
	assert_eq!(RustBigInt::from(111), adder.get_sum());

	assert!(!adder.call(b"invalid_endpoint"));

	assert!(adder.call(b"version"));

	let own_proxy = sample_adder::ProxyObj::new_proxy_obj(adder.send().clone(), Address::zero());
	let _ = own_proxy.get_sum();
}
