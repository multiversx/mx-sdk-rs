multiversx_sc::imports!();
multiversx_sc::derive_imports!();
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::types::native::{NativeConvertible, NativeValue, NativeValueManagedVecItem};

#[derive(TopEncode, NestedEncode, TopDecode, NestedDecode, Clone, PartialEq, Debug)]
struct SampleStruct {
    pub buffer: ManagedBuffer<DebugApi>,
    pub biguint: BigUint<DebugApi>
}

#[derive(TopEncode, NestedEncode, TopDecode, NestedDecode, ManagedVecItem, Clone, PartialEq, Debug)]
struct SampleStructManagedVecItem {
    pub buffer: ManagedBuffer<DebugApi>,
    pub biguint: BigUint<DebugApi>
}

#[test]
fn test_user_struct_to_native() {
    let _ = DebugApi::dummy();
    let buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from("buffer");
    let biguint: BigUint<DebugApi> = BigUint::from(1000u64);

    let sample_struct = SampleStruct {
        buffer,
        biguint
    };
    let native = NativeValue::new(sample_struct.clone()).to_native();

    let expected_result = sample_struct;

    assert_eq!(
        native,
        expected_result
    );
}

#[test]
fn test_user_struct_managed_vec_item_to_native() {
    let _ = DebugApi::dummy();
    let buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from("buffer");
    let biguint: BigUint<DebugApi> = BigUint::from(1000u64);

    let sample_struct = SampleStructManagedVecItem {
        buffer,
        biguint
    };
    let native = NativeValueManagedVecItem::new(sample_struct.clone()).to_native();

    let expected_result = sample_struct;

    assert_eq!(
        native,
        expected_result
    );
}

#[test]
fn test_user_struct_in_managed_vec_to_native() {
    let _ = DebugApi::dummy();
    let buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from("buffer");
    let biguint: BigUint<DebugApi> = BigUint::from(1000u64);

    let first_sample_struct = SampleStructManagedVecItem {
        buffer,
        biguint
    };
    let second_sample_struct = first_sample_struct.clone();
    let mut managed_vec: ManagedVec<DebugApi, NativeValueManagedVecItem<SampleStructManagedVecItem>> = ManagedVec::new();
    managed_vec.push(NativeValueManagedVecItem::new(first_sample_struct.clone()));
    managed_vec.push(NativeValueManagedVecItem::new(second_sample_struct.clone()));

    let native = managed_vec.to_native();

    let expected_result = vec![first_sample_struct, second_sample_struct];

    assert_eq!(
        native,
        expected_result
    );
}