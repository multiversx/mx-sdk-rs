extern crate non_fungible_tokens;
use non_fungible_tokens::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/non-fungible-tokens.wasm",
		Box::new(|context| Box::new(NonFungibleTokensImpl::new(context))),
	);
	contract_map
}

#[test]
fn nft_init() {
	parse_execute_mandos("mandos/nft-init.scen.json", &contract_map());
}

#[test]
fn mint_more_tokens_receiver_owner() {
	parse_execute_mandos(
		"mandos/nft-mint-more-tokens-receiver-owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn mint_more_tokens_receiver_acc1() {
	parse_execute_mandos(
		"mandos/nft-mint-more-tokens-receiver-acc1.scen.json",
		&contract_map(),
	);
}
#[test]
fn mint_more_tokens_caller_not_owner() {
	parse_execute_mandos(
		"mandos/nft-mint-more-tokens-caller-not-owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_token_ok() {
	parse_execute_mandos("mandos/nft-transfer-token-ok.scen.json", &contract_map());
}

#[test]
fn transfer_token_steal() {
	parse_execute_mandos(
		"mandos/nft-transfer-token-not-owner-no-approval-to-caller.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_token_without_approval() {
	parse_execute_mandos(
		"mandos/nft-transfer-token-not-owner-no-approval-to-other.scen.json",
		&contract_map(),
	);
}

#[test]
fn approve_ok() {
	parse_execute_mandos("mandos/nft-approve-ok.scen.json", &contract_map());
}

#[test]
fn approve_non_owned_token() {
	parse_execute_mandos(
		"mandos/nft-approve-non-owned-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn approve_non_existent_token() {
	parse_execute_mandos(
		"mandos/nft-approve-non-existent-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn revoke_ok() {
	parse_execute_mandos("mandos/nft-revoke-ok.scen.json", &contract_map());
}

#[test]
fn revoke_non_approved() {
	parse_execute_mandos("mandos/nft-revoke-non-approved.scen.json", &contract_map());
}

#[test]
fn transfer_ok() {
	parse_execute_mandos("mandos/nft-transfer-ok.scen.json", &contract_map());
}

#[test]
fn transfer_non_existent_token() {
	parse_execute_mandos(
		"mandos/nft-transfer-non-existent-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_non_owned_without_approval() {
	parse_execute_mandos(
		"mandos/nft-transfer-not-owned-not-approved-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_approved_token() {
	parse_execute_mandos(
		"mandos/nft-transfer-approved-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_after_revoked() {
	parse_execute_mandos(
		"mandos/nft-transfer-token-after-revoked.scen.json",
		&contract_map(),
	);
}
