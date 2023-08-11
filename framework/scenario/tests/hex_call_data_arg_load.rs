#![feature(exhaustive_patterns)]

use multiversx_sc::{
    codec::{
        multi_types::{MultiValue2, MultiValueVec, OptionalValue},
        PanicErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
    },
    types::{AsyncCallResult, BigUint},
    HexCallDataDeserializer,
};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_simple_args() {
    let input: &[u8] = b"func@1111@2222";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(arg1) = i32::multi_decode_or_handle_err(&mut de, PanicErrorHandler);
    assert_eq!(arg1, 0x1111i32);

    let Ok(arg2) = i32::multi_decode_or_handle_err(&mut de, PanicErrorHandler);
    assert_eq!(arg2, 0x2222i32);

    de.assert_no_more_args(PanicErrorHandler).unwrap();
}

#[test]
fn test_simple_managed_arg() {
    let input: &[u8] = b"some_other_func@05";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(arg1) = BigUint::<StaticApi>::multi_decode_or_handle_err(&mut de, PanicErrorHandler);
    assert_eq!(arg1, BigUint::from(5u32));

    de.assert_no_more_args(PanicErrorHandler).unwrap();
}

#[test]
fn test_simple_vec_arg() {
    let input: &[u8] = b"some_other_func@000000020000000300000006";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(arg1) = Vec::<usize>::multi_decode_or_handle_err(&mut de, PanicErrorHandler);
    assert_eq!(arg1, [2usize, 3usize, 6usize].to_vec());

    de.assert_no_more_args(PanicErrorHandler).unwrap();
}

#[test]
fn test_var_args() {
    let input: &[u8] = b"func@1111@2222";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(var_arg) = MultiValueVec::<i32>::multi_decode_or_handle_err(&mut de, PanicErrorHandler);
    let arg_vec = var_arg.into_vec();
    assert_eq!(arg_vec.len(), 2);
    assert_eq!(arg_vec[0], 0x1111i32);
    assert_eq!(arg_vec[1], 0x2222i32);
}

#[test]
fn test_multi_arg_2() {
    let input: &[u8] = b"func@1111@2222";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(tuple_arg) =
        MultiValue2::<i32, i32>::multi_decode_or_handle_err(&mut de, PanicErrorHandler);
    let tuple = tuple_arg.into_tuple();
    assert_eq!(tuple.0, 0x1111i32);
    assert_eq!(tuple.1, 0x2222i32);
}

#[test]
fn test_var_multi_arg_2() {
    let input: &[u8] = b"func@1111@2222";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(tuple_arg) = MultiValueVec::<MultiValue2<i32, i32>>::multi_decode_or_handle_err(
        &mut de,
        PanicErrorHandler,
    );
    let tuple_vec = tuple_arg.into_vec();
    assert_eq!(tuple_vec.len(), 1);
    let mut iter = tuple_vec.into_iter();
    let tuple = iter.next().unwrap().into_tuple();
    assert_eq!(tuple.0, 0x1111i32);
    assert_eq!(tuple.1, 0x2222i32);
}

#[test]
fn test_opt_multi_arg_2() {
    let input: &[u8] = b"func@1111@2222";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(opt_tuple_arg) = OptionalValue::<MultiValue2<i32, i32>>::multi_decode_or_handle_err(
        &mut de,
        PanicErrorHandler,
    );
    match opt_tuple_arg {
        OptionalValue::Some(tuple_arg) => {
            let tuple = tuple_arg.into_tuple();
            assert_eq!(tuple.0, 0x1111i32);
            assert_eq!(tuple.1, 0x2222i32);
        },
        OptionalValue::None => {
            panic!("OptionalValue::Some expected");
        },
    }
}

#[test]
fn test_async_call_result_ok() {
    let input: &[u8] = b"func@@1111@2222";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(acr) = AsyncCallResult::<MultiValue2<i32, i32>>::multi_decode_or_handle_err(
        &mut de,
        PanicErrorHandler,
    );
    match acr {
        AsyncCallResult::Ok(tuple_arg) => {
            let tuple = tuple_arg.into_tuple();
            assert_eq!(tuple.0, 0x1111i32);
            assert_eq!(tuple.1, 0x2222i32);
        },
        AsyncCallResult::Err(_) => {
            panic!("AsyncCallResult::Ok expected");
        },
    }
}

#[test]
fn test_async_call_result_ok2() {
    let input: &[u8] = b"func@00";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(acr) =
        AsyncCallResult::<MultiValueVec<MultiValue2<i32, i32>>>::multi_decode_or_handle_err(
            &mut de,
            PanicErrorHandler,
        );
    match acr {
        AsyncCallResult::Ok(var_args) => {
            assert_eq!(var_args.len(), 0);
        },
        AsyncCallResult::Err(_) => {
            panic!("AsyncCallResult::Ok expected");
        },
    }
}

#[test]
fn test_async_call_result_err() {
    let input: &[u8] = b"func@0123@1111";
    let mut de = HexCallDataDeserializer::new(input);
    let Ok(acr) = AsyncCallResult::<MultiValue2<i32, i32>>::multi_decode_or_handle_err(
        &mut de,
        PanicErrorHandler,
    );
    match acr {
        AsyncCallResult::Ok(_) => {
            panic!("AsyncCallResult::Err expected");
        },
        AsyncCallResult::Err(async_call_error) => {
            assert_eq!(async_call_error.err_code, 0x0123);
            assert_eq!(async_call_error.err_msg.as_slice(), &[0x11u8, 0x11u8][..]);
        },
    }
}
