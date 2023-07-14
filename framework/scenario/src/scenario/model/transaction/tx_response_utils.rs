use multiversx_sdk::data::transaction::ApiSmartContractResult;

/// Checks for invalid topics.
pub fn process_topics_error(topics: Option<&Vec<String>>) -> Option<String> {
    if topics.is_none() {
        return Some("missing topics".to_string());
    }

    let topics = topics.unwrap();
    if topics.len() != 2 {
        Some(format!(
            "expected to have 2 topics, found {} instead",
            topics.len()
        ))
    } else {
        None
    }
}

/// Decodes the data of a smart contract result.
pub fn decode_scr_data_or_panic(data: &str) -> Vec<Vec<u8>> {
    let mut split = data.split('@');
    let _ = split.next().expect("SCR data should start with '@'");
    let result_code = split.next().expect("missing result code");
    assert_eq!(result_code, "6f6b", "result code is not 'ok'");

    split
        .map(|encoded_arg| hex::decode(encoded_arg).expect("error hex-decoding result"))
        .collect()
}

/// Checks if the given smart contract result is an out smart contract result.
pub fn is_out_scr(scr: &&ApiSmartContractResult) -> bool {
    scr.nonce != 0 && scr.data.starts_with('@')
}
