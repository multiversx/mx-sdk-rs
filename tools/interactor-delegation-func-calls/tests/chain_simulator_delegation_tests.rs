use std::fs;

use delegation_sc_interact::{Config, DelegateCallsInteract};
use multiversx_sc_snippets::hex;

pub fn extract_public_key_from_str(header: &str) -> Option<String> {
    if header.contains("BEGIN") {
        let mut parts = header.split_whitespace();

        if let Some(last) = parts.next_back() {
            let address = last.replace("-----", "").replace("\n", "");
            return Some(address);
        }
    }

    None
}

pub fn get_validator_public_key(file: &str) -> Option<Vec<u8>> {
    let pem_content = fs::read_to_string(file).expect("Failed to read PEM file");
    let lines: Vec<&str> = pem_content.split('\n').collect();
    let header = lines[0];
    if let Some(public_key) = extract_public_key_from_str(header) {
        return Some(hex::decode(public_key).expect("Failed to decode public key from hex"));
    }

    None
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn cs_builtin_run_tests() {
    let mut interactor = DelegateCallsInteract::new(Config::chain_simulator_config()).await;

    interactor.set_state().await;
    interactor
        .create_new_delegation_contract(7231941000000000000000u128, 3745u64)
        .await;
    interactor
        .set_metadata("Test Mx Provider", "testmx.provider", "testmxprovider")
        .await;
    interactor.set_automatic_activation(false).await;
    interactor
        .modify_total_delegation_cap(7231941000000000000000u128)
        .await;

    let public_key_46 = get_validator_public_key("./validatorKey_46.pem")
        .expect("Failed to get validator public key");
    let public_key_11 = get_validator_public_key("./validatorKey_11.pem")
        .expect("Failed to get validator public key");
    interactor
        .add_nodes(
            vec![public_key_46.clone(), public_key_11.clone()],
            vec!["signed1", "signed2"],
        )
        .await;

    interactor.delegate(5000000000000000000u128).await;
}
