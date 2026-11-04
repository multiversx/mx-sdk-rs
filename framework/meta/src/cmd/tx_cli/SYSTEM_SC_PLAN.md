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

Order: ESDT (in progress), governance, delegation manager + delegation, validator.
The validator commands need a validator PEM loader and BLS signing in `sdk/core` (via `multiversx-bls`),
alternatively raw `--bls-key`/`--signature` hex pairs.

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
