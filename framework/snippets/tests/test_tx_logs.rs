use multiversx_sc_scenario::imports::ReturnCode;
use multiversx_sc_snippets::network_response;
use multiversx_sdk::data::transaction::{TransactionInfo, TransactionOnNetwork};

#[test]
fn test_tx_multiple_logs() {
    let data = r#"
    {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "4c554d060e1b489d403759e445c4a4d80b0daa5a8eceafc7b9093eb8a7dd4b7a",
      "nonce": 6768,
      "round": 5269269,
      "epoch": 2169,
      "value": "0",
      "receiver": "erd1qqqqqqqqqqqqqpgqqnw862rla67qnm7qwcxnkaw42kpg2t7ld8sssw0vgu",
      "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
      "gasPrice": 1000000000,
      "gasLimit": 30000000,
      "gasUsed": 30000000,
      "data": "cG9uZw==",
      "signature": "dccb8bf68defef89e938e768fea872d34d6a1ba716813fe78c583233e418b6c800973bb353c61c1babf816bae2200c2ff24197e7c7748c007d72560e2ce16108",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 5200682,
      "blockHash": "b70f33fd98a8cd465cf1a679977d4c0c27f38db6a7634cab57d82e3fd8bd4841",
      "notarizedAtSourceInMetaNonce": 5204138,
      "NotarizedAtSourceInMetaHash": "1b908cfc413beb0e4d89812e2d4430d7cfbf67f7c65098aa476a2cdf2a892ac8",
      "notarizedAtDestinationInMetaNonce": 5204138,
      "notarizedAtDestinationInMetaHash": "1b908cfc413beb0e4d89812e2d4430d7cfbf67f7c65098aa476a2cdf2a892ac8",
      "miniblockType": "TxBlock",
      "miniblockHash": "3217265a72ce8cf40b900a8467b797610fb80eb92158143aac6e9f85e8177945",
      "hyperblockNonce": 5204138,
      "hyperblockHash": "1b908cfc413beb0e4d89812e2d4430d7cfbf67f7c65098aa476a2cdf2a892ac8",
      "timestamp": 1725615614,
      "smartContractResults": [
        {
          "hash": "66debb4f02a735f00bd7da565069f7d26412341d6fae56bbab9b98696c8701e9",
          "nonce": 0,
          "value": 1000000000000000,
          "receiver": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
          "sender": "erd1qqqqqqqqqqqqqpgqqnw862rla67qnm7qwcxnkaw42kpg2t7ld8sssw0vgu",
          "prevTxHash": "4c554d060e1b489d403759e445c4a4d80b0daa5a8eceafc7b9093eb8a7dd4b7a",
          "originalTxHash": "4c554d060e1b489d403759e445c4a4d80b0daa5a8eceafc7b9093eb8a7dd4b7a",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "originalSender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
          "operation": "transfer"
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqqnw862rla67qnm7qwcxnkaw42kpg2t7ld8sssw0vgu",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqqnw862rla67qnm7qwcxnkaw42kpg2t7ld8sssw0vgu",
            "identifier": "transferValueOnly",
            "topics": [
              "A41+pMaAAA==",
              "ATlHLv9ohncamC8wg9pdQh8kwpGB5jiIIo3IHKYNaeE="
            ],
            "data": "RGlyZWN0Q2FsbA==",
            "additionalData": [
              "RGlyZWN0Q2FsbA==",
              ""
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqqnw862rla67qnm7qwcxnkaw42kpg2t7ld8sssw0vgu",
            "identifier": "writeLog",
            "topics": [
              "ATlHLv9ohncamC8wg9pdQh8kwpGB5jiIIo3IHKYNaeE=",
              "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gMjk5NDQwMDAsIGdhcyB1c2VkID0gMjE3MTk0OA=="
            ],
            "data": "QDZmNmI=",
            "additionalData": [
              "QDZmNmI="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqqnw862rla67qnm7qwcxnkaw42kpg2t7ld8sssw0vgu",
            "identifier": "completedTxEvent",
            "topics": [
              "TFVNBg4bSJ1AN1nkRcSk2AsNqlqOzq/HuQk+uKfdS3o="
            ],
            "data": null,
            "additionalData": null
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "pong",
      "initiallyPaidFee": "355440000000000",
      "fee": "355440000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
  },
  "error": "",
  "code": "successful"
}"#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network, ReturnCode::Success);

    assert!(tx_response.logs.len() == 3);
    assert_eq!(tx_response.logs[0].endpoint, "transferValueOnly");
    assert_eq!(tx_response.logs[1].endpoint, "writeLog");
    assert_eq!(tx_response.logs[2].endpoint, "completedTxEvent");
}
