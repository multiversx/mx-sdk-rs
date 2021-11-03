use elrond_wasm::*;
use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/erc721.wasm",
        Box::new(|context| Box::new(erc721::contract_obj(context))),
    );
    blockchain
}

#[test]
fn nft_approve_non_existent_token_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-approve-non-existent-token.scen.json", world());
}

#[test]
fn nft_approve_non_owned_token_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-approve-non-owned-token.scen.json", world());
}

#[test]
fn nft_approve_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-approve-ok.scen.json", world());
}

#[test]
fn nft_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-init.scen.json", world());
}

#[test]
fn nft_mint_more_tokens_caller_not_owner_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/nft-mint-more-tokens-caller-not-owner.scen.json",
        world(),
    );
}

#[test]
fn nft_mint_more_tokens_receiver_acc1_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/nft-mint-more-tokens-receiver-acc1.scen.json",
        world(),
    );
}

#[test]
fn nft_mint_more_tokens_receiver_owner_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/nft-mint-more-tokens-receiver-owner.scen.json",
        world(),
    );
}

#[test]
fn nft_revoke_non_approved_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-revoke-non-approved.scen.json", world());
}

#[test]
fn nft_revoke_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-revoke-ok.scen.json", world());
}

#[test]
fn nft_transfer_approved_token_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-transfer-approved-token.scen.json", world());
}

#[test]
fn nft_transfer_non_existent_token_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-transfer-non-existent-token.scen.json", world());
}

#[test]
fn nft_transfer_not_owned_not_approved_token_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/nft-transfer-not-owned-not-approved-token.scen.json",
        world(),
    );
}

#[test]
fn nft_transfer_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-transfer-ok.scen.json", world());
}

#[test]
fn nft_transfer_token_after_revoked_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-transfer-token-after-revoked.scen.json", world());
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_caller_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/nft-transfer-token-not-owner-no-approval-to-caller.scen.json",
        world(),
    );
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_other_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/nft-transfer-token-not-owner-no-approval-to-other.scen.json",
        world(),
    );
}

#[test]
fn nft_transfer_token_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-transfer-token-ok.scen.json", world());
}
