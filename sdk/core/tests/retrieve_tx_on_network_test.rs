use multiversx_chain_core::types::ReturnCode;
use multiversx_sdk::{
    data::transaction::TransactionOnNetwork,
    retrieve_tx_on_network::{
        extract_message_from_string_reason, find_code_and_message, parse_reason,
        replace_with_error_message,
    },
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

#[test]
fn replace_logs_reason_with_message_test() {
    let tx_str = r#"
{
    "type": "normal",
    "processingTypeOnSource": "SCInvoking",
    "processingTypeOnDestination": "SCInvoking",
    "hash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
    "nonce": 9785,
    "round": 5861933,
    "epoch": 2416,
    "value": "10000000000000000",
    "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
    "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
    "gasPrice": 1000000000,
    "gasLimit": 100000000,
    "gasUsed": 100000000,
    "data": "cmVnaXN0ZXJBbmRTZXRBbGxSb2xlc0A2OTZlNzQ2NTcyNjE2Mzc0NmY3MkA0OTRlNTQ1MkA0NjRlNDdA",
    "signature": "48cf58cc4064580cd5bdb48fa134d59f40ad4a8fed4de53fa29e160fa6e8dc438fbec234391f0668d65d99493f2d4f133691ee40be578e23680a923b503d1405",
    "sourceShard": 1,
    "destinationShard": 4294967295,
    "blockNonce": 5795888,
    "blockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "notarizedAtSourceInMetaNonce": 5795888,
    "NotarizedAtSourceInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "notarizedAtDestinationInMetaNonce": 5795888,
    "notarizedAtDestinationInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "miniblockType": "TxBlock",
    "miniblockHash": "029a399205b1274c87f672f07dddef4a7e37c44ed63f3c022db7d06b2dad6079",
    "hyperblockNonce": 5795888,
    "hyperblockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "timestamp": 1729171598,
    "smartContractResults": [
    {
        "hash": "76c83f623acd1ef2978353a5c2d04dc7fce727b1d48400215a3832e6c04c9dbe",
        "nonce": 9785,
        "value": 10000000000000000,
        "receiver": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
        "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
        "data": "@6f7574206f662066756e6473",
        "prevTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
        "originalTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
        "gasLimit": 0,
        "gasPrice": 0,
        "callType": 0,
        "returnMessage": "callValue not equals with baseIssuingCost",
        "originalSender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
        "operation": "transfer"
    }
    ],
    "logs": {
    "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
    "events": [
        {
        "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
        "identifier": "signalError",
        "topics": [
            "ATlHLv9ohncamC8wg9pdQh8kwpGB5jiIIo3IHKYNaeE=",
            "Y2FsbFZhbHVlIG5vdCBlcXVhbHMgd2l0aCBiYXNlSXNzdWluZ0Nvc3Q="
        ],
        "data": "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw==",
        "additionalData": [
            "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw=="
        ]
        }
    ]
    },
    "status": "success",
    "operation": "transfer",
    "function": "registerAndSetAllRoles",
    "initiallyPaidFee": "1138600000000000",
    "fee": "1138600000000000",
    "chainID": "D",
    "version": 1,
    "options": 0
}"#;

    let expected_tx_str = r#"
{
    "type": "normal",
    "processingTypeOnSource": "SCInvoking",
    "processingTypeOnDestination": "SCInvoking",
    "hash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
    "nonce": 9785,
    "round": 5861933,
    "epoch": 2416,
    "value": "10000000000000000",
    "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
    "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
    "gasPrice": 1000000000,
    "gasLimit": 100000000,
    "gasUsed": 100000000,
    "data": "cmVnaXN0ZXJBbmRTZXRBbGxSb2xlc0A2OTZlNzQ2NTcyNjE2Mzc0NmY3MkA0OTRlNTQ1MkA0NjRlNDdA",
    "signature": "48cf58cc4064580cd5bdb48fa134d59f40ad4a8fed4de53fa29e160fa6e8dc438fbec234391f0668d65d99493f2d4f133691ee40be578e23680a923b503d1405",
    "sourceShard": 1,
    "destinationShard": 4294967295,
    "blockNonce": 5795888,
    "blockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "notarizedAtSourceInMetaNonce": 5795888,
    "NotarizedAtSourceInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "notarizedAtDestinationInMetaNonce": 5795888,
    "notarizedAtDestinationInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "miniblockType": "TxBlock",
    "miniblockHash": "029a399205b1274c87f672f07dddef4a7e37c44ed63f3c022db7d06b2dad6079",
    "hyperblockNonce": 5795888,
    "hyperblockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
    "timestamp": 1729171598,
    "smartContractResults": [
    {
        "hash": "76c83f623acd1ef2978353a5c2d04dc7fce727b1d48400215a3832e6c04c9dbe",
        "nonce": 9785,
        "value": 10000000000000000,
        "receiver": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
        "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
        "data": "@6f7574206f662066756e6473",
        "prevTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
        "originalTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
        "gasLimit": 0,
        "gasPrice": 0,
        "callType": 0,
        "returnMessage": "callValue not equals with baseIssuingCost",
        "originalSender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
        "operation": "transfer"
    }
    ],
    "logs": {
    "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
    "events": [
        {
        "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
        "identifier": "signalError",
        "topics": [
            "ATlHLv9ohncamC8wg9pdQh8kwpGB5jiIIo3IHKYNaeE=",
            "b3V0IG9mIGZ1bmRz"
        ],
        "data": "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw==",
        "additionalData": [
            "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw=="
        ]
        }
    ]
    },
    "status": "success",
    "operation": "transfer",
    "function": "registerAndSetAllRoles",
    "initiallyPaidFee": "1138600000000000",
    "fee": "1138600000000000",
    "chainID": "D",
    "version": 1,
    "options": 0
}"#;

    let mut tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(tx_str).unwrap();
    let expected_tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(expected_tx_str).unwrap();

    replace_with_error_message(&mut tx, "b3V0IG9mIGZ1bmRz");
    assert_eq!(
        expected_tx.logs.unwrap().events[0].topics,
        tx.logs.unwrap().events[0].topics
    );
}

#[test]
fn replace_logs_parse_empty_reason_test() {
    let tx_str = r#"
{
    "type": "invalid",
    "processingTypeOnSource": "BuiltInFunctionCall",
    "processingTypeOnDestination": "BuiltInFunctionCall",
    "hash": "d9db8aa162e1ad70e962b5152e8fd182144aa565ff8469dfcf6d2d36526f08b0",
    "nonce": 90,
    "round": 5574021,
    "epoch": 2296,
    "value": "0",
    "receiver": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
    "sender": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
    "gasPrice": 1000000000,
    "gasLimit": 1276500,
    "gasUsed": 1276500,
    "data": "TXVsdGlFU0RUTkZUVHJhbnNmZXJAMGRmNWFhMWU5MTE5NGQ3YzQ2ZTQ4MTVmNjE1YjhhY2Y3ZmE3YzI5YmViYzQ4NTg1MWQ5ZTIyZmRkOWE3MzhkN0AwMUAzMTY1NjYzNzYzNjQzNTM1MmQzNzM1MzQzNTJkMzY2NTM4MzYyZDMwMzkzNjM4QDZhYmFhMzNkMDk2ZUAwMQ==",
    "signature": "511b1ae0ee25a3e4e297f8c19a5ef337beb0bafd06b87064532d3a51d4d609aac6f6e9c39ffdc961210e90d78b2abcc00c8f18f1060dfa3b79a91c1ff258de09",
    "sourceShard": 1,
    "destinationShard": 1,
    "blockNonce": 5505400,
    "blockHash": "6011da28c01082b03a1a294601d12f191d210058f0dd69232dd7ee67b02af15c",
    "notarizedAtSourceInMetaNonce": 5508839,
    "NotarizedAtSourceInMetaHash": "aeeb8315a4a3b4893641b7f95d49435a96a54e76d7d45d9fb79c24a0cd576274",
    "notarizedAtDestinationInMetaNonce": 5508839,
    "notarizedAtDestinationInMetaHash": "aeeb8315a4a3b4893641b7f95d49435a96a54e76d7d45d9fb79c24a0cd576274",
    "miniblockType": "InvalidBlock",
    "miniblockHash": "7663b45144fd531fbb7bfa726dee876862853e22c42bc9b2c845742b4f03189a",
    "hyperblockNonce": 5508839,
    "hyperblockHash": "aeeb8315a4a3b4893641b7f95d49435a96a54e76d7d45d9fb79c24a0cd576274",
    "timestamp": 1727444126,
    "logs": {
        "address": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
        "events": [
          {
            "address": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
            "identifier": "signalError",
            "topics": [
              "9CzJan/Zokd07a/9WFAo1ikG1771CP4FXb3G4WnUHa0=",
              "bmV3IE5GVCBkYXRhIG9uIHNlbmRlciBmb3IgdG9rZW4gMWVmN2NkNTUtNzU0NS02ZTg2LTA5Njg="
            ],
            "data": "QDZlNjU3NzIwNGU0NjU0MjA2NDYxNzQ2MTIwNmY2ZTIwNzM2NTZlNjQ2NTcyMjA2NjZmNzIyMDc0NmY2YjY1NmUyMDMxNjU2NjM3NjM2NDM1MzUyZDM3MzUzNDM1MmQzNjY1MzgzNjJkMzAzOTM2Mzg=",
            "additionalData": [
              "QDZlNjU3NzIwNGU0NjU0MjA2NDYxNzQ2MTIwNmY2ZTIwNzM2NTZlNjQ2NTcyMjA2NjZmNzIyMDc0NmY2YjY1NmUyMDMxNjU2NjM3NjM2NDM1MzUyZDM3MzUzNDM1MmQzNjY1MzgzNjJkMzAzOTM2Mzg="
            ]
          }
        ]
    },
    "status": "invalid",
    "tokens": [
        "1ef7cd55-7545-6e86-0968-6abaa33d096e"
    ],
    "esdtValues": [
        "1"
    ],
    "receivers": [
        "erd1ph665853r9xhc3hys90kzku2eal60s5ma0zgtpganc30mkd88rtsq3wsc7"
    ],
    "receiversShardIDs": [
        1
    ],
    "operation": "MultiESDTNFTTransfer",
    "initiallyPaidFee": "286500000000000",
    "fee": "286500000000000",
    "chainID": "D",
    "version": 1,
    "options": 0
}
"#;

    let expected_tx_str = r#"
{
    "type": "invalid",
    "processingTypeOnSource": "BuiltInFunctionCall",
    "processingTypeOnDestination": "BuiltInFunctionCall",
    "hash": "d9db8aa162e1ad70e962b5152e8fd182144aa565ff8469dfcf6d2d36526f08b0",
    "nonce": 90,
    "round": 5574021,
    "epoch": 2296,
    "value": "0",
    "receiver": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
    "sender": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
    "gasPrice": 1000000000,
    "gasLimit": 1276500,
    "gasUsed": 1276500,
    "data": "TXVsdGlFU0RUTkZUVHJhbnNmZXJAMGRmNWFhMWU5MTE5NGQ3YzQ2ZTQ4MTVmNjE1YjhhY2Y3ZmE3YzI5YmViYzQ4NTg1MWQ5ZTIyZmRkOWE3MzhkN0AwMUAzMTY1NjYzNzYzNjQzNTM1MmQzNzM1MzQzNTJkMzY2NTM4MzYyZDMwMzkzNjM4QDZhYmFhMzNkMDk2ZUAwMQ==",
    "signature": "511b1ae0ee25a3e4e297f8c19a5ef337beb0bafd06b87064532d3a51d4d609aac6f6e9c39ffdc961210e90d78b2abcc00c8f18f1060dfa3b79a91c1ff258de09",
    "sourceShard": 1,
    "destinationShard": 1,
    "blockNonce": 5505400,
    "blockHash": "6011da28c01082b03a1a294601d12f191d210058f0dd69232dd7ee67b02af15c",
    "notarizedAtSourceInMetaNonce": 5508839,
    "NotarizedAtSourceInMetaHash": "aeeb8315a4a3b4893641b7f95d49435a96a54e76d7d45d9fb79c24a0cd576274",
    "notarizedAtDestinationInMetaNonce": 5508839,
    "notarizedAtDestinationInMetaHash": "aeeb8315a4a3b4893641b7f95d49435a96a54e76d7d45d9fb79c24a0cd576274",
    "miniblockType": "InvalidBlock",
    "miniblockHash": "7663b45144fd531fbb7bfa726dee876862853e22c42bc9b2c845742b4f03189a",
    "hyperblockNonce": 5508839,
    "hyperblockHash": "aeeb8315a4a3b4893641b7f95d49435a96a54e76d7d45d9fb79c24a0cd576274",
    "timestamp": 1727444126,
    "logs": {
        "address": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
        "events": [
          {
            "address": "erd17skvj6nlmx3ywa8d4l74s5pg6c5sd4a775y0up2ahhrwz6w5rkks9mag5m",
            "identifier": "signalError",
            "topics": [
              "9CzJan/Zokd07a/9WFAo1ikG1771CP4FXb3G4WnUHa0=",
              "bmV3IE5GVCBkYXRhIG9uIHNlbmRlciBmb3IgdG9rZW4gMWVmN2NkNTUtNzU0NS02ZTg2LTA5Njg="
            ],
            "data": "QDZlNjU3NzIwNGU0NjU0MjA2NDYxNzQ2MTIwNmY2ZTIwNzM2NTZlNjQ2NTcyMjA2NjZmNzIyMDc0NmY2YjY1NmUyMDMxNjU2NjM3NjM2NDM1MzUyZDM3MzUzNDM1MmQzNjY1MzgzNjJkMzAzOTM2Mzg=",
            "additionalData": [
              "QDZlNjU3NzIwNGU0NjU0MjA2NDYxNzQ2MTIwNmY2ZTIwNzM2NTZlNjQ2NTcyMjA2NjZmNzIyMDc0NmY2YjY1NmUyMDMxNjU2NjM3NjM2NDM1MzUyZDM3MzUzNDM1MmQzNjY1MzgzNjJkMzAzOTM2Mzg="
            ]
          }
        ]
    },
    "status": "invalid",
    "tokens": [
        "1ef7cd55-7545-6e86-0968-6abaa33d096e"
    ],
    "esdtValues": [
        "1"
    ],
    "receivers": [
        "erd1ph665853r9xhc3hys90kzku2eal60s5ma0zgtpganc30mkd88rtsq3wsc7"
    ],
    "receiversShardIDs": [
        1
    ],
    "operation": "MultiESDTNFTTransfer",
    "initiallyPaidFee": "286500000000000",
    "fee": "286500000000000",
    "chainID": "D",
    "version": 1,
    "options": 0
}
"#;

    let mut tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(tx_str).unwrap();
    let expected_tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(expected_tx_str).unwrap();

    replace_with_error_message(&mut tx, "");
    assert_eq!(
        expected_tx.logs.unwrap().events[0].topics,
        tx.logs.unwrap().events[0].topics
    );
}

#[test]
fn replace_logs_parse_reason_test() {
    let tx_str = r#"
{
    "type": "normal",
    "processingTypeOnSource": "SCInvoking",
    "processingTypeOnDestination": "SCInvoking",
    "hash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
    "nonce": 8683,
    "round": 5874368,
    "epoch": 2422,
    "value": "0",
    "receiver": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
    "sender": "erd1m63u4asyua53ghpqxlj4qfmtceg9m23mcejz92eddq6qd8k2s48sx54qpc",
    "gasPrice": 1000000000,
    "gasLimit": 80000000,
    "gasUsed": 80000000,
    "data": "d2l0aGRyYXdGcm9tQDAwMDAwMDAwMDAwMDAwMDAwMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDZmZmZmZmY=",
    "signature": "5970b4fa29e50b2547bfefaae8de4e09a302dd64b1619ac4dc9985335c370d2c8aca8d3d52378018f67a11427ac779709c9f5a82cd38aec804ae7a228195c907",
    "sourceShard": 1,
    "destinationShard": 1,
    "blockNonce": 5805005,
    "blockHash": "45ce355e858955fa1015fbdda7342caa11c2bac30309aa85d18e08d264c03bc0",
    "notarizedAtSourceInMetaNonce": 5808320,
    "NotarizedAtSourceInMetaHash": "51ca88f7fab274547729756415b9cb9ff741086d943a5222cac864998373a786",
    "notarizedAtDestinationInMetaNonce": 5808320,
    "notarizedAtDestinationInMetaHash": "51ca88f7fab274547729756415b9cb9ff741086d943a5222cac864998373a786",
    "miniblockType": "TxBlock",
    "miniblockHash": "135b568551786d02312c0fa8edde5b0ed9f5db9f5c0c44fc01da61cc7e6af3b9",
    "hyperblockNonce": 5808320,
    "hyperblockHash": "51ca88f7fab274547729756415b9cb9ff741086d943a5222cac864998373a786",
    "timestamp": 1729246208,
    "smartContractResults": [
        {
          "hash": "dc52b9660d836289f86c5766879b6c5e26240ef2c9a895d0e918c1d2d484bca8",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
          "sender": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "data": "withdraw@33ab9d0628dc69b84cf0af7d74038e11aca9cb9ba8ace6c96e98cac550db2323@92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be@800ffc",
          "prevTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "originalTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "gasLimit": 73120670,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1m63u4asyua53ghpqxlj4qfmtceg9m23mcejz92eddq6qd8k2s48sx54qpc",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
                "identifier": "signalError",
                "topics": [
                  "AAAAAAAAAAAFAHKmRmOde7umzArGPMxtTcuRf8O8Y90=",
                  "Y2FsbGVyIGlzIG5vdCBhIGRlbGVnYXRvcg=="
                ],
                "data": "QDA0QDYzNjE2YzZjNjU3MjIwNjk3MzIwNmU2Zjc0MjA2MTIwNjQ2NTZjNjU2NzYxNzQ2ZjcyQDU4NGZkZDUzM2Q2ZGIwYjAxZmRlZDMxNjZhNDA2ZGMxNzlkZDJkMDc5NjgzYzVlMjVlN2E5YmVhOWM0MWE4NjhAMzNhYjlkMDYyOGRjNjliODRjZjBhZjdkNzQwMzhlMTFhY2E5Y2I5YmE4YWNlNmM5NmU5OGNhYzU1MGRiMjMyM0A5MmIyOWI1MzMwNDFmZWVjYzgwYThiZDlkM2VkM2E3NTU2NzY1MjQ4ZDJlZjAwNjdiMTQ4ZDcyMmQ4ZjY2MWJlQDAw",
                "additionalData": [
                  "QDA0QDYzNjE2YzZjNjU3MjIwNjk3MzIwNmU2Zjc0MjA2MTIwNjQ2NTZjNjU2NzYxNzQ2ZjcyQDU4NGZkZDUzM2Q2ZGIwYjAxZmRlZDMxNjZhNDA2ZGMxNzlkZDJkMDc5NjgzYzVlMjVlN2E5YmVhOWM0MWE4NjhAMzNhYjlkMDYyOGRjNjliODRjZjBhZjdkNzQwMzhlMTFhY2E5Y2I5YmE4YWNlNmM5NmU5OGNhYzU1MGRiMjMyM0A5MmIyOWI1MzMwNDFmZWVjYzgwYThiZDlkM2VkM2E3NTU2NzY1MjQ4ZDJlZjAwNjdiMTQ4ZDcyMmQ4ZjY2MWJlQDAw"
                ]
              }
            ]
          },
          "operation": "transfer",
          "function": "withdraw"
        },
        {
          "hash": "98fd84d6b0259262c4dd369b2928a237f62f4c1f40f2c5a83af8b2f6c0db9bdd",
          "nonce": 1,
          "value": 16872570000000,
          "receiver": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "sender": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "data": "@6f6b",
          "prevTxHash": "483a12a73eb0e1a6842e54a6e0ff00cf8d35ed02ead6930728ac5dfd63593aef",
          "originalTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        },
        {
          "hash": "483a12a73eb0e1a6842e54a6e0ff00cf8d35ed02ead6930728ac5dfd63593aef",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
          "data": "@04@63616c6c6572206973206e6f7420612064656c656761746f72@584fdd533d6db0b01fded3166a406dc179dd2d079683c5e25e7a9bea9c41a868@33ab9d0628dc69b84cf0af7d74038e11aca9cb9ba8ace6c96e98cac550db2323@92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be@00",
          "prevTxHash": "dc52b9660d836289f86c5766879b6c5e26240ef2c9a895d0e918c1d2d484bca8",
          "originalTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "gasLimit": 8392700,
          "gasPrice": 1000000000,
          "callType": 2,
          "returnMessage": "caller is not a delegator",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
                "identifier": "callBack",
                "topics": [
                  "YXN5bmNfY2FsbF9lcnJvcl9ldmVudA==",
                  "BA==",
                  "Y2FsbGVyIGlzIG5vdCBhIGRlbGVnYXRvcg=="
                ],
                "data": null,
                "additionalData": [
                  ""
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
                "identifier": "completedTxEvent",
                "topics": [
                  "3FK5Zg2DYon4bFdmh5tsXiYkDvLJqJXQ6RjB0tSEvKg="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        }
    ],
    "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAb///8="
            ],
            "data": "QXN5bmNDYWxs",
            "additionalData": [
              "QXN5bmNDYWxs",
              "d2l0aGRyYXc="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
            "identifier": "writeLog",
            "topics": [
              "3qPK9gTnaRRcIDflUCdrxlBdqjvGZCKrLWg0Bp7KhU8="
            ],
            "data": "QDZmNmI=",
            "additionalData": [
              "QDZmNmI="
            ]
            }
        ]
    },
      "status": "success",
      "operation": "transfer",
      "function": "withdrawFrom",
      "initiallyPaidFee": "963845000000000",
      "fee": "963845000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
