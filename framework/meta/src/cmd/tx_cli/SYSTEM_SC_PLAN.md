# System smart contracts in `sc-meta tx` — plan

Goal: make working with the MultiversX system smart contracts (ESDT, validator, delegation manager,
delegation, governance) available from `sc-meta tx system ...`, correct against mx-chain-go, and documented.

## Background

- The `tools/interactor-*` crates (system, delegation, governance) are chain-simulator test harnesses
  (hardcoded test wallets, `config.toml`/`state.toml`, `set_state`), not user tools.
  All their on-chain logic goes through the framework proxies in
  `framework/base/src/types/interaction/system_proxy/`.
- The CLI is therefore built on those proxies and on the interactor machinery already used by
  the rest of `sc-meta tx` (`Interactor`, `into_sdk_transaction`, `sign_and_dispatch`), not by porting the interactor crates.
- mxpy equivalents: `mxpy token`, `mxpy validator`, `mxpy staking-provider`, `mxpy governance`.
  mxpy offers almost no queries outside governance; we aim to cover the views too.
- Reference implementation: `mx-chain-go/vm/systemSmartContracts` (`esdt.go`, `validator.go`,
  `delegationManager.go`, `delegation.go`, `governance.go`), plus builtin functions in `mx-chain-vm-common-go`.

## Phase 0 — fix the proxies ✅ done

Proxies verified against mx-chain-go, bugs fixed, missing endpoints added, call data and result decoding covered by
`framework/scenario/tests/system_proxy_call_data_test.rs`.

- Wire bugs: governance `claimAccumulatedFees` (no args), `changeConfig` (decimal strings),
  hardcoded proposal fee / unjail fee, validator `stake` node count derived from keys,
  legacy proxy DynamicSFT decimals.
- Decoding bugs: `getAllNodeStates`, `getAllContractAddresses`, token type names (`FungibleESDT`, `NonFungibleESDTv2`),
  `getTokenProperties` (minted/burnt values), governance views (`BigUint` amounts, decimal nonce), untyped view results.
- Added: `EsdtLocalRole::NftUpdate`; full validator proxy; delegation getters, whitelist/merge, owner change;
  delegation manager merge/multi-claim/config; ESDT `stopNFTCreate`, `changeToMultiShardCreate`,
  burn-role-globally, `sendAllTransferRoleAddresses`, `getAllAddressesAndRoles`, `getContractConfig`.
- Deprecated: ESDT system SC `mint`/`burn` (global mint/burn disabled on-chain).
- Not done: `canCreateMultiShard` issue property (flag-gated, would change public structs).

## Phase 1 — CLI under `sc-meta tx system`

```
sc-meta tx system esdt <cmd>
sc-meta tx system validator <cmd>
sc-meta tx system delegation-manager <cmd>
sc-meta tx system delegation <contract> <cmd>
sc-meta tx system governance <cmd>
```

Conventions:

- Reuse `GatewayArgs`, `SenderArgs`, `RelayerArgs`, `--send` / `--wait-result` / `--outfile`, relayed v3.
- `--gas-limit` optional, with a default gas limit per command (mxpy values).
- Required payments resolved from chain when `--value` is omitted:
  ESDT issue cost (ESDT `getContractConfig`), min deposit (delegation manager `getContractConfig`),
  proposal fee (governance `viewConfig`).
- Queries need no wallet and print typed JSON.
- Roles by name (`--roles LocalMint,NftCreate` or the full `ESDTRoleLocalMint` names).

Status: ESDT ✅, governance ✅, delegation manager + delegation ✅, validator ✅.

- `esdt`: issue/register (classic, dynamic, set-all-roles), roles, pause/freeze/wipe, ownership, control-changes,
  dynamic conversions; queries `token-properties`, `special-roles`, `contract-config`.
- `governance`: propose (fee from `viewConfig`), vote, close, clear ended proposals, claim fees, change config;
  queries `config`, `proposal`, `voting-power`, `delegated-vote-info`. `delegateVote` is SC-only, not exposed.
