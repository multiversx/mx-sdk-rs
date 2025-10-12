#![no_std]

use multiversx_sc::imports::*;

mod abi_enum;
pub mod abi_proxy;
mod abi_test_type;
mod only_nested;

use abi_enum::*;
use abi_test_type::*;
use only_nested::*;

/// Contract whose sole purpose is to verify that
/// the ABI generation framework works as expected.
///
/// Note: any change in this contract must also be reflected in `abi_test_expected.abi.json`,
/// including Rust docs.
#[multiversx_sc::contract]
#[esdt_attribute("TICKER1", BigUint)]
#[esdt_attribute("TICKER2", ManagedBuffer)]
#[esdt_attribute("TICKER3", u32)]
#[esdt_attribute("STRUCT1", AbiEnum)]
#[esdt_attribute("STRUCT2", AbiManagedType<Self::Api>)]
#[esdt_attribute("OnlyInEsdt", OnlyShowsUpInEsdtAttr)]
#[esdt_attribute("ExplicitDiscriminant", ExplicitDiscriminant)]
#[esdt_attribute("ExplicitDiscriminantMixed", ExplicitDiscriminantMixed)]
#[esdt_attribute("ManagedDecimalVar", ManagedDecimal<Self::Api, NumDecimals>)]
#[esdt_attribute("ManagedDecimalConst", ManagedDecimalWrapper<Self::Api>)]
pub trait AbiTester {
    /// Contract constructor.
    #[init]
    #[payable("EGLD")]
    fn init(&self, _constructor_arg_1: i32, _constructor_arg_2: OnlyShowsUpInConstructor) {}

    /// Upgrade constructor.
    #[upgrade]
    fn upgrade(&self, _constructor_arg_1: i32, _constructor_arg_2: OnlyShowsUpInConstructor) {
        self.init(_constructor_arg_1, _constructor_arg_2)
    }

    /// Example endpoint docs.
    #[endpoint]
    #[output_name("single output")]
    #[output_name("this one doesn't show up")]
    fn echo_abi_test_type(&self, att: AbiTestType) -> AbiTestType {
        att
    }

    #[endpoint]
    #[only_owner]
    fn echo_enum(&self, e: AbiEnum) -> AbiEnum {
        e
    }

    #[endpoint]
    #[only_owner]
    fn take_managed_type(&self, _arg: AbiManagedType<Self::Api>) {}

    #[endpoint]
    #[title("result-3")]
    #[output_name("multi-result-1")]
    #[output_name("multi-result-2")]
    #[output_name("multi-result-3")]
    #[output_name("multi-result-in-excess")]
    fn multi_result_3(&self) -> MultiValue3<i32, [u8; 3], BoxedBytes> {
        (1, [2; 3], BoxedBytes::empty()).into()
    }

    #[endpoint]
    #[output_name("multi-too-few-1")]
    #[output_name("multi-too-few-2")]
    fn multi_result_4(&self) -> MultiValue4<i32, [u8; 3], BoxedBytes, OnlyShowsUpAsNested03> {
        (1, [2; 3], BoxedBytes::empty(), OnlyShowsUpAsNested03()).into()
    }

    #[endpoint]
    fn var_args(
        &self,
        _simple_arg: u32,
        _var_args: MultiValueVec<MultiValue2<OnlyShowsUpAsNested04, i32>>,
    ) {
    }

    #[endpoint]
    fn multi_result_vec(&self) -> MultiValueVec<MultiValue3<OnlyShowsUpAsNested05, bool, ()>> {
        MultiValueVec::new()
    }

    #[endpoint]
    fn optional_arg(&self, _simple_arg: u32, _opt_args: OptionalValue<OnlyShowsUpAsNested06>) {}

    #[endpoint]
    fn optional_result(&self) -> OptionalValue<OnlyShowsUpAsNested07> {
        OptionalValue::None
    }

    #[endpoint]
    fn address_vs_h256(&self, address: Address, h256: H256) -> MultiValue2<Address, H256> {
        self.address_h256_event(&address, &h256);
        (address, h256).into()
    }

