use multiversx_sc_snippets::network_response;
use multiversx_sdk_wbg::data::transaction::{TransactionInfo, TransactionOnNetwork};

#[test]
fn test_process_issued_token_identifier_fungible() {
    let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
      "nonce": 61,
      "round": 173598,
      "epoch": 72,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
      "sender": "erd1x39tc3q3nn72ecjnmcz7x0qp09kp97t080x99dgyhx7zh95j0n4szskhlv",
      "gasPrice": 1000000000,
      "gasLimit": 100000000,
      "gasUsed": 100000000,
      "data": "aXNzdWVMcFRva2VuQDAwMDAwMDAwMDAwMDAwMDAwNTAwMTM5ZWQ3YWU0YWEwMzc5MmU2YmNiMzMyMzk0YTQwZmU3NDZlZWZhNDdjZWJANDU0NzRjNDQ0ZDQ1NTg0YzUwQDQ1NDc0YzQ0NGQ0NTU4",
      "signature": "b5049d2906adc1305a6a8d0f42749254ca6259c6996d9a35e7dc7528b3c87b48a421879aff70bc6d81483a7559b75e5dcf9be499dcb7d57aa9f25c79ac2ad40d",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 173354,
      "blockHash": "09d85ac264a54e12e7613395211c53fe0ee5a7d3b7111bf5fec1d02794caaacd",
      "notarizedAtSourceInMetaNonce": 173321,
      "NotarizedAtSourceInMetaHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
      "notarizedAtDestinationInMetaNonce": 173321,
      "notarizedAtDestinationInMetaHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
      "miniblockType": "TxBlock",
      "miniblockHash": "7f45eee4e35ffc1fbce66b92e4dd2aeae2acb092416aa5aa775b96493256b81d",
      "hyperblockNonce": 173321,
      "hyperblockHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
      "timestamp": 1695041588,
      "smartContractResults": [
        {
          "hash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "data": "issue@45474c444d45584c50@45474c444d4558@03e8@12@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e4d696e74@74727565@63616e4275726e@74727565@63616e4368616e67654f776e6572@74727565@63616e55706772616465@74727565@63616e4164645370656369616c526f6c6573@74727565@65ba30",
          "prevTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 89624222,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1x39tc3q3nn72ecjnmcz7x0qp09kp97t080x99dgyhx7zh95j0n4szskhlv",
          "operation": "transfer",
          "function": "issue"
        },
        {
          "hash": "2a452ff652791d79be5f6933fb583cc5503e876893e54b3b51381a92aa2e904d",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetBurnRoleForAll@45474c444d45582d393563366435",
          "prevTxHash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "logs": {
            "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
            "events": [
              {
                "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                "identifier": "completedTxEvent",
                "topics": [
                  "vOPQ3OsLPlyMV4DX2jdVw/dJLVUWhdSTpzv2br02dUs="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTTransfer@45474c444d45582d393563366435@03e8@00",
          "prevTxHash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 39624222,
          "gasPrice": 1000000000,
          "callType": 2,
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                "identifier": "ESDTTransfer",
                "topics": [
                  "RUdMRE1FWC05NWM2ZDU=",
                  "",
                  "A+g=",
                  "AAAAAAAAAAAFAO+ux8+3RD51ieGHV10Z68X293CYfOs="
                ],
                "data": null,
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                "identifier": "completedTxEvent",
                "topics": [
                  "vOPQ3OsLPlyMV4DX2jdVw/dJLVUWhdSTpzv2br02dUs="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "tokens": [
            "EGLDMEX-95c6d5"
          ],
          "esdtValues": [
            "1000"
          ],
          "operation": "ESDTTransfer",
          "function": "\u0000"
        },
        {
          "hash": "c9dfc4de3c3cee319123087a4f5dd03cc051e728ec6070707a63ea977b535227",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "data": "\u0000",
          "prevTxHash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 39424222,
          "gasPrice": 1000000000,
          "callType": 2,
          "operation": "transfer",
          "function": "\u0000"
        },
        {
          "hash": "609c3a8e1903680fef1f6d9e47527b1b5c1259664b868af600162120ce0b8192",
          "nonce": 1,
          "value": 300925400000000,
          "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "data": "@6f6b",
          "prevTxHash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
            "identifier": "transferValueOnly",
            "topics": [
              "AAAAAAAAAAAFAO+ux8+3RD51ieGHV10Z68X293CYfOs=",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
              "saK8LsUAAA=="
            ],
            "data": null,
            "additionalData": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
            "identifier": "writeLog",
            "topics": [
              "NEq8RBGc/KziU94F4zwBeWwS+W87zFK1BLm8K5aSfOs="
            ],
            "data": "QDZmNmI=",
            "additionalData": null
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "issueLpToken",
      "initiallyPaidFee": "1214335000000000",
      "fee": "1214335000000000",
      "chainID": "D",
      "version": 2,
      "options": 0
    }
  },
  "error": "",
  "code": "successful"
}
      "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected: Option<String> = Some("EGLDMEX-95c6d5".to_string());

    assert_eq!(tx_response.new_issued_token_identifier, expected)
}

#[test]
fn test_process_issued_token_identifier_semi_fungible() {
    let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
      "nonce": 65,
      "round": 8422527,
      "epoch": 584,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
      "sender": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
      "gasPrice": 1000000000,
      "gasLimit": 80000000,
      "gasUsed": 80000000,
      "data": "aXNzdWVUb2tlbkA0NDZmNzA2NTU0NjU3Mzc0QDQ0NGY1MDQ1NTQ0NTUzNTQ=",
      "signature": "0191848976e930996f6c62d4921e732f9b0ada8b41ca3b5b63d6bfd304fd44c2a1e8e6643479618ba4a764a36e87f53882b4f707600d5b7d476f2fdd2bac040e",
      "sourceShard": 0,
      "destinationShard": 0,
      "blockNonce": 8420241,
      "blockHash": "4d302220f6015876c95e7961b770cc67f8ab63c5f0ab69b4d6c2fb15c8bc23bd",
      "notarizedAtSourceInMetaNonce": 8403647,
      "NotarizedAtSourceInMetaHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
      "notarizedAtDestinationInMetaNonce": 8403647,
      "notarizedAtDestinationInMetaHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
      "miniblockType": "TxBlock",
      "miniblockHash": "b7b8fc9f3b81d7daae1113cbf73457e16ee31f3a864ef3729a1a21f3a929e112",
      "hyperblockNonce": 8403647,
      "hyperblockHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
      "timestamp": 1646652762,
      "smartContractResults": [
        {
          "hash": "9aecf3bd5dd5c706a28d1cc7059ac20db74340f136816f667dbefcc58daa3aba",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "data": "issueSemiFungible@446f706554657374@444f504554455354@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e4368616e67654f776e6572@74727565@63616e55706772616465@74727565@63616e4164645370656369616c526f6c6573@74727565@5ca148",
          "prevTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 75958360,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
          "operation": "transfer",
          "function": "issueSemiFungible"
        },
        {
          "hash": "aacfe9088bb9d2d5b3fbe9cab2b2f1c6a7e9cbab2f1a41020e2c819fc9b43570",
          "nonce": 66,
          "value": 0,
          "receiver": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
          "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "data": "@6f6b",
          "prevTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer"
        },
        {
          "hash": "3f6f0f3de9e942884e7e1592823a7db7ce935a3f9d3359d8c1ee98a5645332d8",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@444f5045544553542d373732303063",
          "prevTxHash": "9aecf3bd5dd5c706a28d1cc7059ac20db74340f136816f667dbefcc58daa3aba",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 25958360,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                "identifier": "completedTxEvent",
                "topics": [
                  "muzzvV3VxwaijRzHBZrCDbdDQPE2gW9mfb78xY2qOro="
                ],
                "data": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "c6e4f7c5da455009fb4f6967ce8a273a97b826aa617fa798ffd0cf17bde6b97a",
          "nonce": 1,
          "value": 225516180000000,
          "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "data": "@6f6b",
          "prevTxHash": "3f6f0f3de9e942884e7e1592823a7db7ce935a3f9d3359d8c1ee98a5645332d8",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
            "identifier": "transferValueOnly",
            "topics": [
              "AAAAAAAAAAAFAH6d74PDz8xLqvowrlOA5lVDBMUghBg=",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
              "saK8LsUAAA=="
            ],
            "data": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
            "identifier": "writeLog",
            "topics": [
              "NFD3vy7whM1YgLyLL6nEojWMeUDt2MMJRRM9qTgahBg="
            ],
            "data": "QDZmNmI="
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "issueToken",
      "initiallyPaidFee": "914840000000000",
      "fee": "914840000000000",
      "chainID": "1",
      "version": 1,
      "options": 0
    }
  },
  "error": "",
  "code": "successful"
}
        "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected: Option<String> = Some("DOPETEST-77200c".to_string());

    assert_eq!(tx_response.new_issued_token_identifier, expected)
}

#[test]
fn test_process_issued_token_identifier_non_fungible() {
    let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
      "nonce": 16,
      "round": 820170,
      "epoch": 341,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
      "sender": "erd162knt53z7m0f9jjms9wxphr3q9d7zu4ky85xs2cc0ekrqz7k4fdq85lkuc",
      "gasPrice": 1000000000,
      "gasLimit": 200000000,
      "gasUsed": 200000000,
      "data": "aXNzdWVUb2tlbkA2NzY1NmU2NTdhNzk3M0A0NzQ1NGU=",
      "signature": "e80d45f4de419799a2bbff1cae1235521c8eef1853ee45b02f95c2da74ce50d241bf75b6ab0c650245562700862ea9759caad40f3e381ac0c4d82cfe56e67c09",
      "sourceShard": 2,
      "destinationShard": 2,
      "blockNonce": 819313,
      "blockHash": "a1db4ef13f07b86678000df9cc78f244d83dcc35ae51de545f333bf616930d39",
      "notarizedAtSourceInMetaNonce": 819396,
      "NotarizedAtSourceInMetaHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
      "notarizedAtDestinationInMetaNonce": 819396,
      "notarizedAtDestinationInMetaHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
      "miniblockType": "TxBlock",
      "miniblockHash": "afdb278522181aeb9b12f08840e6c534e398e6af9c7f757548308e300e7ec4e9",
      "hyperblockNonce": 819396,
      "hyperblockHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
      "timestamp": 1698921020,
      "smartContractResults": [
        {
          "hash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "data": "issueNonFungible@67656e657a7973@47454e@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e5472616e736665724e4654437265617465526f6c65@74727565@63616e4368616e67654f776e6572@66616c7365@63616e55706772616465@66616c7365@63616e4164645370656369616c526f6c6573@74727565@5e30e4",
          "prevTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 196098365,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd162knt53z7m0f9jjms9wxphr3q9d7zu4ky85xs2cc0ekrqz7k4fdq85lkuc",
          "operation": "transfer",
          "function": "issueNonFungible"
        },
        {
          "hash": "98afe82512c79f1caaf171bd5919ee469d11ba0c4f725aefcab834278c0f1e58",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetBurnRoleForAll@47454e2d383638353933",
          "prevTxHash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "logs": {
            "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
            "events": [
              {
                "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                "identifier": "completedTxEvent",
                "topics": [
                  "b+DMACgCrxdE85Tu5KaSJLXndZYdg4bgTnpbkkL3/2U="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "data": "setSpecialRole@47454e2d383638353933@00000000000000000500de51fa8943c26e6933419f9bb7ceb79b7ff4f7bbaa5a@45534454526f6c654e4654437265617465@5e30e4",
          "prevTxHash": "112d18ec0364b4700b1bfb3df517c80dba547a53373ece2a9e71acd5266e7b64",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 142399698,
          "gasPrice": 1000000000,
          "callType": 1,
          "operation": "transfer",
          "function": "setSpecialRole"
        },
        {
          "hash": "112d18ec0364b4700b1bfb3df517c80dba547a53373ece2a9e71acd5266e7b64",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@47454e2d383638353933",
          "prevTxHash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 146098365,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "writeLog",
                "topics": [
                  "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo="
                ],
                "data": "QDZmNmI=",
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "db5d74970374337956fa61fb4fd90057b3f6a82ea3e259b389934b71a1652e5f",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetRole@47454e2d383638353933@45534454526f6c654e4654437265617465",
          "prevTxHash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "ESDTSetRole",
                "topics": [
                  "R0VOLTg2ODU5Mw==",
                  "",
                  "",
                  "RVNEVFJvbGVORlRDcmVhdGU="
                ],
                "data": null,
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "completedTxEvent",
                "topics": [
                  "g0lK2TaXOLV0pyZsv7Es5jzPY0lQzWsOwWEHuPtC+PY="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "ESDTSetRole",
          "function": "ESDTSetRole"
        },
        {
          "hash": "a6a665f47977a59c4c2baf460281fc938e04ae0f87ac2e78040a14ae27822701",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00",
          "prevTxHash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 92399698,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "writeLog",
                "topics": [
                  "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo=",
                  "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gOTIzOTk2OTgsIGdhcyB1c2VkID0gMzE0MTg4MA=="
                ],
                "data": "QDZmNmI=",
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "completedTxEvent",
                "topics": [
                  "g0lK2TaXOLV0pyZsv7Es5jzPY0lQzWsOwWEHuPtC+PY="
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
        "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "identifier": "transferValueOnly",
            "topics": [
              "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo=",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
              "saK8LsUAAA=="
            ],
            "data": null,
            "additionalData": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "identifier": "writeLog",
            "topics": [
              "0q010iL23pLKW4FcYNxxAVvhcrYh6GgrGH5sMAvWqlo="
            ],
            "data": "QDZmNmI=",
            "additionalData": null
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "issueToken",
      "initiallyPaidFee": "2097020000000000",
      "fee": "2097020000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
  },
  "error": "",
  "code": "successful"
}
        "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected: Option<String> = Some("GEN-868593".to_string());

    assert_eq!(tx_response.new_issued_token_identifier, expected)
}

#[test]
fn test_process_issued_token_identifier_meta_esdt() {
    let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
      "nonce": 419,
      "round": 1787093,
      "epoch": 744,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
      "sender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
      "gasPrice": 1000000000,
      "gasLimit": 600000000,
      "gasUsed": 157220928,
      "data": "ZGVwbG95QXNoc3dhcExQQUNTdHJhdGVneUA0MTRjNTAyZDYzNjE2NTYxNjMzNUA0MTU0NTMyZDM0NjMzMDM5MzIzMEAwM2U4QDAzZThAQDNiOWFjYTAwQDAwMDAwMDAwMDAwMDAwMDAwNTAwOTU3MzkwYWVkYTQzMmY1MmE0MTFkNTE5NzRmZTkzZDQwZDI3NzMzZTA0NjNAMDAwMDAwMDAwMDAwMDAwMDA1MDBkMTJjYzczY2JkYTZmMjY1OWM5NTllNWQ1NzU4YWY5MmNhMTM4NDg2NTIzM0AwMDAwMDAwMDAwMDAwMDAwMDUwMDUxZGY3MTc1OGNmMmFjYTViNDZkZWQ4MTU1OGI1NTE1ZGMyOWYzZjM1MjMzQEAwMDAwMDAwMDAwMDAwMDAwMDUwMDdlNGExZGZjNDM3Y2VkNDlkYjlmMTYzNzk4NDE2Yjg0YWMyMWQ0Yzk3Y2ViMDAwMDAwMGM1NzQ1NDc0YzQ0MmQ2MTMyMzg2MzM1MzkwMDAwMDAwMDAwMDAwMDAwMDUwMGE4YmE5ZTY4NjI2YmJjOTkzZmQ3OTVlOGJiNmY0Nzk0M2IyZjVmZmE3Y2ViMDAwMDAwMGE1NTU0NGIyZDMxMzQ2NDM1Mzc2NEAwMDAwMDAwMTAwMDAwMDAwMDAwMDAwMDAwNTAwNTFkZjcxNzU4Y2YyYWNhNWI0NmRlZDgxNTU4YjU1MTVkYzI5ZjNmMzUyMzMwMDAwMDAwYjQyNTU1MzQ0MmQ2NDM0NjMzMDMxMzQwMDAwMDAwMDAwQDAxODZhMEAyNzEw",
      "signature": "4648af0b96eb430e4986b9fb760549742de09c809b46b984e5d995c898d80c25bfc0717c30da34bd89cd3005d98ee895afa39ee588b7b74b4807c63cbeade807",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 1785520,
      "blockHash": "8f926a5d79fa84bc69949a21bfbba17447091a8a074ac172fa0b88e4475a1214",
      "notarizedAtSourceInMetaNonce": 1785568,
      "NotarizedAtSourceInMetaHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
      "notarizedAtDestinationInMetaNonce": 1785568,
      "notarizedAtDestinationInMetaHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
      "miniblockType": "TxBlock",
      "miniblockHash": "b85d82db6d69cbc1911b3455d2837eeb3170b391926efa2eacb4d9c8e3c96ee4",
      "hyperblockNonce": 1785568,
      "hyperblockHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
      "timestamp": 1704722558,
      "smartContractResults": [
        {
          "hash": "ea9a96c079e66249e6b73c0341991dad96ca81f855f2fc4abe0d432be117a882",
          "nonce": 420,
          "value": 4427790720000000,
          "receiver": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "@6f6b",
          "prevTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        },
        {
          "hash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "registerMetaESDT@415453417368537761704c5041435661756c74@4156415348@12@63616e467265657a65@66616c7365@63616e57697065@66616c7365@63616e5061757365@66616c7365@63616e5472616e736665724e4654437265617465526f6c65@66616c7365@63616e4368616e67654f776e6572@66616c7365@63616e55706772616465@66616c7365@63616e4164645370656369616c526f6c6573@74727565@9eb30a87c92674ab1469700c0b385b3850e86de80f87dec6cf3213c7e379a646@408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43@03eb4a30",
          "prevTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 125751600,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "operation": "transfer",
          "function": "registerMetaESDT"
        },
        {
          "hash": "290f85d7ec2f7d5797510290358e9e0f76bb880451efaacb0d69280b8d94c67a",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetBurnRoleForAll@41564153482d376438623564",
          "prevTxHash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
            "events": [
              {
                "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                "identifier": "completedTxEvent",
                "topics": [
                  "YIKXUTKiydgZff0PmFK0VK00R0Duvb35P2ILJ5arcjs="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "1aa62a6251edd216bd4e5ae59f7e676d5d2f88597685e0ec0e25ac4434bfccdb",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@41564153482d376438623564@d0644194444642fd16ee156307f6fda0e8f8baf4c496e1a1dc85e027ecc08a4a@9eb30a87c92674ab1469700c0b385b3850e86de80f87dec6cf3213c7e379a646@408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43@00",
          "prevTxHash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 75751600,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "writeLog",
                "topics": [
                  "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
                  "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNzU3NTE2MDAsIGdhcyB1c2VkID0gNDE3NjA1OQ=="
                ],
                "data": "QDZmNmI=",
                "additionalData": [
                  "QDZmNmI="
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "completedTxEvent",
                "topics": [
                  "YIKXUTKiydgZff0PmFK0VK00R0Duvb35P2ILJ5arcjs="
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
        "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM="
            ],
            "data": "RGVwbG95RnJvbVNvdXJjZQ==",
            "additionalData": [
              "RGVwbG95RnJvbVNvdXJjZQ==",
              "aW5pdA==",
              "QUxQLWNhZWFjNQ==",
              "QVRTLTRjMDkyMA==",
              "A+g=",
              "A+g=",
              "",
              "O5rKAA=="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM="
            ],
            "data": "RGVwbG95RnJvbVNvdXJjZQ==",
            "additionalData": [
              "RGVwbG95RnJvbVNvdXJjZQ==",
              "aW5pdA==",
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
              "AAAAAAAAAAAFAJVzkK7aQy9SpBHVGXT+k9QNJ3M+BGM=",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM=",
              "AAAAAAAAAAAFAFHfcXWM8qyltG3tgVWLVRXcKfPzUjM=",
              "",
              "AAAAAAAAAAAFAH5KHfxDfO1J258WN5hBa4SsIdTJfOsAAAAMV0VHTEQtYTI4YzU5AAAAAAAAAAAFAKi6nmhia7yZP9eV6LtvR5Q7L1/6fOsAAAAKVVRLLTE0ZDU3ZA==",
              "AAAAAQAAAAAAAAAABQBR33F1jPKspbRt7YFVi1UV3Cnz81IzAAAAC0JVU0QtZDRjMDE0AAAAAAA=",
              "AYag",
              "JxA="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
            ],
            "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
            "additionalData": [
              "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
              "Z2V0RmFybWluZ1Rva2VuSWQ="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
            ],
            "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
            "additionalData": [
              "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
              "Z2V0RmFybVRva2VuSWQ="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
            ],
            "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
            "additionalData": [
              "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
              "Z2V0UmV3YXJkVG9rZW5JZA=="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "saK8LsUAAA==",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
            ],
            "data": "QXN5bmNDYWxs",
            "additionalData": [
              "QXN5bmNDYWxs",
              "cmVnaXN0ZXJNZXRhRVNEVA==",
              "QVRTQXNoU3dhcExQQUNWYXVsdA==",
              "QVZBU0g=",
              "Eg==",
              "Y2FuRnJlZXpl",
              "ZmFsc2U=",
              "Y2FuV2lwZQ==",
              "ZmFsc2U=",
              "Y2FuUGF1c2U=",
              "ZmFsc2U=",
              "Y2FuVHJhbnNmZXJORlRDcmVhdGVSb2xl",
              "ZmFsc2U=",
              "Y2FuQ2hhbmdlT3duZXI=",
              "ZmFsc2U=",
              "Y2FuVXBncmFkZQ==",
              "ZmFsc2U=",
              "Y2FuQWRkU3BlY2lhbFJvbGVz",
              "dHJ1ZQ=="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "SCDeploy",
            "topics": [
              "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM=",
              "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
              "fvRqbue54Womde/CN2IkRGkrx8tsU+xkLvi3+uwMkhY="
            ],
            "data": null,
            "additionalData": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
            "identifier": "SCDeploy",
            "topics": [
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
              "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
              "E3blQfRJfCKLWDr06Od703DSZenIzq8KND+xUjmGY/M="
            ],
            "data": null,
            "additionalData": null
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "deployAshswapLPACStrategy",
      "initiallyPaidFee": "6936045000000000",
      "fee": "2508254280000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
  },
  "error": "",
  "code": "successful"
}
        "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected: Option<String> = Some("AVASH-7d8b5d".to_string());

    assert_eq!(tx_response.new_issued_token_identifier, expected)
}

#[test]
fn test_set_special_roles_should_not_process_issued_token_identifier() {
    let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
      "nonce": 420,
      "round": 1787109,
      "epoch": 744,
      "value": "0",
      "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
      "sender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
      "gasPrice": 1000000000,
      "gasLimit": 600000000,
      "gasUsed": 129636807,
      "data": "ZmluaXNoVmF1bHREZXBsb3ltZW50cw==",
      "signature": "dca943ef1a788bfa6cb0e9aa3900b8340e4908075cbfefaa2a66688f6f0c0fed349edb2eb48eec427cd9098822fba875e4d66072fbdb44cb7f4c1a416736e20c",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 1785536,
      "blockHash": "93ca539e81612768b67a85b7135f7c104e76bec031a758a6b1782910ae49dd8f",
      "notarizedAtSourceInMetaNonce": 1785584,
      "NotarizedAtSourceInMetaHash": "71d17afe660282bb42de1ea3eec3e3534a179bd32aa1471c2861ce411bf30552",
      "notarizedAtDestinationInMetaNonce": 1785584,
      "notarizedAtDestinationInMetaHash": "71d17afe660282bb42de1ea3eec3e3534a179bd32aa1471c2861ce411bf30552",
      "miniblockType": "TxBlock",
      "miniblockHash": "f8c60565af746e92d2c9c09a92734e5eb8da7e42c67a86854c93b349bfe287eb",
      "hyperblockNonce": 1785584,
      "hyperblockHash": "71d17afe660282bb42de1ea3eec3e3534a179bd32aa1471c2861ce411bf30552",
      "timestamp": 1704722654,
      "smartContractResults": [
        {
          "hash": "c3ce9c364de3823ffae250c2bfb40aaf2b18f771ed4bd37bf788ad83a2c651f3",
          "nonce": 421,
          "value": 4703631930000000,
          "receiver": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "@6f6b",
          "prevTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        },
        {
          "hash": "50f9c25a1402ce6d87ae9f890659c8a67462292e471e02c74d64ff7ba1995e60",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "setSpecialRole@41564153482d376438623564@00000000000000000500d00cc0e63887ff6b792d34234a44e7ac6b575d4b0463@45534454526f6c654e4654437265617465@45534454526f6c654e46544164645175616e74697479@45534454526f6c654e46544275726e@0192c6db2c69f50b6968fb22ac558337a851719519cfd1e6bbf79a07bbcf18bc@cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0@03eb4a30",
          "prevTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 125751600,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "operation": "transfer",
          "function": "setSpecialRole"
        },
        {
          "hash": "d6a5824a60b6c9050462c3f5a02ace00c36e8b4ba1958d132bd394e2ed1e7226",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetRole@41564153482d376438623564@45534454526f6c654e4654437265617465@45534454526f6c654e46544164645175616e74697479@45534454526f6c654e46544275726e",
          "prevTxHash": "50f9c25a1402ce6d87ae9f890659c8a67462292e471e02c74d64ff7ba1995e60",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
                "identifier": "ESDTSetRole",
                "topics": [
                  "QVZBU0gtN2Q4YjVk",
                  "",
                  "",
                  "RVNEVFJvbGVORlRDcmVhdGU=",
                  "RVNEVFJvbGVORlRBZGRRdWFudGl0eQ==",
                  "RVNEVFJvbGVORlRCdXJu"
                ],
                "data": null,
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
                "identifier": "completedTxEvent",
                "topics": [
                  "UPnCWhQCzm2Hrp+JBlnIpnRiKS5HHgLHTWT/e6GZXmA="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "ESDTSetRole",
          "function": "ESDTSetRole"
        },
        {
          "hash": "bf1b8b4b301ff548368dfd972896489d5e2a088d5cbdfa1bfe2421cc7f641f7a",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@a68d44c751eba85db0713db8dc9c10c78749189ec0d6f1af5fc67bb656c1254b@0192c6db2c69f50b6968fb22ac558337a851719519cfd1e6bbf79a07bbcf18bc@cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0@00",
          "prevTxHash": "50f9c25a1402ce6d87ae9f890659c8a67462292e471e02c74d64ff7ba1995e60",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 75751600,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "transferValueOnly",
                "topics": [
                  "",
                  "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM="
                ],
                "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                "additionalData": [
                  "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                  "c2V0U2hhcmVUb2tlbklkZW50aWZpZXI=",
                  "QVZBU0gtN2Q4YjVk"
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "transferValueOnly",
                "topics": [
                  "",
                  "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM="
                ],
                "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                "additionalData": [
                  "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                  "c2V0U3RyYXRlZ3lBZGRyZXNz",
                  "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM="
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "completedTxEvent",
                "topics": [
                  "UPnCWhQCzm2Hrp+JBlnIpnRiKS5HHgLHTWT/e6GZXmA="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "9d75a398545f488d4764149245e6ec3101debfce99477c353ac11c3239acd897",
          "nonce": 1,
          "value": 648519550000000,
          "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "@6f6b",
          "prevTxHash": "bf1b8b4b301ff548368dfd972896489d5e2a088d5cbdfa1bfe2421cc7f641f7a",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
            ],
            "data": "QXN5bmNDYWxs",
            "additionalData": [
              "QXN5bmNDYWxs",
              "c2V0U3BlY2lhbFJvbGU=",
              "QVZBU0gtN2Q4YjVk",
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
              "RVNEVFJvbGVORlRDcmVhdGU=",
              "RVNEVFJvbGVORlRBZGRRdWFudGl0eQ==",
              "RVNEVFJvbGVORlRCdXJu"
            ]
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "finishVaultDeployments",
      "initiallyPaidFee": "6082170000000000",
      "fee": "1378538070000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
  },
  "error": "",
  "code": "successful"
}
        "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected: Option<String> = None;

    assert_eq!(tx_response.new_issued_token_identifier, expected)
}

#[test]
fn test_multisig_issue_nft_and_set_all_roles() {
    let data = r#"
{
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd",
      "nonce": 53,
      "round": 3050972,
      "epoch": 1246,
      "value": "0",
      "receiver": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
      "sender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
      "gasPrice": 1000000000,
      "gasLimit": 80000000,
      "gasUsed": 80000000,
      "data": "cGVyZm9ybUFjdGlvbkAwMQ==",
      "signature": "cb67645595cee5f7967d8d85af05bb7db73e80d9b97611796819249d87cd174b69b4abfc2a3fbe52df1aec965bdea921f7eb34d2b1118aa480699ad1dc85790a",
      "sourceShard": 0,
      "destinationShard": 0,
      "blockNonce": 2984930,
      "blockHash": "644ae8703b826a23e89429953919ec37f875e34a547ea9f7edd53fb71a99c746",
      "notarizedAtSourceInMetaNonce": 2988311,
      "NotarizedAtSourceInMetaHash": "4f608a72e654dd9f466801cd489be8ee1a73fbcd77b128559cd46182d3b9455a",
      "notarizedAtDestinationInMetaNonce": 2988311,
      "notarizedAtDestinationInMetaHash": "4f608a72e654dd9f466801cd489be8ee1a73fbcd77b128559cd46182d3b9455a",
      "miniblockType": "TxBlock",
      "miniblockHash": "c5a73671bc1d37835ddd15b926157721bc83203ec4e00cd48ae0d46015cb5f0b",
      "hyperblockNonce": 2988311,
      "hyperblockHash": "4f608a72e654dd9f466801cd489be8ee1a73fbcd77b128559cd46182d3b9455a",
      "timestamp": 1712305832,
      "smartContractResults": [
        {
          "hash": "b0b3c8df519c33b314c0ee3d25abae6f17c4432fb3382676ce17a42690811cff",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
          "data": "registerAndSetAllRoles@54657374436f6c6c656374696f6e31@54455354434f4c4c31@4e4654@@98fa4ff554b9c6990ce577fbb816a271f690dcbd6b148f6583fe7692868ae538@08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd@5e2338",
          "prevTxHash": "08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd",
          "originalTxHash": "08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd",
          "gasLimit": 73052300,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
          "operation": "transfer",
          "function": "registerAndSetAllRoles"
        },
        {
          "hash": "5ae4f74e134e4fa63c8b92e06ff12b2a4b544233d01d80db6a922af35ee55356",
          "nonce": 1,
          "value": 196430610000000,
          "receiver": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
          "sender": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
          "data": "@6f6b",
          "prevTxHash": "c4a24b01b48d32308636310e2d335d6ed1f34dcbdfc1133aed7995e78e831c18",
          "originalTxHash": "08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        },
        {
          "hash": "7589c1ad622d8a9ab2f186731fc82aeeab0aea5a8198cb94b6eba85a966e7962",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqq2m3f0f",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetBurnRoleForAll@54455354434f4c4c312d356161383063",
          "prevTxHash": "b0b3c8df519c33b314c0ee3d25abae6f17c4432fb3382676ce17a42690811cff",
          "originalTxHash": "08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "originalSender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
          "logs": {
            "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqq2m3f0f",
            "events": [
              {
                "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqq2m3f0f",
                "identifier": "completedTxEvent",
                "topics": [
                  "sLPI31GcM7MUwO49JauubxfEQy+zOCZ2zhekJpCBHP8="
                ]
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "86d1ec3365ea1311dbde2f2366de4ea8627d7e49c29a974578c0869b66903cbc",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetRole@54455354434f4c4c312d356161383063@45534454526f6c654e4654437265617465@45534454526f6c654e46544275726e@45534454526f6c654e465455706461746541747472696275746573@45534454526f6c654e4654416464555249",
          "prevTxHash": "b0b3c8df519c33b314c0ee3d25abae6f17c4432fb3382676ce17a42690811cff",
          "originalTxHash": "08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "originalSender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
                "identifier": "ESDTSetRole",
                "topics": [
                  "VEVTVENPTEwxLTVhYTgwYw==",
                  "",
                  "",
                  "RVNEVFJvbGVORlRDcmVhdGU=",
                  "RVNEVFJvbGVORlRCdXJu",
                  "RVNEVFJvbGVORlRVcGRhdGVBdHRyaWJ1dGVz",
                  "RVNEVFJvbGVORlRBZGRVUkk="
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
                "identifier": "completedTxEvent",
                "topics": [
                  "sLPI31GcM7MUwO49JauubxfEQy+zOCZ2zhekJpCBHP8="
                ]
              }
            ]
          },
          "operation": "ESDTSetRole",
          "function": "ESDTSetRole"
        },
        {
          "hash": "c4a24b01b48d32308636310e2d335d6ed1f34dcbdfc1133aed7995e78e831c18",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@54455354434f4c4c312d356161383063@3ec73c55022548038bbe06c0639156b3db70b7c770955e340f14fcfcd45df06a@98fa4ff554b9c6990ce577fbb816a271f690dcbd6b148f6583fe7692868ae538@08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd@00",
          "prevTxHash": "b0b3c8df519c33b314c0ee3d25abae6f17c4432fb3382676ce17a42690811cff",
          "originalTxHash": "08582bc19734ad82d7390be88463c948e5d9f026f4b8f0bfc57620957c3433bd",
          "gasLimit": 23052300,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
                "identifier": "callBack",
                "topics": [
                  "YXN5bmNDYWxsU3VjY2Vzcw==",
                  "VEVTVENPTEwxLTVhYTgwYw=="
                ],
                "additionalData": [
                  ""
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
                "identifier": "completedTxEvent",
                "topics": [
                  "sLPI31GcM7MUwO49JauubxfEQy+zOCZ2zhekJpCBHP8="
                ]
              }
            ]
          },
          "operation": "transfer"
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
            "identifier": "performAction",
            "topics": [
              "c3RhcnRQZXJmb3JtQWN0aW9u"
            ],
            "data": "AAAAAQYAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAL//wAAAAexorwuxQAAAAAAFnJlZ2lzdGVyQW5kU2V0QWxsUm9sZXMAAAAEAAAAD1Rlc3RDb2xsZWN0aW9uMQAAAAlURVNUQ09MTDEAAAADTkZUAAAAAAAAAATjKv7ckE/hk5dGrZc76zg1Y89jZCumabMED5uUKKXtYLE6AXQjw2bK/4zs+3ehJhChMPSIgTQSLHk3/q4NbX0XOvjZyUI7JXfGJSciwdkCEqQRH3ID+XRPdvz6HQoxADOyoRVVzlIeSUTgmrF1SdhbSH3NJshLUBejnjGjZwiJug==",
            "additionalData": [
              "AAAAAQYAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAL//wAAAAexorwuxQAAAAAAFnJlZ2lzdGVyQW5kU2V0QWxsUm9sZXMAAAAEAAAAD1Rlc3RDb2xsZWN0aW9uMQAAAAlURVNUQ09MTDEAAAADTkZUAAAAAAAAAATjKv7ckE/hk5dGrZc76zg1Y89jZCumabMED5uUKKXtYLE6AXQjw2bK/4zs+3ehJhChMPSIgTQSLHk3/q4NbX0XOvjZyUI7JXfGJSciwdkCEqQRH3ID+XRPdvz6HQoxADOyoRVVzlIeSUTgmrF1SdhbSH3NJshLUBejnjGjZwiJug=="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
            "identifier": "performAction",
            "topics": [
              "cGVyZm9ybUFzeW5jQ2FsbA==",
              "AQ==",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
              "saK8LsUAAA==",
              "BGa4HQ==",
              "cmVnaXN0ZXJBbmRTZXRBbGxSb2xlcw==",
              "VGVzdENvbGxlY3Rpb24x",
              "VEVTVENPTEwx",
              "TkZU",
              ""
            ],
            "additionalData": [
              ""
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
            "identifier": "transferValueOnly",
            "topics": [
              "saK8LsUAAA==",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
            ],
            "data": "QXN5bmNDYWxs",
            "additionalData": [
              "QXN5bmNDYWxs",
              "cmVnaXN0ZXJBbmRTZXRBbGxSb2xlcw==",
              "VGVzdENvbGxlY3Rpb24x",
              "VEVTVENPTEwx",
              "TkZU",
              ""
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqrp3n58vp2dmcaur4whazxngvuhac4xwqa4sq2pjl73",
            "identifier": "writeLog",
            "topics": [
              "4yr+3JBP4ZOXRq2XO+s4NWPPY2QrpmmzBA+blCil7WA="
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
      "function": "performAction",
      "initiallyPaidFee": "873260000000000",
      "fee": "873260000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
  },
  "code": "successful"
}
        "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected = Some("TESTCOLL1-5aa80c".to_string());

    assert_eq!(tx_response.new_issued_token_identifier, expected)
}
