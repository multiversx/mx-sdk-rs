//! Checks the exact data field produced by the system SC proxies,
//! and the decoding of the system SC view results.
//!
//! Expected formats are taken from the system SC implementations in mx-chain-go
//! (vm/systemSmartContracts) and from mxpy/mx-sdk-py.

use multiversx_sc::codec::{
    TopDecodeMulti,
    multi_types::{MultiValue2, MultiValueVec},
};
use multiversx_sc_scenario::imports::*;

fn tx() -> TxBaseWithEnv<ScenarioTxEnvData> {
    Tx::new_with_env(ScenarioTxEnvData::default())
}

fn hex_str(s: &str) -> String {
    hex(s.as_bytes())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn data_string(buffer: ManagedBuffer<StaticApi>) -> String {
    String::from_utf8(buffer.to_vec()).unwrap()
}

fn bls_key(b: u8) -> BLSKey {
    BLSKey::from([b; 96])
}

fn bls_sig(b: u8) -> BLSSignature {
    BLSSignature::from([b; 48])
}

fn decode<T: TopDecodeMulti>(values: &[&[u8]]) -> T {
    let mut input: Vec<Vec<u8>> = values.iter().map(|v| v.to_vec()).collect();
    T::multi_decode(&mut input).unwrap()
}

// ---------------------------------------------------------------------------
// Governance
// ---------------------------------------------------------------------------

#[test]
fn governance_proposal_data() {
    let call = tx()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .proposal(
            "6db132d759482f9f3515fe3ca8f72a8d6dc61244",
            9u64,
            11u64,
            BigUint::<StaticApi>::from(500u64),
        );
    assert_eq!(call.payment.0, BigUint::from(500u64));
    assert_eq!(
        data_string(call.to_call_data_string()),
        format!(
            "proposal@{}@09@0b",
            hex_str("6db132d759482f9f3515fe3ca8f72a8d6dc61244")
        )
    );
}

/// changeConfig expects all arguments as base 10 strings.
#[test]
fn governance_change_config_data() {
    let proposal_fee = BigUint::<StaticApi>::from(500_000_000_000_000_000_000u128);
    let lost_proposal_fee = BigUint::<StaticApi>::from(10_000_000_000_000_000_000u128);
    let data = tx()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .change_config(&proposal_fee, &lost_proposal_fee, 2000, 3300, 5000)
        .to_call_data_string();
    assert_eq!(
        data_string(data),
        format!(
            "changeConfig@{}@{}@{}@{}@{}",
            hex_str("500000000000000000000"),
            hex_str("10000000000000000000"),
            hex_str("2000"),
            hex_str("3300"),
            hex_str("5000"),
        )
    );
}

/// claimAccumulatedFees takes no arguments.
#[test]
fn governance_claim_accumulated_fees_data() {
    let data = tx()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .claim_accumulated_fees()
        .to_call_data_string();
    assert_eq!(data_string(data), "claimAccumulatedFees");
}

#[test]
fn governance_vote_data() {
    let data = tx()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .vote(1u64, "veto")
        .to_call_data_string();
    assert_eq!(data_string(data), format!("vote@01@{}", hex_str("veto")));
}

/// viewConfig returns all values as strings, including the last proposal nonce.
#[test]
fn governance_view_config_decode() {
    let result: GovernanceConfigResult = decode(&[
        b"500000000000000000000",
        b"10000000000000000000",
        b"0.2000",
        b"0.3300",
        b"0.5000",
        b"12",
    ]);
    assert_eq!(result.proposal_fee, "500000000000000000000");
    assert_eq!(result.lost_proposal_fee, "10000000000000000000");
    assert_eq!(result.min_quorum, "0.2000");
    assert_eq!(result.min_veto_threshold, "0.3300");
    assert_eq!(result.min_pass_threshold, "0.5000");
    assert_eq!(result.last_proposal_nonce, 12);
}

/// Vote amounts are expressed in EGLD atto-units, so they easily exceed u64.
#[test]
fn governance_view_proposal_decode_large_amounts() {
    let big = 3_000_000_000_000_000_000_000u128; // 3000 EGLD
    let big_bytes = big.to_be_bytes();
    let big_bytes = &big_bytes[big_bytes.iter().position(|b| *b != 0).unwrap()..];
    let issuer = [7u8; 32];

    let result: ProposalViewResult<StaticApi> = decode(&[
        &[0x1b, 0x1a, 0xe4, 0xd6, 0xe2, 0xef, 0x50, 0x00, 0x00], // 500 EGLD
        b"6db132d759482f9f3515fe3ca8f72a8d6dc61244",
        &[1],
        &issuer,
        &[9],
        &[11],
        big_bytes,
        big_bytes,
        &[],
        &[],
        &[],
        b"true",
        b"false",
    ]);
    assert_eq!(
        result.proposal_cost,
        BigUint::from(500_000_000_000_000_000_000u128)
    );
    assert_eq!(result.proposal_nonce, 1);
    assert_eq!(result.issuer_address, Address::from(issuer));
    assert_eq!(result.start_vote_epoch, 9);
    assert_eq!(result.end_vote_epoch, 11);
    assert_eq!(result.quorum_stake, BigUint::from(big));
    assert_eq!(result.yes, BigUint::from(big));
    assert_eq!(result.no, BigUint::zero());
    assert!(result.closed);
    assert!(!result.passed);
}

// ---------------------------------------------------------------------------
// Validator
// ---------------------------------------------------------------------------

/// The number of nodes is derived from the list of keys and signatures.
#[test]
fn validator_stake_data() {
    let keys_sigs = MultiValueVec::from(vec![
        MultiValue2::from((bls_key(1), bls_sig(2))),
        MultiValue2::from((bls_key(3), bls_sig(4))),
    ]);
    let data = tx()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .stake(keys_sigs, BigUint::<StaticApi>::from(5000u64))
        .to_call_data_string();
    assert_eq!(
        data_string(data),
        format!(
            "stake@02@{}@{}@{}@{}",
            hex(&[1; 96]),
            hex(&[2; 48]),
            hex(&[3; 96]),
            hex(&[4; 48])
        )
    );
}

#[test]
fn validator_stake_with_reward_address_data() {
    let keys_sigs = MultiValueVec::from(vec![MultiValue2::from((bls_key(1), bls_sig(2)))]);
    let reward_address = ManagedAddress::<StaticApi>::from(&Address::from([5u8; 32]));
    let data = tx()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .stake_with_reward_address(keys_sigs, reward_address, BigUint::<StaticApi>::from(1u64))
        .to_call_data_string();
    assert_eq!(
        data_string(data),
        format!(
            "stake@01@{}@{}@{}",
            hex(&[1; 96]),
            hex(&[2; 48]),
            hex(&[5; 32])
        )
    );
}

#[test]
fn validator_stake_top_up_data() {
    let data = tx()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .stake_top_up(BigUint::<StaticApi>::from(1u64))
        .to_call_data_string();
    assert_eq!(data_string(data), "stake");
}

#[test]
fn validator_unjail_payment() {
    let call = tx()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .unjail(
            MultiValueVec::from(vec![bls_key(1), bls_key(2)]),
            BigUint::from(2_500_000_000_000_000_000u128),
        );
    assert_eq!(call.payment.0, BigUint::from(5_000_000_000_000_000_000u128));
    assert_eq!(
        data_string(call.to_call_data_string()),
        format!("unJail@{}@{}", hex(&[1; 96]), hex(&[2; 96]))
    );
}

// ---------------------------------------------------------------------------
// Delegation
// ---------------------------------------------------------------------------

#[test]
fn delegation_unjail_nodes_payment() {
    let call = tx()
        .to(ValidatorSystemSCAddress) // any address will do
        .typed(DelegationSCProxy)
        .unjail_nodes(
            MultiValueVec::from(vec![bls_key(1), bls_key(2), bls_key(3)]),
            BigUint::from(2_500_000_000_000_000_000u128),
        );
    assert_eq!(call.payment.0, BigUint::from(7_500_000_000_000_000_000u128));
}

#[test]
fn delegation_node_states_decode() {
    let result: DelegationNodeStatesResult = decode(&[
        b"staked",
        &[1; 96],
        &[2; 96],
        b"notStaked",
        &[3; 96],
        b"unStaked",
        &[4; 96],
    ]);
    assert_eq!(result.staked, vec![bls_key(1), bls_key(2)]);
    assert_eq!(result.not_staked, vec![bls_key(3)]);
    assert_eq!(result.un_staked, vec![bls_key(4)]);
}

/// Empty groups are omitted, together with their label.
#[test]
fn delegation_node_states_decode_partial() {
    let result: DelegationNodeStatesResult = decode(&[b"unStaked", &[4; 96]]);
    assert!(result.staked.is_empty());
    assert!(result.not_staked.is_empty());
    assert_eq!(result.un_staked, vec![bls_key(4)]);

    let result: DelegationNodeStatesResult = decode(&[]);
    assert_eq!(result, DelegationNodeStatesResult::default());
}

#[test]
fn delegation_contract_config_decode() {
    let owner = [9u8; 32];
    let result: DelegationContractConfigResult<StaticApi> = decode(&[
        &owner,
        &[0x0e, 0xa1], // 3745
        &[],
        &[0x43, 0xc3, 0x3c, 0x19, 0x37, 0x56, 0x48, 0x00, 0x00], // 1250 EGLD
        b"false",
        b"false",
        b"true",
        b"true",
        &[0x01, 0x00],
        &[10],
    ]);
    assert_eq!(result.owner, ManagedAddress::from(&Address::from(owner)));
    assert_eq!(result.service_fee, 3745);
    assert_eq!(result.max_delegation_cap, BigUint::zero());
    assert_eq!(
        result.initial_owner_funds,
        BigUint::from(1_250_000_000_000_000_000_000u128)
    );
    assert!(!result.automatic_activation);
    assert!(!result.with_delegation_cap);
    assert!(result.changeable_service_fee);
    assert!(result.check_cap_on_redelegate);
    assert_eq!(result.created_nonce, 256);
    assert_eq!(result.unbond_period_in_epochs, 10);
}

// ---------------------------------------------------------------------------
// ESDT system SC
// ---------------------------------------------------------------------------

/// Dynamic NFTs and SFTs take no decimals argument.
#[test]
fn esdt_issue_dynamic_sft_data() {
    let data = tx()
        .to(ESDTSystemSCAddress)
        .typed(ESDTSystemSCProxy)
        .issue_dynamic(
            BigUint::from(50_000_000_000_000_000u64),
            "TestSFT",
            "TSFT",
            EsdtTokenType::DynamicSFT,
            0,
        )
        .to_call_data_string();
    assert_eq!(
        data_string(data),
        format!(
            "registerDynamic@{}@{}@{}",
            hex_str("TestSFT"),
            hex_str("TSFT"),
            hex_str("SFT")
        )
    );
}

#[test]
fn esdt_issue_dynamic_meta_data() {
    let data = tx()
        .to(ESDTSystemSCAddress)
        .typed(ESDTSystemSCProxy)
        .issue_dynamic(
            BigUint::from(50_000_000_000_000_000u64),
            "TestMeta",
            "TMETA",
            EsdtTokenType::DynamicMeta,
            18,
        )
        .to_call_data_string();
    assert_eq!(
        data_string(data),
        format!(
            "registerDynamic@{}@{}@{}@12",
            hex_str("TestMeta"),
            hex_str("TMETA"),
            hex_str("META")
        )
    );
}

#[test]
fn esdt_issue_and_set_all_roles_dynamic_sft_data() {
    let data = tx()
        .to(ESDTSystemSCAddress)
        .typed(ESDTSystemSCProxy)
        .issue_and_set_all_roles(
            BigUint::from(50_000_000_000_000_000u64),
            "TestSFT",
            "TSFT",
            EsdtTokenType::DynamicSFT,
            0,
        )
        .to_call_data_string();
    assert_eq!(
        data_string(data),
        format!(
            "registerAndSetAllRolesDynamic@{}@{}@{}",
            hex_str("TestSFT"),
            hex_str("TSFT"),
            hex_str("SFT")
        )
    );
}

#[test]
fn esdt_set_special_roles_nft_update_data() {
    let address = ManagedAddress::<StaticApi>::from(&Address::from([5u8; 32]));
    let data = tx()
        .to(ESDTSystemSCAddress)
        .typed(ESDTSystemSCProxy)
        .set_special_roles(
            address,
            TestTokenIdentifier::new("NFT-123456"),
            [EsdtLocalRole::NftUpdate, EsdtLocalRole::ModifyCreator].into_iter(),
        )
        .to_call_data_string();
    assert_eq!(
        data_string(data),
        format!(
            "setSpecialRole@{}@{}@{}@{}",
            hex_str("NFT-123456"),
            hex(&[5; 32]),
            hex_str("ESDTRoleNFTUpdate"),
            hex_str("ESDTRoleModifyCreator"),
        )
    );
}

#[test]
fn esdt_token_type_from_string() {
    let cases = [
        ("FungibleESDT", EsdtTokenType::Fungible),
        ("NonFungibleESDT", EsdtTokenType::NonFungible),
        ("NonFungibleESDTv2", EsdtTokenType::NonFungibleV2),
        ("SemiFungibleESDT", EsdtTokenType::SemiFungible),
        ("MetaESDT", EsdtTokenType::MetaFungible),
        ("DynamicNonFungibleESDT", EsdtTokenType::DynamicNFT),
        ("DynamicSemiFungibleESDT", EsdtTokenType::DynamicSFT),
        ("DynamicMetaESDT", EsdtTokenType::DynamicMeta),
        ("Unknown", EsdtTokenType::Invalid),
    ];
    for (name, expected) in cases {
        assert_eq!(
            EsdtTokenType::collection_from_string(name),
            expected,
            "{name}"
        );
    }
}

/// Minted and burnt values precede the named properties.
#[test]
fn esdt_token_properties_fungible_decode() {
    let owner = [3u8; 32];
    let result: TokenPropertiesResult = decode(&[
        b"TestToken",
        b"FungibleESDT",
        &owner,
        b"1000000000000000000000000000000000000000000000000000000000000000000000",
        b"25",
        b"NumDecimals-6",
        b"IsPaused-true",
        b"CanUpgrade-true",
        b"CanMint-false",
        b"CanBurn-false",
        b"CanChangeOwner-true",
        b"CanPause-true",
        b"CanFreeze-true",
        b"CanWipe-true",
        b"CanAddSpecialRoles-true",
        b"CanTransferNFTCreateRole-false",
        b"NFTCreateStopped-false",
        b"NumWiped-2",
    ]);
    assert_eq!(result.token_name, "TestToken");
    assert_eq!(result.token_type, EsdtTokenType::Fungible);
    assert_eq!(result.owner_address, Address::from(owner));
    assert_eq!(
        result.minted_value,
        "1000000000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(result.burnt_value, "25");
    assert_eq!(result.num_decimals, 6);
    assert!(result.is_paused);
    assert!(result.can_upgrade);
    assert!(!result.can_mint);
    assert!(result.can_change_owner);
    assert!(result.can_wipe);
    assert!(!result.can_transfer_nft_create_role);
    assert_eq!(result.num_wiped, 2);
}
