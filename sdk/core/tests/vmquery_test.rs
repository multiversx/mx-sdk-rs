multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc::types::{BigUint, ManagedAddress, ManagedBuffer};
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::address::Address;
use multiversx_sdk::data::types::native::{NativeValue, NativeValueManagedVecItem};
use multiversx_sdk::data::types::payment::Payment;
use multiversx_sdk::data::vm::{VMOutputApi, VmValueRequest};

const CONTRACT_ADDRESS: &str = "erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh";
const CALLER_ADDRESS: &str = "erd1devnet6uy8xjusvusfy3q83qadfhwrtty5fwa8ceh9cl60q2p6ysra7aaa";
const FUNC_NAME: &str = "";
const VALUE: &str = "0";

const SAMPLE_RETURN_DATA: [&'static str; 8] = [
    "VGhpcyBpcyBhIGJ1ZmZlcg==", // "This is a buffer",
    "DeC2s6dkAAA=", // 1e18
    "AAAAAAAAAAAFAPEt0QxNK+gmT+M52hS5+te982SufOs=", // erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh,
    "AAAAFFRoaXMgaXMgYSBzYW1wbGUgc3RyAAAAAgPoAAAAAAAAAAAFAPEt0QxNK+gmT+M52hS5+te982SufOs=", // SampleCodable : "This is a sample str", 1000, erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh
    "V0VHTEQtYWJjZGVm", // "WEGLD-abcdef"
    "Zmlyc3Q=", // "first"
    "c2Vjb25k", // "second"
    "dGhpcmQ=" // "third"
];

const SAMPLE_ESDT_TOKEN_PAYMENT_DATA: [&'static str; 1] = [
    "AAAADFdFR0xELWFiY2RlZgAAAAAAAAAFAAAAAgPo"
];

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
struct SampleCodable<M: ManagedTypeApi> {
    pub managed_buffer: ManagedBuffer<M>,
    pub biguint: BigUint<M>,
    pub address: ManagedAddress<M>
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Debug)]
struct SampleCodableManagedVecItem<M: ManagedTypeApi> {
    pub managed_buffer: ManagedBuffer<M>,
    pub biguint: BigUint<M>,
    pub address: ManagedAddress<M>
}

fn get_dummy_output_result_from_data(data: &[&str]) -> VMOutputApi {
    VMOutputApi {
        return_data: data.into_iter().map(|e| e.to_string()).collect(),
        return_code: "".to_string(),
        return_message: "".to_string(),
        gas_remaining: 0,
        gas_refund: 0,
        output_accounts: Default::default(),
        deleted_accounts: None,
        touched_accounts: None,
        logs: None,
    }
}

fn assert_common_vm_value_request_infos(
    vm_value_request: &VmValueRequest
) {
    assert_eq!(
        vm_value_request.sc_address.to_string(),
        CONTRACT_ADDRESS.to_string()
    );

    assert_eq!(
        vm_value_request.func_name.to_string(),
        FUNC_NAME.to_string()
    );

    assert_eq!(
        vm_value_request.caller.to_string(),
        CALLER_ADDRESS.to_string()
    );

    assert_eq!(
        vm_value_request.value.to_string(),
        VALUE.to_string()
    );
}

#[test]
fn test_create_vm_value_request_from_managed_buffer() {
    let _ = DebugApi::dummy();
    let input_str = "This is a sample str";
    let input_buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from(input_str);

    let mut result = VmValueRequest {
        sc_address: Address::from_bech32_string(CONTRACT_ADDRESS).unwrap(),
        func_name: FUNC_NAME.to_string(),
        caller: Address::from_bech32_string(CALLER_ADDRESS).unwrap(),
        value: VALUE.to_string(),
        args: vec![],
    };

    assert_common_vm_value_request_infos(&result);

    result.push_managed_arg(input_buffer);

    assert_eq!(
        result.args.len(),
        1
    );

    assert_eq!(
        result.args[0],
        "5468697320697320612073616d706c6520737472"
    );
}

#[test]
fn test_create_vm_value_request_from_biguint() {
    let _ = DebugApi::dummy();
    let input_biguint: BigUint<DebugApi> = BigUint::from(1000u64);

    let mut result = VmValueRequest {
        sc_address: Address::from_bech32_string(CONTRACT_ADDRESS).unwrap(),
        func_name: FUNC_NAME.to_string(),
        caller: Address::from_bech32_string(CALLER_ADDRESS).unwrap(),
        value: VALUE.to_string(),
        args: vec![],
    };

    assert_common_vm_value_request_infos(&result);

    result.push_managed_arg(input_biguint);

    assert_eq!(
        result.args.len(),
        1
    );

    assert_eq!(
        result.args[0],
        "03e8"
    );
}

