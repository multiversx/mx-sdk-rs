use multiversx_sc_scenario::{
    multiversx_sc::{chain_core::std::new_address::compute_new_address, types::Address},
    scenario_format::value_interpreter::keccak256,
};

fn get_initial_dns_address() -> Address {
    Address::from_slice(&[1u8; 32])
}

fn compute_dns_address_for_shard_id(shard_id: u8) -> Address {
    let initial_dns_address = get_initial_dns_address();
    let initial_dns_address_slice = initial_dns_address.as_array();
    let shard_identifier = &[0u8, shard_id];
    let deployer_pubkey_prefix =
        &initial_dns_address_slice[0..initial_dns_address_slice.len() - shard_identifier.len()];

    let deployer_pubkey = [deployer_pubkey_prefix, shard_identifier].concat();
    let deployer_address = Address::from_slice(&deployer_pubkey);
    compute_new_address(&deployer_address, 0)
}

fn shard_id_from_name(name: &str) -> u8 {
    let name_hash = keccak256(name.as_bytes());
    name_hash[31]
}

pub fn dns_address_for_name(name: &str) -> Address {
    let shard_id = shard_id_from_name(name);
    compute_dns_address_for_shard_id(shard_id)
}

#[test]
fn test_compute_dns_address() {
    assert_eq!(
        compute_dns_address_for_shard_id(0).to_bech32_default(),
        "erd1qqqqqqqqqqqqqpgqnhvsujzd95jz6fyv3ldmynlf97tscs9nqqqq49en6w"
    );
    assert_eq!(
        compute_dns_address_for_shard_id(1).to_bech32_default(),
        "erd1qqqqqqqqqqqqqpgqysmcsfkqed279x6jvs694th4e4v50p4pqqqsxwywm0"
    );
    assert_eq!(
        compute_dns_address_for_shard_id(2).to_bech32_default(),
        "erd1qqqqqqqqqqqqqpgqnk5fq8sgg4vc63ffzf7qez550xe2l5jgqqpqe53dcq"
    );
}

#[test]
fn test_dns_for_name() {
    assert_eq!(
        dns_address_for_name("test.elrond")
            .to_bech32_default()
            .bech32,
        "erd1qqqqqqqqqqqqqpgqx4ca3eu4k6w63hl8pjjyq2cp7ul7a4ukqz0skq6fxj"
    );
    assert_eq!(
        dns_address_for_name("helloworld.elrond")
            .to_bech32_default()
            .bech32,
        "erd1qqqqqqqqqqqqqpgqhcm9k2xkk75e47wpmvfgj8fuzwaguvzyqqrqsteg8w"
    );
}