"#;

    let expected_tx_str = r#"
{
    "type": "normal",
    "processingTypeOnSource": "SCInvoking",
    "processingTypeOnDestination": "SCInvoking",
    "hash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
    "nonce": 8683,
    "round": 5874368,
    "epoch": 2422,
    "value": "0",
    "receiver": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
    "sender": "erd1m63u4asyua53ghpqxlj4qfmtceg9m23mcejz92eddq6qd8k2s48sx54qpc",
    "gasPrice": 1000000000,
    "gasLimit": 80000000,
    "gasUsed": 80000000,
    "data": "d2l0aGRyYXdGcm9tQDAwMDAwMDAwMDAwMDAwMDAwMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDZmZmZmZmY=",
    "signature": "5970b4fa29e50b2547bfefaae8de4e09a302dd64b1619ac4dc9985335c370d2c8aca8d3d52378018f67a11427ac779709c9f5a82cd38aec804ae7a228195c907",
    "sourceShard": 1,
    "destinationShard": 1,
    "blockNonce": 5805005,
    "blockHash": "45ce355e858955fa1015fbdda7342caa11c2bac30309aa85d18e08d264c03bc0",
    "notarizedAtSourceInMetaNonce": 5808320,
    "NotarizedAtSourceInMetaHash": "51ca88f7fab274547729756415b9cb9ff741086d943a5222cac864998373a786",
    "notarizedAtDestinationInMetaNonce": 5808320,
    "notarizedAtDestinationInMetaHash": "51ca88f7fab274547729756415b9cb9ff741086d943a5222cac864998373a786",
    "miniblockType": "TxBlock",
    "miniblockHash": "135b568551786d02312c0fa8edde5b0ed9f5db9f5c0c44fc01da61cc7e6af3b9",
    "hyperblockNonce": 5808320,
    "hyperblockHash": "51ca88f7fab274547729756415b9cb9ff741086d943a5222cac864998373a786",
    "timestamp": 1729246208,
    "smartContractResults": [
        {
          "hash": "dc52b9660d836289f86c5766879b6c5e26240ef2c9a895d0e918c1d2d484bca8",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
          "sender": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "data": "withdraw@33ab9d0628dc69b84cf0af7d74038e11aca9cb9ba8ace6c96e98cac550db2323@92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be@800ffc",
          "prevTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "originalTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "gasLimit": 73120670,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1m63u4asyua53ghpqxlj4qfmtceg9m23mcejz92eddq6qd8k2s48sx54qpc",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
                "identifier": "signalError",
                "topics": [
                  "AAAAAAAAAAAFAHKmRmOde7umzArGPMxtTcuRf8O8Y90=",
                  "Y2FsbGVyIGlzIG5vdCBhIGRlbGVnYXRvcg=="
                ],
                "data": "QDA0QDYzNjE2YzZjNjU3MjIwNjk3MzIwNmU2Zjc0MjA2MTIwNjQ2NTZjNjU2NzYxNzQ2ZjcyQDU4NGZkZDUzM2Q2ZGIwYjAxZmRlZDMxNjZhNDA2ZGMxNzlkZDJkMDc5NjgzYzVlMjVlN2E5YmVhOWM0MWE4NjhAMzNhYjlkMDYyOGRjNjliODRjZjBhZjdkNzQwMzhlMTFhY2E5Y2I5YmE4YWNlNmM5NmU5OGNhYzU1MGRiMjMyM0A5MmIyOWI1MzMwNDFmZWVjYzgwYThiZDlkM2VkM2E3NTU2NzY1MjQ4ZDJlZjAwNjdiMTQ4ZDcyMmQ4ZjY2MWJlQDAw",
                "additionalData": [
                  "QDA0QDYzNjE2YzZjNjU3MjIwNjk3MzIwNmU2Zjc0MjA2MTIwNjQ2NTZjNjU2NzYxNzQ2ZjcyQDU4NGZkZDUzM2Q2ZGIwYjAxZmRlZDMxNjZhNDA2ZGMxNzlkZDJkMDc5NjgzYzVlMjVlN2E5YmVhOWM0MWE4NjhAMzNhYjlkMDYyOGRjNjliODRjZjBhZjdkNzQwMzhlMTFhY2E5Y2I5YmE4YWNlNmM5NmU5OGNhYzU1MGRiMjMyM0A5MmIyOWI1MzMwNDFmZWVjYzgwYThiZDlkM2VkM2E3NTU2NzY1MjQ4ZDJlZjAwNjdiMTQ4ZDcyMmQ4ZjY2MWJlQDAw"
                ]
              }
            ]
          },
          "operation": "transfer",
          "function": "withdraw"
        },
        {
          "hash": "98fd84d6b0259262c4dd369b2928a237f62f4c1f40f2c5a83af8b2f6c0db9bdd",
          "nonce": 1,
          "value": 16872570000000,
          "receiver": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "sender": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "data": "@6f6b",
          "prevTxHash": "483a12a73eb0e1a6842e54a6e0ff00cf8d35ed02ead6930728ac5dfd63593aef",
          "originalTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        },
        {
          "hash": "483a12a73eb0e1a6842e54a6e0ff00cf8d35ed02ead6930728ac5dfd63593aef",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqphllllsndz99p",
          "data": "@04@63616c6c6572206973206e6f7420612064656c656761746f72@584fdd533d6db0b01fded3166a406dc179dd2d079683c5e25e7a9bea9c41a868@33ab9d0628dc69b84cf0af7d74038e11aca9cb9ba8ace6c96e98cac550db2323@92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be@00",
          "prevTxHash": "dc52b9660d836289f86c5766879b6c5e26240ef2c9a895d0e918c1d2d484bca8",
          "originalTxHash": "92b29b533041feecc80a8bd9d3ed3a7556765248d2ef0067b148d722d8f661be",
          "gasLimit": 8392700,
          "gasPrice": 1000000000,
          "callType": 2,
          "returnMessage": "caller is not a delegator",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
                "identifier": "callBack",
                "topics": [
                  "YXN5bmNfY2FsbF9lcnJvcl9ldmVudA==",
                  "BA==",
                  "Y2FsbGVyIGlzIG5vdCBhIGRlbGVnYXRvcg=="
                ],
                "data": null,
                "additionalData": [
                  ""
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
                "identifier": "completedTxEvent",
                "topics": [
                  "3FK5Zg2DYon4bFdmh5tsXiYkDvLJqJXQ6RjB0tSEvKg="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        }
      ],
    "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAb///8="
            ],
            "data": "QXN5bmNDYWxs",
            "additionalData": [
              "QXN5bmNDYWxs",
              "d2l0aGRyYXc="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqw2nyvcua0wa6dnq2cc7vcm2dewghlsauv0wsrhv9ff",
            "identifier": "writeLog",
            "topics": [
              "3qPK9gTnaRRcIDflUCdrxlBdqjvGZCKrLWg0Bp7KhU8="
            ],
            "data": "QDZmNmI=",
            "additionalData": [
              "QDZmNmI="
            ]
          }
        ]
    },
    "status": "success",
    "operation": "transfer",
    "function": "withdrawFrom",
    "initiallyPaidFee": "963845000000000",
    "fee": "963845000000000",
    "chainID": "D",
    "version": 1,
    "options": 0
}"#;

    let mut tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(tx_str).unwrap();
    let expected_tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(expected_tx_str).unwrap();

    replace_with_error_message(&mut tx, "Y2FsbGVyIGlzIG5vdCBhIGRlbGVnYXRvcg==");
    assert_eq!(
        expected_tx.logs.unwrap().events[0].topics,
        tx.logs.unwrap().events[0].topics
    );
}