#[test]
fn test_create_vm_value_request_from_address() {
    let _ = DebugApi::dummy();
    let address_bytes = Address::from_bech32_string(CONTRACT_ADDRESS).unwrap().to_bytes();
    let input_address: ManagedAddress<DebugApi> = ManagedAddress::from(address_bytes);

    let mut result = VmValueRequest {
        sc_address: Address::from_bech32_string(CONTRACT_ADDRESS).unwrap(),
        func_name: FUNC_NAME.to_string(),
        caller: Address::from_bech32_string(CALLER_ADDRESS).unwrap(),
        value: VALUE.to_string(),
        args: vec![],
    };

    assert_common_vm_value_request_infos(&result);

    result.push_managed_arg(input_address);

    assert_eq!(
        result.args.len(),
        1
    );

    assert_eq!(
        result.args[0],
        "00000000000000000500f12dd10c4d2be8264fe339da14b9fad7bdf364ae7ceb"
    );
}

#[test]
fn test_create_vm_value_request_from_struct() {
    let _ = DebugApi::dummy();

    let input_str = "This is a sample str";
    let input_buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from(input_str);

    let input_biguint: BigUint<DebugApi> = BigUint::from(1000u64);

    let address_bytes = Address::from_bech32_string(CONTRACT_ADDRESS).unwrap().to_bytes();
    let input_address: ManagedAddress<DebugApi> = ManagedAddress::from(address_bytes);

    let input_struct = SampleCodable {
        managed_buffer: input_buffer,
        biguint: input_biguint,
        address: input_address
    };
    let mut result = VmValueRequest {
        sc_address: Address::from_bech32_string(CONTRACT_ADDRESS).unwrap(),
        func_name: FUNC_NAME.to_string(),
        caller: Address::from_bech32_string(CALLER_ADDRESS).unwrap(),
        value: VALUE.to_string(),
        args: vec![],
    };

    assert_common_vm_value_request_infos(&result);

    result.push_managed_arg(input_struct);

    assert_eq!(
        result.args.len(),
        1
    );

    assert_eq!(
        result.args[0],
        "000000145468697320697320612073616d706c65207374720000000203e800000000000000000500f12dd10c4d2be8264fe339da14b9fad7bdf364ae7ceb"
    );
}

#[test]
fn test_create_vm_value_request_from_multi_value_encoded() {
    let _ = DebugApi::dummy();

    let buffer1: ManagedBuffer<DebugApi> = ManagedBuffer::from("first");
    let buffer2: ManagedBuffer<DebugApi> = ManagedBuffer::from("second");
    let buffer3: ManagedBuffer<DebugApi> = ManagedBuffer::from("third");

    let mut input_multi_value: MultiValueEncoded<DebugApi, ManagedBuffer<DebugApi>> = MultiValueEncoded::new();
    input_multi_value.push(buffer1);
    input_multi_value.push(buffer2);
    input_multi_value.push(buffer3);

    let mut result = VmValueRequest {
        sc_address: Address::from_bech32_string(CONTRACT_ADDRESS).unwrap(),
        func_name: FUNC_NAME.to_string(),
        caller: Address::from_bech32_string(CALLER_ADDRESS).unwrap(),
        value: VALUE.to_string(),
        args: vec![],
    };

    assert_common_vm_value_request_infos(&result);

    result.push_multi_managed_arg(input_multi_value);

    assert_eq!(
        result.args.len(),
        3
    );

    assert_eq!(
        result.args[0],
        "6669727374"
    );

    assert_eq!(
        result.args[1],
        "7365636f6e64"
    );

    assert_eq!(
        result.args[2],
        "7468697264"
    );
}

#[test]
fn test_create_vm_value_request_from_multiple_args() {
    let _ = DebugApi::dummy();

    let buffer1: ManagedBuffer<DebugApi> = ManagedBuffer::from("first");
    let buffer2: ManagedBuffer<DebugApi> = ManagedBuffer::from("second");
    let buffer3: ManagedBuffer<DebugApi> = ManagedBuffer::from("third");
    let buffer4: ManagedBuffer<DebugApi> = ManagedBuffer::from("fourth");

    let mut input_multi_value: MultiValueEncoded<DebugApi, ManagedBuffer<DebugApi>> = MultiValueEncoded::new();
    input_multi_value.push(buffer3);
    input_multi_value.push(buffer4);

    let mut result = VmValueRequest {
        sc_address: Address::from_bech32_string(CONTRACT_ADDRESS).unwrap(),
        func_name: FUNC_NAME.to_string(),
        caller: Address::from_bech32_string(CALLER_ADDRESS).unwrap(),
        value: VALUE.to_string(),
        args: vec![],
    };

    assert_common_vm_value_request_infos(&result);

    result.push_managed_arg(buffer1);
    result.push_managed_arg(buffer2);
    result.push_multi_managed_arg(input_multi_value);

    assert_eq!(
        result.args.len(),
        4
    );

    assert_eq!(
        result.args[0],
        "6669727374"
    );

    assert_eq!(
        result.args[1],
        "7365636f6e64"
    );

    assert_eq!(
        result.args[2],
        "7468697264"
    );

    assert_eq!(
        result.args[3],
        "666f75727468"
    );
}

