use multiversx_chain_core::types::ReturnCode;
use multiversx_sdk::retrieve_tx_on_network::{
    extract_message_from_string_reason, find_code_and_message, parse_reason,
};

#[test]
fn parse_reason_only_message_test() {
    let reason = "@6f7574206f662066756e6473";
    let (code, message) = find_code_and_message(reason);
    assert!(code.is_none());
    assert_eq!("out of funds", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::OutOfFunds, code);
    assert_eq!("out of funds", message);
}

#[test]
fn parse_reason_empty_test() {
    let (code, message) = parse_reason("");
    assert_eq!(ReturnCode::UserError, code);
    assert_eq!("invalid transaction", message);
}

#[test]
fn parse_reason_test() {
    let reason = "@04@63616c6c56616c7565206e6f7420657175616c732077697468206261736549737375696e67436f7374@20248548f50a8fda29910e851c07c6c331a7f9e7784201ff2486be0934dbf612@bce4fac4ef67b79dbd2ce619b96fc7f51a3d36f68d5989b16b6fc1e47bde345d@ea64ae9803ff02ad12d496ab9e2838cc3ec3f9197973749610b4ccd4def8c1d1@00";

    let (code, message) = find_code_and_message(reason);
    assert_eq!(Some(ReturnCode::UserError), code);
    assert_eq!("callValue not equals with baseIssuingCost", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::UserError, code);
    assert_eq!("callValue not equals with baseIssuingCost", message);
}

#[test]
fn parse_reason_sc_panic_test() {
    let reason = "\n\truntime.go:856 [error signalled by smartcontract] [compoundRewards]\n\truntime.go:856 [error signalled by smartcontract] [compoundRewards]\n\truntime.go:853 [Guild closing]";
    let (code, message) = find_code_and_message(reason);
    assert!(code.is_none());
    assert!(message.is_empty());

    let message = extract_message_from_string_reason(reason);
    assert_eq!("Guild closing", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::UserError, code);
    assert_eq!("Guild closing", message);
}

#[test]
fn parse_reason_invalid_contract_test() {
    let reason = "\n\truntime.go:831 [invalid contract code (not found)] [buyCards]";
    let (code, message) = find_code_and_message(reason);
    assert!(code.is_none());
    assert!(message.is_empty());

    let message = extract_message_from_string_reason(reason);
    assert_eq!("invalid contract code (not found)", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::UserError, code);
    assert_eq!("invalid contract code (not found)", message);
}
