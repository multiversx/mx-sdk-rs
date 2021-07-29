#[test]
fn nft_approve_non_existent_token_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-approve-non-existent-token.scen.json");
}

#[test]
fn nft_approve_non_owned_token_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-approve-non-owned-token.scen.json");
}

#[test]
fn nft_approve_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-approve-ok.scen.json");
}

#[test]
fn nft_init_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-init.scen.json");
}

#[test]
fn nft_mint_more_tokens_caller_not_owner_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-mint-more-tokens-caller-not-owner.scen.json");
}

#[test]
fn nft_mint_more_tokens_receiver_acc1_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-mint-more-tokens-receiver-acc1.scen.json");
}

#[test]
fn nft_mint_more_tokens_receiver_owner_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-mint-more-tokens-receiver-owner.scen.json");
}

#[test]
fn nft_revoke_non_approved_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-revoke-non-approved.scen.json");
}

#[test]
fn nft_revoke_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-revoke-ok.scen.json");
}

#[test]
fn nft_transfer_approved_token_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-transfer-approved-token.scen.json");
}

#[test]
fn nft_transfer_non_existent_token_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-transfer-non-existent-token.scen.json");
}

#[test]
fn nft_transfer_not_owned_not_approved_token_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-transfer-not-owned-not-approved-token.scen.json");
}

#[test]
fn nft_transfer_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-transfer-ok.scen.json");
}

#[test]
fn nft_transfer_token_after_revoked_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-transfer-token-after-revoked.scen.json");
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_caller_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/nft-transfer-token-not-owner-no-approval-to-caller.scen.json",
    );
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_other_go() {
    elrond_wasm_debug::mandos_go(
        "mandos/nft-transfer-token-not-owner-no-approval-to-other.scen.json",
    );
}

#[test]
fn nft_transfer_token_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/nft-transfer-token-ok.scen.json");
}