- `delegation-manager`: create (min deposit from config), from validator, merge, claim/redelegate multi;
  queries `contract-config`, `contract-addresses`.
- `delegation <contract>`: owner settings, node management, whitelist/ownership, delegate/claim/redelegate/undelegate/withdraw;
  queries `contract-config`, `metadata`, `node-states`, `totals`, `delegator`, `reward-data`, `whitelisted-for-merge`.
- `validator`: stake (+ reward address), top-up, unstake/unbond (nodes, tokens), restake, unjail, reward address,
  clean registered data; queries `total-staked`, `total-staked-top-up`, `unstaked-tokens`, `bls-keys-status`.

Implementation (`tx_cli_system/`): typed system SC proxies, working directly with the interactor.
`system_common.rs` has `system_proxy` (typed proxy from interactor/sender/relayer/gas, no CLI dependency)
and `system_tx` (CLI glue: wallet, relayer, build via async closure, nonce, sign, dispatch).
Default gas (no `--gas-limit`), like mxpy: operation cost + network min gas limit + gas per data byte × data length
(+ 50,000 when relayed).

BLS: `--bls-keys` (+ `--signatures` for stake/add-nodes) always work; `--validators-pem` signing needs the optional
`bls` feature (`multiversx-sdk/bls`, forwarded by `multiversx-sc-meta/bls`), since `multiversx-bls` links prebuilt
static libs and does not support Windows MSVC. Each key signs the staker address (sender for `stake`,
delegation contract for `addNodes`), as verified by mx-chain-go.

Tests: `cs_tx_system_esdt_test.rs` and `cs_tx_system_staking_test.rs` (both in `cs-test-runner`),
`sdk/core/tests/validator_test.rs` (BLS signing, with `--features bls`).
Not covered on the chain simulator: node activation (default config has node limit 0 — "number of nodes too high"),
so stake-top-up, change-reward-address, BLS key status and node staking can't be exercised there; it also
does not verify BLS signatures.

## Phase 2 — tests and interactor crates

- Chain-simulator tests behind sc-meta's `chain-simulator-tests` feature, calling the CLI handlers end to end.
- Fix the interactor test bugs (duplicate carol wallet, nonce hex formatting for 11–15, `control_changes` sending
  `canMint` for NFTs), keep the crates as chain-simulator tests for the proxies, drop their bespoke CLIs/bins.
- The delegation and governance chain-simulator tests are `#[ignore]`d and not in `cs-test-runner`:
  they need a simulator with a non-zero node limit, and `validatorKey3.pem` is missing.

## Phase 3 — docs (mx-docs)

- Reference page for `sc-meta tx` (currently undocumented), plus a `tx system` page with one runnable example per SC.
- "System SC proxies" page under developers/transactions; replace the "here TBD" link in `tx-to.md`.
- Update `sc-to-sc-calls.md` to `self.tx()...typed(ESDTSystemSCProxy)`.
- Fix stale items: unjail examples send 2500 EGLD (should be 2.5 EGLD per node), wrong `funcName` in the
  `getDelegatorFundsData` example, inconsistent NFT role names, empty system SC gas/fees page, `stake` "genesis" placeholder.
- Document missing endpoints: governance `changeConfig`, `claimAccumulatedFees`, `clearEndedProposals` tx format;
  validator `unStakeNodes`/`unStakeTokens`/`unBondNodes`/`unBondTokens`/`reStakeUnStakedNodes` and views;
  ESDT `registerAndSetAllRoles`, `set/unsetBurnRoleGlobally`; delegation `setCheckCapOnReDelegateRewards`.

## Related: legacy delegation contract (mx-delegation-sc)

- Its "auction SC" is the validator system SC (`erd1…plllst77y4l`).
- `claimUnusedFunds` calls `claim` on the validator SC, which fails since StakingV2 ("claim function is disabled").
- `auction-mock` has not been checked against `validator.go`.
