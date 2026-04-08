use std::{fs::read_to_string, path::PathBuf};

use multiversx_sc_meta_lib::abi_json::deserialize_abi_from_json;

/// Test all historical ABI versions from newest to oldest
#[test]
fn test_read_historical_abi() {
    test_read_historical_abi_version("abi_v0.63.0.abi.json");
    test_read_historical_abi_version("abi_v0.57.0.abi.json");
    test_read_historical_abi_version("abi_v0.54.0.abi.json");
    test_read_historical_abi_version("abi_v0.44.0.abi.json");
    test_read_historical_abi_version("abi_v0.32.0.abi.json");
    test_read_historical_abi_version("abi_v0.20.0.abi.json");
    test_read_historical_abi_version("abi_v0.18.0.abi.json");
    test_read_historical_abi_version("abi_v0.10.0.abi.json");
}

fn test_read_historical_abi_version(file_name: &str) {
    let abi_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("abi_json_history")
        .join(file_name);

    let abi_raw = read_to_string(&abi_path)
        .unwrap_or_else(|err| panic!("failed to read ABI file {file_name}: {err}"));
    let _abi_json = deserialize_abi_from_json(&abi_raw)
        .unwrap_or_else(|err| panic!("could not decode ABI from file {file_name}: {err}"));
}