#[test]
fn replace_logs_reason_sc_panic_test() {
    let tx_str = r#"
    {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "8947aab38f2b37539327ecff8761d0d13c76db4ee0c7acafae1839464ee4c788",
      "nonce": 475,
      "round": 5874477,
      "epoch": 2422,
      "value": "0",
      "receiver": "erd1qqqqqqqqqqqqqpgq965hzhtvu6glhf7t4vcqpmssfxhssplea2wsgehtzu",
      "sender": "erd1akawdpkphvrwg5vn3fxyqg0tghzkumzugwutqyy0ew8xalfaa2ws3rxrh9",
      "gasPrice": 1000000000,
      "gasLimit": 39000000,
      "gasUsed": 39000000,
      "data": "ZUBkYjc1MTFAMDFhNkBlM0AxMGFlYzBAMDFhNkBlM0AwNg==",
      "signature": "b26ea25197c7b93400294f99c723cecd3b6bb3219d1c334250a9109c21e2f47775f85f4b57e4a19d1a27b3751a21432dd976c8f1331bacc999ef76defa150800",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 5805114,
      "blockHash": "9d168a8621ca44ea6a595697eb74486ee5caf97353d0a80b5b6b509720f9d86a",
      "notarizedAtSourceInMetaNonce": 5808429,
      "NotarizedAtSourceInMetaHash": "b88cf2f3b6db8da4bfca18b55ae1e53db1cbf1e884b12a5ff7aadb4e4ee20fc5",
      "notarizedAtDestinationInMetaNonce": 5808429,
      "notarizedAtDestinationInMetaHash": "b88cf2f3b6db8da4bfca18b55ae1e53db1cbf1e884b12a5ff7aadb4e4ee20fc5",
      "miniblockType": "TxBlock",
      "miniblockHash": "849e488d10e94c470e724cd8792117d4566ef3e2ad9a6360ccdea4edff43e279",
      "hyperblockNonce": 5808429,
      "hyperblockHash": "b88cf2f3b6db8da4bfca18b55ae1e53db1cbf1e884b12a5ff7aadb4e4ee20fc5",
      "timestamp": 1729246862,
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgq965hzhtvu6glhf7t4vcqpmssfxhssplea2wsgehtzu",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq965hzhtvu6glhf7t4vcqpmssfxhssplea2wsgehtzu",
            "identifier": "signalError",
            "topics": [
              "7brmhsG7BuRRk4pMQCHrRcVubFxDuLAQj8uObv096p0=",
              "c3RvcmFnZSBkZWNvZGUgZXJyb3IgKGtleTogcG9vbENvbnRyYWN0AAAAAAAAAaYpOiBpbnB1dCB0b28gc2hvcnQ="
            ],
            "data": "QDc1NzM2NTcyMjA2NTcyNzI2Zjcy",
            "additionalData": [
              "QDc1NzM2NTcyMjA2NTcyNzI2Zjcy"
            ]
          },
          {
            "address": "erd1akawdpkphvrwg5vn3fxyqg0tghzkumzugwutqyy0ew8xalfaa2ws3rxrh9",
            "identifier": "internalVMErrors",
            "topics": [
              "AAAAAAAAAAAFAC6pcV1s5pH7p8urMADuEEmvCAf56p0=",
              "ZQ=="
            ],
            "data": "CglydW50aW1lLmdvOjg1NiBbZXJyb3Igc2lnbmFsbGVkIGJ5IHNtYXJ0Y29udHJhY3RdIFtlXQoJcnVudGltZS5nbzo4NTYgW2Vycm9yIHNpZ25hbGxlZCBieSBzbWFydGNvbnRyYWN0XSBbZV0KCXJ1bnRpbWUuZ286ODUzIFtzdG9yYWdlIGRlY29kZSBlcnJvciAoa2V5OiBwb29sQ29udHJhY3QAAAAAAAABpik6IGlucHV0IHRvbyBzaG9ydF0=",
            "additionalData": [
              "CglydW50aW1lLmdvOjg1NiBbZXJyb3Igc2lnbmFsbGVkIGJ5IHNtYXJ0Y29udHJhY3RdIFtlXQoJcnVudGltZS5nbzo4NTYgW2Vycm9yIHNpZ25hbGxlZCBieSBzbWFydGNvbnRyYWN0XSBbZV0KCXJ1bnRpbWUuZ286ODUzIFtzdG9yYWdlIGRlY29kZSBlcnJvciAoa2V5OiBwb29sQ29udHJhY3QAAAAAAAABpik6IGlucHV0IHRvbyBzaG9ydF0="
            ]
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "e",
      "initiallyPaidFee": "489990000000000",
      "fee": "489990000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
    "#;

    let expected_tx_str = r#"
    {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "8947aab38f2b37539327ecff8761d0d13c76db4ee0c7acafae1839464ee4c788",
      "nonce": 475,
      "round": 5874477,
      "epoch": 2422,
      "value": "0",
      "receiver": "erd1qqqqqqqqqqqqqpgq965hzhtvu6glhf7t4vcqpmssfxhssplea2wsgehtzu",
      "sender": "erd1akawdpkphvrwg5vn3fxyqg0tghzkumzugwutqyy0ew8xalfaa2ws3rxrh9",
      "gasPrice": 1000000000,
      "gasLimit": 39000000,
      "gasUsed": 39000000,
      "data": "ZUBkYjc1MTFAMDFhNkBlM0AxMGFlYzBAMDFhNkBlM0AwNg==",
      "signature": "b26ea25197c7b93400294f99c723cecd3b6bb3219d1c334250a9109c21e2f47775f85f4b57e4a19d1a27b3751a21432dd976c8f1331bacc999ef76defa150800",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 5805114,
      "blockHash": "9d168a8621ca44ea6a595697eb74486ee5caf97353d0a80b5b6b509720f9d86a",
      "notarizedAtSourceInMetaNonce": 5808429,
      "NotarizedAtSourceInMetaHash": "b88cf2f3b6db8da4bfca18b55ae1e53db1cbf1e884b12a5ff7aadb4e4ee20fc5",
      "notarizedAtDestinationInMetaNonce": 5808429,
      "notarizedAtDestinationInMetaHash": "b88cf2f3b6db8da4bfca18b55ae1e53db1cbf1e884b12a5ff7aadb4e4ee20fc5",
      "miniblockType": "TxBlock",
      "miniblockHash": "849e488d10e94c470e724cd8792117d4566ef3e2ad9a6360ccdea4edff43e279",
      "hyperblockNonce": 5808429,
      "hyperblockHash": "b88cf2f3b6db8da4bfca18b55ae1e53db1cbf1e884b12a5ff7aadb4e4ee20fc5",
      "timestamp": 1729246862,
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgq965hzhtvu6glhf7t4vcqpmssfxhssplea2wsgehtzu",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq965hzhtvu6glhf7t4vcqpmssfxhssplea2wsgehtzu",
            "identifier": "signalError",
            "topics": [
              "7brmhsG7BuRRk4pMQCHrRcVubFxDuLAQj8uObv096p0=",
              "c3RvcmFnZSBkZWNvZGUgZXJyb3IgKGtleTogcG9vbENvbnRyYWN0AAAAAAAAAe+/vSk6IGlucHV0IHRvbyBzaG9ydA=="
            ],
            "data": "QDc1NzM2NTcyMjA2NTcyNzI2Zjcy",
            "additionalData": [
              "QDc1NzM2NTcyMjA2NTcyNzI2Zjcy"
            ]
          },
          {
            "address": "erd1akawdpkphvrwg5vn3fxyqg0tghzkumzugwutqyy0ew8xalfaa2ws3rxrh9",
            "identifier": "internalVMErrors",
            "topics": [
              "AAAAAAAAAAAFAC6pcV1s5pH7p8urMADuEEmvCAf56p0=",
              "ZQ=="
            ],
            "data": "CglydW50aW1lLmdvOjg1NiBbZXJyb3Igc2lnbmFsbGVkIGJ5IHNtYXJ0Y29udHJhY3RdIFtlXQoJcnVudGltZS5nbzo4NTYgW2Vycm9yIHNpZ25hbGxlZCBieSBzbWFydGNvbnRyYWN0XSBbZV0KCXJ1bnRpbWUuZ286ODUzIFtzdG9yYWdlIGRlY29kZSBlcnJvciAoa2V5OiBwb29sQ29udHJhY3QAAAAAAAABpik6IGlucHV0IHRvbyBzaG9ydF0=",
            "additionalData": [
              "CglydW50aW1lLmdvOjg1NiBbZXJyb3Igc2lnbmFsbGVkIGJ5IHNtYXJ0Y29udHJhY3RdIFtlXQoJcnVudGltZS5nbzo4NTYgW2Vycm9yIHNpZ25hbGxlZCBieSBzbWFydGNvbnRyYWN0XSBbZV0KCXJ1bnRpbWUuZ286ODUzIFtzdG9yYWdlIGRlY29kZSBlcnJvciAoa2V5OiBwb29sQ29udHJhY3QAAAAAAAABpik6IGlucHV0IHRvbyBzaG9ydF0="
            ]
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "e",
      "initiallyPaidFee": "489990000000000",
      "fee": "489990000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
    "#;

    let mut tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(tx_str).unwrap();
    let expected_tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(expected_tx_str).unwrap();
    replace_with_error_message(&mut tx, "c3RvcmFnZSBkZWNvZGUgZXJyb3IgKGtleTogcG9vbENvbnRyYWN0AAAAAAAAAe+/vSk6IGlucHV0IHRvbyBzaG9ydA==");
    assert_eq!(
        expected_tx.logs.unwrap().events[0].topics,
        tx.logs.unwrap().events[0].topics
    );
}

#[test]
fn replace_logs_reason_invalid_test() {
    let tx_str = r#"
{
    "type": "normal",
    "processingTypeOnSource": "SCInvoking",
    "processingTypeOnDestination": "SCInvoking",
    "hash": "ff39d7bc821e22abb5ea79b2cc4756e3ae29de30fe840a21f71303e8d3c5dec6",
    "nonce": 1572,
    "round": 5867253,
    "epoch": 2419,
    "value": "0",
    "receiver": "erd1qqqqqqqqqqqqqpgqhdqz9j3zgpl8fg2z0jzx9n605gwxx4djd8ssruw094",
    "sender": "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx",
    "gasPrice": 1000000000,
    "gasLimit": 5000000,
    "gasUsed": 5000000,
    "data": "ZHVtbXlAMDU=",
    "signature": "cf6e7d03a97da0ba170500411ea3fded5abef78e59c16fcc603fcb8c08bcbc1dc031dc54d4cb9cb518c46c1f11d68e4af248978f0d3a8b6a3b6be9febe319102",
    "sourceShard": 0,
    "destinationShard": 1,
    "blockNonce": 5797898,
    "blockHash": "7713514439a8a85b9ba37ffde4ffbfb186ff1c066939d426fbbbc2f2793c5665",
    "notarizedAtSourceInMetaNonce": 5801206,
    "NotarizedAtSourceInMetaHash": "0763b5c89744a947c7d439103ea28b17bf1428d8e510e4436e284c8823fe45d1",
    "notarizedAtDestinationInMetaNonce": 5801210,
    "notarizedAtDestinationInMetaHash": "67e31450cab3357d0a61008eccea12cd436eec9e1ac0b316f692059a7d51939b",
    "miniblockType": "TxBlock",
    "miniblockHash": "460a612689c07878445a2aeb8ae77364a7d597f05f5ad65bea108f6cc897d11e",
    "hyperblockNonce": 5801210,
    "hyperblockHash": "67e31450cab3357d0a61008eccea12cd436eec9e1ac0b316f692059a7d51939b",
    "timestamp": 1729203518,
    "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqhdqz9j3zgpl8fg2z0jzx9n605gwxx4djd8ssruw094",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqhdqz9j3zgpl8fg2z0jzx9n605gwxx4djd8ssruw094",
            "identifier": "signalError",
            "topics": [
              "gEnWOeWmmA0c0jkqvM5BApzadKFWNSOiAvCWQcwmGPg=",
              "aW52YWxpZCBmdW5jdGlvbiAobm90IGZvdW5kKQ=="
            ],
            "data": "QDY2NzU2ZTYzNzQ2OTZmNmUyMDZlNmY3NDIwNjY2Zjc1NmU2NA==",
            "additionalData": [
              "QDY2NzU2ZTYzNzQ2OTZmNmUyMDZlNmY3NDIwNjY2Zjc1NmU2NA=="
            ]
          },
          {
            "address": "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx",
            "identifier": "internalVMErrors",
            "topics": [
              "AAAAAAAAAAAFALtAIsoiQH50oUJ8hGLPT6IcY1WyaeE=",
              "ZHVtbXk="
            ],
            "data": "CglydW50aW1lLmdvOjg1MyBbaW52YWxpZCBmdW5jdGlvbiAobm90IGZvdW5kKV0gW2R1bW15XQ==",
            "additionalData": [
              "CglydW50aW1lLmdvOjg1MyBbaW52YWxpZCBmdW5jdGlvbiAobm90IGZvdW5kKV0gW2R1bW15XQ=="
            ]
          }
        ]
    },
    "status": "success",
    "operation": "transfer",
    "function": "dummy",
    "initiallyPaidFee": "111380000000000",
    "fee": "111380000000000",
    "chainID": "D",
    "version": 2,
    "options": 0
}
"#;

    let expected_tx_str = r#"
{
    "type": "normal",
    "processingTypeOnSource": "SCInvoking",
    "processingTypeOnDestination": "SCInvoking",
    "hash": "ff39d7bc821e22abb5ea79b2cc4756e3ae29de30fe840a21f71303e8d3c5dec6",
    "nonce": 1572,
    "round": 5867253,
    "epoch": 2419,
    "value": "0",
    "receiver": "erd1qqqqqqqqqqqqqpgqhdqz9j3zgpl8fg2z0jzx9n605gwxx4djd8ssruw094",
    "sender": "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx",
    "gasPrice": 1000000000,
    "gasLimit": 5000000,
    "gasUsed": 5000000,
    "data": "ZHVtbXlAMDU=",
    "signature": "cf6e7d03a97da0ba170500411ea3fded5abef78e59c16fcc603fcb8c08bcbc1dc031dc54d4cb9cb518c46c1f11d68e4af248978f0d3a8b6a3b6be9febe319102",
    "sourceShard": 0,
    "destinationShard": 1,
    "blockNonce": 5797898,
    "blockHash": "7713514439a8a85b9ba37ffde4ffbfb186ff1c066939d426fbbbc2f2793c5665",
    "notarizedAtSourceInMetaNonce": 5801206,
    "NotarizedAtSourceInMetaHash": "0763b5c89744a947c7d439103ea28b17bf1428d8e510e4436e284c8823fe45d1",
    "notarizedAtDestinationInMetaNonce": 5801210,
    "notarizedAtDestinationInMetaHash": "67e31450cab3357d0a61008eccea12cd436eec9e1ac0b316f692059a7d51939b",
    "miniblockType": "TxBlock",
    "miniblockHash": "460a612689c07878445a2aeb8ae77364a7d597f05f5ad65bea108f6cc897d11e",
    "hyperblockNonce": 5801210,
    "hyperblockHash": "67e31450cab3357d0a61008eccea12cd436eec9e1ac0b316f692059a7d51939b",
    "timestamp": 1729203518,
    "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqhdqz9j3zgpl8fg2z0jzx9n605gwxx4djd8ssruw094",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqhdqz9j3zgpl8fg2z0jzx9n605gwxx4djd8ssruw094",
            "identifier": "signalError",
            "topics": [
              "gEnWOeWmmA0c0jkqvM5BApzadKFWNSOiAvCWQcwmGPg=",
              "aW52YWxpZCBmdW5jdGlvbiAobm90IGZvdW5kKQ=="
            ],
            "data": "QDY2NzU2ZTYzNzQ2OTZmNmUyMDZlNmY3NDIwNjY2Zjc1NmU2NA==",
            "additionalData": [
              "QDY2NzU2ZTYzNzQ2OTZmNmUyMDZlNmY3NDIwNjY2Zjc1NmU2NA=="
            ]
          },
          {
            "address": "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx",
            "identifier": "internalVMErrors",
            "topics": [
              "AAAAAAAAAAAFALtAIsoiQH50oUJ8hGLPT6IcY1WyaeE=",
              "ZHVtbXk="
            ],
            "data": "CglydW50aW1lLmdvOjg1MyBbaW52YWxpZCBmdW5jdGlvbiAobm90IGZvdW5kKV0gW2R1bW15XQ==",
            "additionalData": [
              "CglydW50aW1lLmdvOjg1MyBbaW52YWxpZCBmdW5jdGlvbiAobm90IGZvdW5kKV0gW2R1bW15XQ=="
            ]
          }
        ]
    },
    "status": "success",
    "operation": "transfer",
    "function": "dummy",
    "initiallyPaidFee": "111380000000000",
    "fee": "111380000000000",
    "chainID": "D",
    "version": 2,
    "options": 0
}
"#;

    let mut tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(tx_str).unwrap();
    let expected_tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(expected_tx_str).unwrap();
    replace_with_error_message(&mut tx, "aW52YWxpZCBmdW5jdGlvbiAobm90IGZvdW5kKQ==");
    assert_eq!(
        expected_tx.logs.unwrap().events[0].topics,
        tx.logs.unwrap().events[0].topics
    );
}
