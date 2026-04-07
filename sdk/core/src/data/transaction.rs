mod api_logs;
mod api_smart_contract_result;
mod arg_create_transaction;
mod events;
mod log_data;
mod send_transaction;
mod send_transactions;
mod simulate_gas;
mod transaction;
mod transaction_info;
mod transaction_on_network;
mod transaction_process_status;
mod transaction_status;
mod tx_cost;

pub use api_logs::ApiLogs;
pub use api_smart_contract_result::ApiSmartContractResult;
pub use arg_create_transaction::ArgCreateTransaction;
pub use events::Events;
pub use log_data::LogData;
pub use send_transaction::{SendTransactionData, SendTransactionResponse};
pub use send_transactions::{SendTransactionsResponse, SendTransactionsResponseData};
pub use simulate_gas::{SimulateGasTransactionData, SimulateGasTransactionResponse};
pub use transaction::Transaction;
pub use transaction_info::{TransactionInfo, TransactionInfoData};
pub use transaction_on_network::TransactionOnNetwork;
pub use transaction_process_status::{TransactionProcessStatus, TransactionProcessStatusData};
pub use transaction_status::{TransactionStatus, TransactionStatusData};
pub use tx_cost::{TxCostResponse, TxCostResponseData};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_event_log_null_data() {
        let data = r#"
{
    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
    "identifier": "completedTxEvent",
    "topics": [],
    "data": null,
    "additionalData": null
}
        "#;

        let event_log = serde_json::from_str::<Events>(data).unwrap();
        assert!(event_log.topics.is_empty());
        assert_eq!(event_log.data, LogData::Empty);
    }

    #[test]
    fn parse_event_log_no_topics() {
        let data = r#"
{
    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
    "identifier": "completedTxEvent",
    "data": null,
    "additionalData": null
}
        "#;

        let event_log = serde_json::from_str::<Events>(data).unwrap();
        assert!(event_log.topics.is_empty());
    }

    #[test]
    fn parse_event_log_null_additional_data() {
        let data = r#"
{
    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
    "identifier": "completedTxEvent",
    "topics": [],
    "data": "data-string",
    "additionalData": null
}
        "#;

        let event_log = serde_json::from_str::<Events>(data).unwrap();
        assert_eq!(event_log.data, LogData::String("data-string".to_owned()));
    }

    #[test]
    fn parse_event_log_with_array_data() {
        let data = r#"
{
    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
    "identifier": "completedTxEvent",
    "topics": [],
    "data": [
        "data1",
        "data2"
    ],
    "additionalData": null
}
        "#;

        let event_log = serde_json::from_str::<Events>(data).unwrap();
        assert_eq!(
            event_log.data,
            LogData::Vec(vec!["data1".to_owned(), "data2".to_owned()])
        );
    }

    #[test]
    fn parse_transaction_info_no_signature() {
        let data = r#"
{
    "data": {
        "transaction": {
            "type": "unsigned",
            "processingTypeOnSource": "SCInvoking",
            "processingTypeOnDestination": "SCInvoking",
            "hash": "34cd9c6d0f68c0975971352ed4dcaacc1acd9a2dbd8f5840a2866d09b1d72298",
            "nonce": 0,
            "round": 5616535,
            "epoch": 2314,
            "value": "0",
            "receiver": "erd1qqqqqqqqqqqqqpgq0mhy244pyr9pzdhahvvyze4rw3xl29q4kklszyzq72",
            "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
            "gasPrice": 1000000000,
            "gasLimit": 25411165,
            "gasUsed": 1197500,
            "data": "",
            "previousTransactionHash": "6c105dc2bb6ca8b89cfcac910a46310812b51312a281837096fc94dd771bb652",
            "originalTransactionHash": "100d1edd0434938ec39e6cb5059601b4618a1ca25b91c38e5be9e75444b3c4f5",
            "originalSender": "erd1wavgcxq9tfyrw49k3s3h34085mayu82wqvpd4h6akyh8559pkklsknwhwh",
            "sourceShard": 4294967295,
            "destinationShard": 1,
            "blockNonce": 5547876,
            "blockHash": "0d7caaf8f2bf46e913f91867527d44cd1c77453c9aee50d91a10739bd272d00c",
            "notarizedAtSourceInMetaNonce": 5551265,
            "NotarizedAtSourceInMetaHash": "4c87bc5161925a3902e43a7f9f186e63f21f827ef1129ad0e609a0d45dca016a",
            "notarizedAtDestinationInMetaNonce": 5551269,
            "notarizedAtDestinationInMetaHash": "83bfa8463558ee6d2c90b34ee03782619b699fea667acfb98924227bacbba93d",
            "miniblockType": "SmartContractResultBlock",
            "miniblockHash": "c12693db88e3b69b68d5279fd8939ec75b7f0d8e529e7fd950c83b5716a436bd",
            "hyperblockNonce": 5551269,
            "hyperblockHash": "83bfa8463558ee6d2c90b34ee03782619b699fea667acfb98924227bacbba93d",
            "timestamp": 1727699210,
            "logs": {
                "address": "erd1qqqqqqqqqqqqqpgq0mhy244pyr9pzdhahvvyze4rw3xl29q4kklszyzq72",
                "events": [
                    {
                        "address": "erd1qqqqqqqqqqqqqpgq0mhy244pyr9pzdhahvvyze4rw3xl29q4kklszyzq72",
                        "identifier": "transferValueOnly",
                        "topics": [
                            "I4byb8EAAA==",
                            "AAAAAAAAAAAFAMMiO8pDAH5z5hUCqfc+N03C7UI6tb8="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                            "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                            "ZGVwbG95SW50ZXJjaGFpblRva2Vu",
                            "GeLN3wLxaJKDPbaxdmqkIh0pFNi1l8WJeqy9TofeG40=",
                            "YXZhbGFuY2hlLWZ1amk=",
                            "SVRTVGVzdFRva2Vu",
                            "SVRTVFQ=",
                            "Bg==",
                            "d1iMGAVaSDdUtowjeNXnpvpOHU4DAtrfXbEuelChtb8="
                        ]
                    }
                ]
            },
            "status": "success",
            "operation": "transfer",
            "fee": "0",
            "callType": "asynchronousCallBack",
            "options": 0
        }
    },
    "error": "",
    "code": "successful"
}
        "#;

        let transaction = serde_json::from_str::<TransactionInfo>(data).unwrap();
        assert_eq!(
            transaction.data.unwrap().transaction.hash.unwrap(),
            "34cd9c6d0f68c0975971352ed4dcaacc1acd9a2dbd8f5840a2866d09b1d72298"
        );
    }
}