    #[endpoint]
    fn managed_address_vs_byte_array(
        &self,
        address: ManagedAddress,
        byte_array: ManagedByteArray<Self::Api, 32>,
    ) -> MultiValue2<ManagedAddress, ManagedByteArray<Self::Api, 32>> {
        (address, byte_array).into()
    }

    #[endpoint]
    fn process_managed_decimal(
        &self,
        input: ManagedDecimal<Self::Api, ConstDecimals<U10>>,
    ) -> ManagedDecimal<Self::Api, usize> {
        input.into()
    }

    #[endpoint]
    fn esdt_local_role(&self) -> EsdtLocalRole {
        EsdtLocalRole::None
    }

    #[endpoint]
    fn esdt_token_payment(&self) -> EsdtTokenPayment<Self::Api> {
        unreachable!()
    }

    #[endpoint]
    fn esdt_token_data(&self) -> EsdtTokenData<Self::Api> {
        unreachable!()
    }

    #[endpoint]
    fn time_types(
        &self,
    ) -> MultiValue4<TimestampMillis, TimestampSeconds, DurationMillis, DurationSeconds> {
        unreachable!()
    }

    #[view]
    #[storage_mapper("sample_storage_mapper")]
    fn sample_storage_mapper(&self) -> SingleValueMapper<OnlyShowsUpAsNestedInSingleValueMapper>;

    #[view]
    fn item_for_vec(&self) -> Vec<OnlyShowsUpAsNestedInVec> {
        Vec::new()
    }

    #[view]
    fn item_for_array_vec(&self) -> ArrayVec<OnlyShowsUpAsNestedInArrayVec, 3> {
        ArrayVec::new()
    }

    #[view]
    fn item_for_managed_vec(&self) -> ManagedVec<AbiManagedVecItem> {
        ManagedVec::new()
    }

    #[view]
    fn echo_permission(&self, p: Permission) -> Permission {
        p
    }

    #[view]
    fn item_for_array(&self, _array: &[OnlyShowsUpAsNestedInArray; 5]) {}

    #[view]
    fn item_for_box(&self) -> Box<OnlyShowsUpAsNestedInBox> {
        Box::new(OnlyShowsUpAsNestedInBox)
    }

    #[view]
    fn item_for_boxed_slice(&self) -> Box<[OnlyShowsUpAsNestedInBoxedSlice]> {
        Vec::new().into_boxed_slice()
    }

    #[view]
    fn item_for_ref(&self, _ref: &OnlyShowsUpAsNestedInRef) {}

    #[view]
    fn item_for_slice(&self, _ref: &[OnlyShowsUpAsNestedInSlice]) {}

    #[view]
    fn item_for_option(&self) -> Option<OnlyShowsUpAsNestedInOption> {
        None
    }

    #[view]
    fn operation_completion_status(&self) -> OperationCompletionStatus {
        OperationCompletionStatus::Completed
    }

    #[view]
    fn takes_object_with_managed_buffer_read_to_end(
        &self,
        arg: AbiWithManagedBufferReadToEnd<Self::Api>,
    ) -> ManagedBuffer {
        arg.flush.into_managed_buffer()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payable_egld(&self) {}

    #[endpoint]
    #[payable("TOKEN-FOR-ABI")]
    fn payable_some_token(&self) {
        let (token, payment) = self.call_value().single_fungible_esdt();
        self.payable_event(&token, &payment);
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_token(&self) {}

    #[endpoint]
    #[label("test-external-view")]
    fn external_view(&self) {}

    #[event("payable-event")]
    fn payable_event(&self, #[indexed] token: &EsdtTokenIdentifier, amount: &BigUint);

    #[event("address-h256-event")]
    fn address_h256_event(&self, #[indexed] address: &Address, #[indexed] h256: &H256);

    #[event]
    fn empty_identifier_event(&self);

    #[endpoint]
    #[label("label1")]
    fn label_a(&self) {}

    #[endpoint]
    #[label("label1")]
    #[label("label2")]
    fn label_b(&self) {}
}
