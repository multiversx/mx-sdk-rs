# Forwarder Blind

A feature-test contract that **blindly forwards calls and payments**, with exactly one endpoint per call type. Every endpoint forwards the full payment received, without inspecting or modifying it.

It is a simplified counterpart to [forwarder-raw](../forwarder-raw), which has multiple endpoints per call type and serves as a more exhaustive API surface test. `forwarder-blind` additionally includes an endpoint for **async v2 (promises)**, which `forwarder-raw` does not have.

---

## Modules

### `fwd_blind_common`
Shared helper used across modules.

- `tx_gas()` — returns `get_gas_left() - 500_000`. Used to reserve a gas overhead before forwarding.

---

### `fwd_blind_async_v1` — Async Call (Legacy)
Uses the async v1 mechanism (`async_call_and_exit`).

- **`blind_async_v1(to, endpoint_name, ...args)`** `#[payable]`  
  Forwards the full payment to `to::endpoint_name(args)` as an async call. Captures the original caller so the callback can return funds.

- **`blind_async_v1_callback`** `#[callback]`  
  On success: emits `blind_async_v1_callback_ok` and sends any back-payments to the original caller.  
  On error: emits `blind_async_v1_callback_error` with the error code and message.

---

### `fwd_blind_async_v2` — Async Promises (V2)
Uses the async v2 mechanism (`register_promise`).

- **`blind_async_v2(to, endpoint_name, ...args)`** `#[payable]`  
  Forwards the full payment to `to::endpoint_name(args)` as a promise. Allocates explicit gas for the call and the callback (`ASYNC_V2_CALLBACK_GAS`). Captures the original caller.

- **`blind_async_v2_callback`** `#[promises_callback]`  
  On success: emits `async_v2_callback_ok` and sends any back-payments to the original caller.  
  On error: emits `async_v2_callback_error` with the error code and message.

---

### `fwd_blind_sync` — Synchronous Call

- **`blind_sync_call(to, endpoint_name, ...args)`** `#[payable]`  
  Forwards the full payment to `to::endpoint_name(args)` synchronously. Any tokens or EGLD returned (back-transfers) are forwarded to the original caller.

---

### `fwd_blind_transf_exec` — Transfer & Execute

- **`blind_transf_exec(to, endpoint_name, ...args)`** `#[payable]`  
  Forwards the full payment via a transfer-execute call (fire-and-forget, no callback).

---

### `fwd_blind_deploy` — Deploy

- **`blind_deploy(code, code_metadata, ...args)`**  
  Deploys a new contract with the given code and metadata. Returns the new contract's address and emits `blind_deploy_ok_event`.

---

### `fwd_blind_upgrade` — Upgrade

- **`blind_upgrade(sc_address, code_metadata, ...args)`**  
  Upgrades an existing contract at `sc_address` using the new code passed as an argument.

- **`blind_upgrade_from_source(sc_address, source_address, code_metadata, ...args)`**  
  Upgrades an existing contract using code copied from `source_address`.

---

## Scenarios

Scenario tests are located in [`../scenarios/`](../scenarios) and cover:

| Scenario | Description |
|--|--|
| `forw_blind_async_v1_accept_egld` | Send EGLD to vault via async v1 |
| `forw_blind_async_v1_accept_esdt` | Send ESDT to vault via async v1 |
| `forw_blind_async_v1_accept_nft` | Send NFT to vault via async v1 |
| `forw_blind_async_v1_retrieve_egld` | Retrieve EGLD from vault back to caller via async v1 callback |
| `forw_blind_async_v1_retrieve_nft` | Retrieve NFT from vault back to caller via async v1 callback |
| `forw_blind_async_v2_accept_egld` | Send EGLD to vault via async v2 promise |
| `forw_blind_async_v2_retrieve_egld` | Retrieve EGLD from vault back to caller via async v2 callback |
| `forw_blind_sync_accept_egld` | Send EGLD to vault via sync call |
| `forw_blind_sync_retrieve_egld` | Retrieve EGLD from vault back to caller via sync call |
| `forw_blind_transf_exec_accept_egld` | Send EGLD to vault via transfer-execute |
| `forw_blind_deploy` | Deploy a new contract |
| `forw_blind_upgrade` | Upgrade an existing contract |