#[test]
fn test_create_vm_value_request_from_multi_value_3() {
    let _ = DebugApi::dummy();

    let buffer1: ManagedBuffer<DebugApi> = ManagedBuffer::from("first");
    let buffer2: ManagedBuffer<DebugApi> = ManagedBuffer::from("second");
    let buffer3: ManagedBuffer<DebugApi> = ManagedBuffer::from("third");

    let input_multi_value: MultiValue3<ManagedBuffer<DebugApi>, ManagedBuffer<DebugApi>, ManagedBuffer<DebugApi>> = MultiValue3::from((
        buffer1,
        buffer2,
        buffer3
    ));

    let mut result = VmValueRequest {
        sc_address: Address::from_bech32_string(CONTRACT_ADDRESS).unwrap(),
        func_name: FUNC_NAME.to_string(),
        caller: Address::from_bech32_string(CALLER_ADDRESS).unwrap(),
        value: VALUE.to_string(),
        args: vec![],
    };

    assert_common_vm_value_request_infos(&result);

    result.push_multi_managed_arg(input_multi_value);

    assert_eq!(
        result.args.len(),
        3
    );

    assert_eq!(
        result.args[0],
        "6669727374"
    );

    assert_eq!(
        result.args[1],
        "7365636f6e64"
    );

    assert_eq!(
        result.args[2],
        "7468697264"
    );
}

#[test]
fn test_parse_return_data_buffer() {
    let _ = DebugApi::dummy();
    let from_index = 0;
    let to_index = 1;
    let result = get_dummy_output_result_from_data(&SAMPLE_RETURN_DATA[from_index..to_index]);
    let parsed_data = result.get_parsed_return_data::<ManagedBuffer<DebugApi>>(0).unwrap();

    let expected_result = String::from("This is a buffer");

    assert_eq!(
        parsed_data,
        expected_result
    );
}

#[test]
fn test_parse_return_data_buffer_another_index() {
    let _ = DebugApi::dummy();
    let from_index = 0;
    let to_index = 8;
    let result = get_dummy_output_result_from_data(&SAMPLE_RETURN_DATA[from_index..to_index]);
    let parsed_data = result.get_parsed_return_data::<ManagedBuffer<DebugApi>>(6).unwrap();

    let expected_result = String::from("second");

    assert_eq!(
        parsed_data,
        expected_result
    );
}

#[test]
fn test_parse_return_data_biguint() {
    let _ = DebugApi::dummy();
    let from_index = 1;
    let to_index = 2;
    let result = get_dummy_output_result_from_data(&SAMPLE_RETURN_DATA[from_index..to_index]);
    let parsed_data = result.get_parsed_return_data::<BigUint<DebugApi>>(0).unwrap();

    let expected_result: num_bigint::BigUint = num_bigint::BigUint::from(10u64).pow(18);

    assert_eq!(
        parsed_data,
        expected_result
    );
}

#[test]
fn test_parse_return_data_address() {
    let _ = DebugApi::dummy();
    let from_index = 2;
    let to_index = 3;
    let result = get_dummy_output_result_from_data(&SAMPLE_RETURN_DATA[from_index..to_index]);
    let parsed_data = result.get_parsed_return_data::<ManagedAddress<DebugApi>>(0).unwrap();

    let expected_address_bytes = Address::from_bech32_string(CONTRACT_ADDRESS).unwrap().to_bytes();

    assert_eq!(
        parsed_data.to_bytes(),
        expected_address_bytes
    );
}

#[test]
fn test_parse_return_data_struct() {
    let _ = DebugApi::dummy();
    let from_index = 3;
    let to_index = 4;
    let result = get_dummy_output_result_from_data(&SAMPLE_RETURN_DATA[from_index..to_index]);
    let parsed_data = result.get_parsed_return_data::<NativeValue<SampleCodable<DebugApi>>>(0).unwrap();

    let expected_address_bytes = Address::from_bech32_string(CONTRACT_ADDRESS).unwrap().to_bytes();

    let expected_result = SampleCodable {
        managed_buffer: ManagedBuffer::from("This is a sample str"),
        biguint: BigUint::from(1000u64),
        address: ManagedAddress::from(expected_address_bytes),
    };

    assert_eq!(
        parsed_data,
        expected_result
    );
}

