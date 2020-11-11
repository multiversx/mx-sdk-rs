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
	parse_execute_mandos(
		"mandos/nft-init.scen.json",
		&contract_map(),
	);
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
	parse_execute_mandos(
		"mandos/nft-transfer-token-ok.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_token_steal() {
	parse_execute_mandos(
		"mandos/nft-transfer-token-not-owner-no-allowance-to-caller.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_token_without_allowance() {
	parse_execute_mandos(
		"mandos/nft-transfer-token-not-owner-no-allowance-to-other.scen.json",
		&contract_map(),
	);
}
