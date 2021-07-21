true
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2018::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use elrond_wasm::api::{
    BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi, ProxyObjApi,
    SendApi,
};
use elrond_wasm::api::{ErrorApi, LogApi};
use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
use elrond_wasm::err_msg;
use elrond_wasm::esdt::*;
use elrond_wasm::io::*;
use elrond_wasm::non_zero_util::*;
use elrond_wasm::storage::mappers::*;
use elrond_wasm::types::*;
use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
use elrond_wasm::{Box, Vec};
/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
pub trait Adder: elrond_wasm::api::ContractBase + Sized
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
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
    Self::BigInt: elrond_wasm::api::BigIntApi,
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
    /// Add desired amount to the storage variable.
    fn add(&self, value: &Self::BigInt) -> SCResult<()> {
        let mut sum = self.get_sum();
        sum += value;
        self.set_sum(&sum);
        Ok(())
    }
    fn get_sum(&self) -> Self::BigInt;
    fn set_sum(&self, sum: &Self::BigInt);
}
pub trait AutoImpl: elrond_wasm::api::ContractBase {}
impl<C> Adder for C
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
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
    Self::BigInt: elrond_wasm::api::BigIntApi,
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
    C: AutoImpl,
{
    fn get_sum(&self) -> Self::BigInt {
        let key: &'static [u8] = b"sum";
        elrond_wasm::storage_get(self.get_storage_raw(), &key[..])
    }
    fn set_sum(&self, sum: &Self::BigInt) {
        let key: &'static [u8] = b"sum";
        elrond_wasm::storage_set(self.get_storage_raw(), &key[..], &sum);
    }
}
pub trait EndpointWrappers: elrond_wasm::api::ContractPrivateApi + Adder
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
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
    Self::BigInt: elrond_wasm::api::BigIntApi,
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
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
        let result = self.get_sum();
        elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
    }
    #[inline]
    fn call_init(&self) {
        self.call_value().check_not_payable();
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
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
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
        let value = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigInt>(
            self.argument_api(),
            0i32,
            ArgId::from(&b"value"[..]),
        );
        let result = self.add(&value);
        elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
    }
    fn call(&self, fn_name: &[u8]) -> bool {
        if match fn_name {
            b"callBack" => {
                self::EndpointWrappers::callback(self);
                return true;
            }
            [103u8, 101u8, 116u8, 83u8, 117u8, 109u8] => {
                self.call_get_sum();
                true
            }
            [105u8, 110u8, 105u8, 116u8] => {
                self.call_init();
                true
            }
            [97u8, 100u8, 100u8] => {
                self.call_add();
                true
            }
            other => false,
        } {
            return true;
        }
        false
    }
    fn callback_selector<'a>(
        &self,
        mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
    ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
        elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
    }
    fn callback(&self) {}
}
pub struct AbiProvider {}
impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
    type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
    type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
    type Storage = elrond_wasm::api::uncallable::UncallableApi;
    type SendApi = elrond_wasm::api::uncallable::UncallableApi;
    fn abi() -> elrond_wasm::abi::ContractAbi {
        let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & ["One of the simplest smart contracts possible," , "it holds a single variable in storage, which anyone can increment."] , name : "Adder" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "getSum",
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_output::<Self::BigInt>(&[]);
        contract_abi.add_type_descriptions::<Self::BigInt>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "init",
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<&Self::BigInt>("initial_value");
        contract_abi.add_type_descriptions::<&Self::BigInt>();
        contract_abi.constructor = Some(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &["Add desired amount to the storage variable."],
            name: "add",
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<&Self::BigInt>("value");
        contract_abi.add_type_descriptions::<&Self::BigInt>();
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        contract_abi
    }
}
pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
    api: A,
}
impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
where
    A: elrond_wasm::api::ContractBase
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
    type LogApi = A::LogApi;
    type ErrorApi = A::ErrorApi;
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
    #[inline]
    fn log_api_raw(&self) -> Self::LogApi {
        self.api.log_api_raw()
    }
    #[inline]
    fn error_api(&self) -> Self::ErrorApi {
        self.api.error_api()
    }
}
impl<A> AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
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
impl<A> EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
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
    Self::BigInt: elrond_wasm::api::BigIntApi,
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
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
where
    A::BigUint: elrond_wasm::api::BigUintApi,
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
    A::BigInt: elrond_wasm::api::BigIntApi,
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
pub fn contract_obj<A>(api: A) -> ContractObj<A>
where
    A::BigUint: elrond_wasm::api::BigUintApi,
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
    A::BigInt: elrond_wasm::api::BigIntApi,
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
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
    ContractObj { api }
}
pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
    #[allow(clippy::too_many_arguments)]
    fn get_sum(
        self,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <Self::BigInt as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            ___token___,
            ___payment___,
            ___nonce___,
            elrond_wasm::types::BoxedBytes::from(&b"getSum"[..]),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    fn init(
        self,
        initial_value: &Self::BigInt,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <() as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            ___token___,
            ___payment___,
            ___nonce___,
            elrond_wasm::types::BoxedBytes::from(&b"init"[..]),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            initial_value,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    fn add(
        self,
        value: &Self::BigInt,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            ___token___,
            ___payment___,
            ___nonce___,
            elrond_wasm::types::BoxedBytes::from(&b"add"[..]),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            value,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        ___contract_call___
    }
}
pub struct Proxy<SA>
where
    SA: elrond_wasm::api::SendApi + 'static,
{
    pub api: SA,
    pub address: Address,
    pub payment_token: elrond_wasm::types::TokenIdentifier,
    pub payment_amount: SA::AmountType,
    pub payment_nonce: u64,
}
impl<SA> elrond_wasm::api::ProxyObjApi for Proxy<SA>
where
    SA: elrond_wasm::api::SendApi + 'static,
{
    type BigUint = SA::AmountType;
    type BigInt = SA::ProxyBigInt;
    type Storage = SA::ProxyStorage;
    type SendApi = SA;
    fn new_proxy_obj(api: SA, address: Address) -> Self {
        Proxy {
            api,
            address,
            payment_token: elrond_wasm::types::TokenIdentifier::egld(),
            payment_amount: Self::BigUint::zero(),
            payment_nonce: 0,
        }
    }
    fn with_token_transfer(mut self, token: TokenIdentifier, payment: Self::BigUint) -> Self {
        self.payment_token = token;
        self.payment_amount = payment;
        self
    }
    #[inline]
    fn with_nft_nonce(mut self, nonce: u64) -> Self {
        self.payment_nonce = nonce;
        self
    }
    #[inline]
    fn into_fields(self) -> (Self::SendApi, Address, TokenIdentifier, Self::BigUint, u64) {
        (
            self.api,
            self.address,
            self.payment_token,
            self.payment_amount,
            self.payment_nonce,
        )
    }
}
impl<SA> ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