#[test]
fn test_parse_return_data_struct_multi_value_encoded() {
    let _ = DebugApi::dummy();
    let from_index = 3;
    let datas = vec![SAMPLE_RETURN_DATA[from_index], SAMPLE_RETURN_DATA[from_index]];
    let result = get_dummy_output_result_from_data(&datas);
    let parsed_data = result.get_parsed_return_data_multi_in_range::<MultiValueEncoded<DebugApi, NativeValueManagedVecItem<SampleCodableManagedVecItem<DebugApi>>>>(0, 2).unwrap();

    let expected_address_bytes = Address::from_bech32_string(CONTRACT_ADDRESS).unwrap().to_bytes();

    let expected_struct = SampleCodableManagedVecItem {
        managed_buffer: ManagedBuffer::from("This is a sample str"),
        biguint: BigUint::from(1000u64),
        address: ManagedAddress::from(expected_address_bytes),
    };

    let expected_result: Vec<SampleCodableManagedVecItem<DebugApi>> = vec![
        expected_struct.clone(),
        expected_struct
    ];

    assert_eq!(
        parsed_data,
        expected_result
    );
}

#[test]
fn test_parse_return_data_multi_value_3() {
    let _ = DebugApi::dummy();
    let from_index = 4;
    let to_index = 7;
    let result = get_dummy_output_result_from_data(&SAMPLE_RETURN_DATA[from_index..to_index]);
    let parsed_data = result.get_parsed_return_data_multi_in_range::<MultiValue3<TokenIdentifier<DebugApi>, ManagedBuffer<DebugApi>, ManagedBuffer<DebugApi>>>(0, 3).unwrap();

    let expected_result = (
        String::from("WEGLD-abcdef"),
        String::from("first"),
        String::from("second")
    );

    assert_eq!(
        parsed_data,
        expected_result
    );
}

#[test]
fn test_parse_return_data_multi_value_encoded() {
    let _ = DebugApi::dummy();
    let from_index = 0;
    let to_index = 8;
    let result = get_dummy_output_result_from_data(&SAMPLE_RETURN_DATA[from_index..to_index]);
    let parsed_data = result.get_parsed_return_data_var_args::<ManagedBuffer<DebugApi>>(5).unwrap();

    let expected_result = vec![
        String::from("first"),
        String::from("second"),
        String::from("third")
    ];

    assert_eq!(
        parsed_data.len(),
        3
    );

    assert_eq!(
        parsed_data,
        expected_result
    );
}

#[test]
fn test_parse_return_data_primitives() {
    let _ = DebugApi::dummy();

    let number_data: [&str; 1] = [
        "Cg==" // 10
    ];

    let false_data: [&str; 1] = [
        ""
    ];

    let true_data: [&str; 1] = [
        "AQ=="
    ];

    let number_result = get_dummy_output_result_from_data(&number_data);
    let false_result = get_dummy_output_result_from_data(&false_data);
    let true_result = get_dummy_output_result_from_data(&true_data);

    let i8_number = number_result.get_parsed_return_data::<i8>(0).unwrap();
    let i16_number = number_result.get_parsed_return_data::<i16>(0).unwrap();
    let i32_number = number_result.get_parsed_return_data::<i32>(0).unwrap();
    let i64_number = number_result.get_parsed_return_data::<i64>(0).unwrap();
    let u8_number = number_result.get_parsed_return_data::<u8>(0).unwrap();
    let u16_number = number_result.get_parsed_return_data::<u16>(0).unwrap();
    let u32_number = number_result.get_parsed_return_data::<u32>(0).unwrap();
    let u64_number = number_result.get_parsed_return_data::<u64>(0).unwrap();
    let bool_false = false_result.get_parsed_return_data::<bool>(0).unwrap();
    let bool_true = true_result.get_parsed_return_data::<bool>(0).unwrap();

    assert_eq!(
        i8_number,
        10i8
    );

    assert_eq!(
        i16_number,
        10i16
    );

    assert_eq!(
        i32_number,
        10i32
    );

    assert_eq!(
        i64_number,
        10i64
    );

    assert_eq!(
        u8_number,
        10u8
    );

    assert_eq!(
        u16_number,
        10u16
    );

    assert_eq!(
        u32_number,
        10u32
    );

    assert_eq!(
        u64_number,
        10u64
    );

    assert_eq!(
        bool_false,
        false
    );

    assert_eq!(
        bool_true,
        true
    );
}

#[test]
fn test_parse_return_data_esdt_token_payment() {
    let _ = DebugApi::dummy();
    let result = get_dummy_output_result_from_data(&SAMPLE_ESDT_TOKEN_PAYMENT_DATA);

    let payment = result.get_parsed_return_data::<EsdtTokenPayment<DebugApi>>(0).unwrap();

    let expected_result = Payment {
        token_identifier: String::from("WEGLD-abcdef"),
        token_nonce: 5,
        amount: num_bigint::BigUint::from(1000u64)
    };

    assert_eq!(
        payment,
        expected_result
    )
}