# Kasmer: overview of the existing components

Analysis date: 2026-11-13. Based on a read of the local checkouts of
`kasmer-multiversx` (HEAD `a67a732`, Aug 2024) and `mx-semantics` (HEAD `216530d`, Oct 2024),
both identical to upstream `master`, plus the parametric-test support already in `mx-sdk-rs`.
Nothing was built or run for this analysis; the stack is not installed on this Mac.

## 1. Summary

- **Two repos, one stack.** `mx-semantics` is the K model of the MultiversX VM. `kasmer-multiversx`
  sits on top of it and adds everything needed for *symbolic* execution (lemmas, the proof driver,
  the `kasmer` CLI). `kasmer-multiversx` depends on `mx-semantics`, never the other way round.
- **The valuable, hard-to-replace part is written in K**: about 8,200 lines of VM semantics and
  about 7,500 lines of lemmas. None of that needs porting; it is input to the K compiler.
- **The Python part is about 7,300 lines of glue**, of which roughly 3,000 are on the path that
  `kasmer build / fuzz / verify` actually uses. It does no proving itself. It builds terms, starts
  K processes and talks JSON-RPC to the Haskell backend.
- **Python can be removed from our code, but not from K's build step without extra work.**
  `kompile` (Java) is needed to compile the semantics. At run time only native binaries are needed:
  `kore-rpc-booster` (Haskell), the LLVM-backend interpreter and shared library, and Z3.
- **Both RV repos have been dormant since late 2024**, pinned to K 7.1.85 / 7.1.140. K itself is
  still active (latest release v7.1.337, June 2026), as are the Wasm semantics (v0.1.157). An
  upgrade spans about 200 to 250 K releases and is the main technical risk.
- **The framework side in `mx-sdk-rs` is ahead of the RV repos** (test API integrated in the SC API,
  `test_raw()`, `test_deploy()`, `test_call()`), so the old `testapi` crate is obsolete.

## 2. How the repositories relate

```
mx-sdk-rs (ours)                 test contracts + contracts under test  ──sc-meta──▶  .wasm, .abi.json
   contracts/parametric-tests/*  kasmer.json
   framework: TestApi, test_raw()

kasmer-multiversx (RV)           python pkg `kmxwasm`  — CLI: kasmer, kmxwasm-property
   K: mxwasm-semantics/          MX-WASM = KASMER + lemmas + ceils + proof extensions
        │ depends on (git tag v0.1.107)
        ▼
mx-semantics (RV)                python pkg `kmultiversx` — CLI: mandos, kelrond, runtime
   K: mx-semantics/              ELROND-NODE, ELROND-CONFIG, ESDT, VMHOOKS, MANDOS, KASMER
        │ depends on
        ├──▶ wasm-semantics (RV)        K: WASM, WASM-TEXT;  python pkg `pykwasm` (wasm binary → KAST)
        ├──▶ blockchain-k-plugin (RV)   K: KRYPTO hooks;  C++ lib `krypto.a` for the LLVM backend
        └──▶ K framework (RV)           kompile (Java), LLVM backend, Haskell backend + booster, `pyk`
                 └──▶ Z3
```

Naming is confusing because both repos contain something called "kasmer":

| Thing | Where | What it is |
|---|---|---|
| `kasmer.md` (module `KASMER`) | mx-semantics | K rules for the test-only VM hooks |
| `kmultiversx/kasmer.py` | mx-semantics | Library: deploy the test contract, fuzz, generate claims. No CLI entry point any more |
| `kasmer` CLI | kasmer-multiversx (`kmxwasm.kasmer`) | The user-facing tool: `build`, `fuzz`, `verify` |

The `mx-semantics` README still documents a `kasmer` command and a `contract_paths` key in
`kasmer.json`. Both are stale: the CLI moved to `kasmer-multiversx` and the key is now `contracts`.

## 3. Components

### 3.1 K semantics in `mx-semantics`

Path: `kmultiversx/src/kmultiversx/kdist/mx-semantics/`. About 8,200 lines.

| File | Module | Role |
|---|---|---|
| `elrond-node.md` | `ELROND-NODE` | Configuration: `<node>` with `<commands>`, `<callState>` (embeds `<wasm/>`, bigInt and buffer heaps, bytes stack), `<callStack>`, `<interimStates>`, `<accounts>` |
| `elrond-config.md` | `ELROND-CONFIG` | Account operations, contract call and deploy, creating a Wasm instance per call, exceptions |
| `esdt.md` | `ESDT` | ESDT built-in functions and transfers |
| `vmhooks/*.md` | `VMHOOKS` and others | Host functions: baseOps (37), bigIntOps (44), manBufOps (20), managedei (17), smallIntOps (10), cryptoei (4) |
| `vmhooks/async.md`, `data/async-call.k` | `ASYNC` | Async calls and callbacks |
| `mandos.md` | `MANDOS` | Scenario steps as K commands: `setAccount`, `newAddress`, `deployTx`, `callTx`, `checkExpectStatus`, … |
| `kasmer.md` | `KASMER` | Adds `<wasmStore>` and `<prank>` cells and the 14 test hooks |
| `auto-allocate.md`, `switch.md`, `data/*.k` | | Helpers; `ListBytes` is a sorted wrapper over the built-in `LIST` |

Layering is `WASM → ELROND-NODE → ELROND-CONFIG → ELROND (+VMHOOKS, ASYNC) → MANDOS → KASMER`.

Test hooks implemented in `kasmer.md`: `createAccount`, `registerNewAddress`, `deployContract`,
`getStorage`, `setStorage`, `setExternalBalance`, `setESDTExternalBalance`, `setESDTRole`,
`checkESDTRole`, `setBlockTimestamp`, `assertBool`, `assumeBool`, `startPrank`, `stopPrank`.
Most are restricted to the test contract, which lives at the fixed address `sc:k-test`.

- `assertBool(false)` throws `ExecutionFailed "assertion failed"`.
- `assumeBool(false)` fires `endFoundryImmediately`, which clears the call stack and ends the
  test with exit code 0, so the path counts as vacuously passing.
- `startPrank` swaps `<callee>` to the given address until `stopPrank`.
- `deployContract` takes a *file path* and looks the module up in `<wasmStore>`, which the tool
  fills in advance with the parsed modules of the contracts listed in `kasmer.json`.

Other pieces in this repo:

- **ESDT system SC** (`src/esdt-system-sc`): a Rust contract compiled to Wasm, used as the model of
  the system contract.
- **Runtime JSON** (`kdist/runtime/llvm-{mandos,kasmer}.json`, about 430 KB each): the initial
  configuration with the ESDT system SC already deployed, serialised as KAST JSON and checked in.
  Regenerated by the `runtime` command. Every run starts from this file.
- **`src/testapi`**: the old Rust crate with the `extern "C"` test hooks, pinned to
  `multiversx-sc 0.50.1`. Superseded by the framework (section 3.4).
- **kdist targets**: `source`, `plugin` (builds `krypto.a`), `llvm-mandos`, `llvm-kasmer`,
  `haskell-mandos`, `haskell-kasmer`.

### 3.2 K semantics in `kasmer-multiversx`

Path: `kmxwasm/src/kmxwasm/kdist/mxwasm-semantics/`. About 7,500 lines, almost all lemmas.

- `mx-wasm.md`: main module `MX-WASM` = `KASMER` + `KWASM-LEMMAS` + `MAP-KORE-SYMBOLIC` + the
  local lemma modules.
- `lemmas/`: about 750 `simplification` rules. Integer inequalities and normalisation, integer
  encoding and length, bytes, and the sparse-bytes memory model. Two files are specific to the
  case studies (`pair-specific-lemmas.md`, `coindrip-specific-lemmas.md`).
- `ceils.k`: definedness rules. `proof-extensions.md`: `infiniteLoop` and a helper predicate.
- Part of the lemmas is generated and machine-checked by `kmxwasm/lemmas/*.py`
  (`make lemmas`, `test-lemmas`, `test-proofs`).
- **kdist targets**: `llvm`, `llvm-library` (C shared library for the booster), `haskell`,
  `lemma-proofs`, `lemma-tests`.

The lemmas are what make symbolic execution terminate in practice. They are the product of the
case studies and are tied to the exact shape of terms the backend produces, so they are the part
most likely to need rework after a K upgrade.

### 3.3 Python

**`kmultiversx`** (mx-semantics, about 1,550 lines)

| File | Lines | Role | Used by kasmer |
|---|---|---|---|
| `scenario.py` | 846 | `mandos` CLI: parses `.scen.json`, turns each step into K terms, runs them on the LLVM interpreter | Helpers only (address and argument encoding, `KMap`, `ListBytes`) |
| `kasmer.py` | 441 | `deploy_test`, `get_test_endpoints`, `run_concrete` (fuzz), `generate_claims` | Yes, core |
| `utils.py` | 85 | `load_wasm` (calls `pykwasm.wasm2kast`), `krun_config`, runtime JSON loading | Yes |
| `runtime.py` | 75 | Regenerates the runtime JSON | Build time |
| `kelrond.py` | 106 | Runs `.wast` unit tests | No |

**`kmxwasm`** (kasmer-multiversx, about 5,750 lines)

| Files | Lines | Role | Used by kasmer |
|---|---|---|---|
| `kasmer/*.py` | 390 | CLI and project loading (`kasmer.json`) | Yes |
| `property.py` | 669 | `kmxwasm-property` CLI: run a claim, restart, show tree or node, bisect, profile | Yes (`RunClaim`) |
| `property_testing/*.py` | 850 | The proof loop, cell abstraction, concrete Wasm initialisation | Yes, core |
| `tools.py`, `build.py` | 220 | Starts `kore-rpc` or `kore-rpc-booster`, wraps `krun`, pretty-printing tweaks | Yes |
| `ast/*.py`, `json.py` | 710 | Path-based reading and rewriting of configuration cells | Yes |
| `proofs.py`, `functions.py`, `execution.py`, `kast.py`, `rules.py`, `specs.py`, `identifiers.py`, `lazy_explorer.py`, `wasm_types.py` | about 2,300 | An older, different approach: summarise each Wasm function into a rule. References a `k-src` directory that no longer exists | **No. Dead code** |
| `lemmas/*.py` | 700 | Lemma generation and proving | Dev time only |

Everything goes through `pyk`, RV's Python library for K, which supplies the KAST and KORE data
types, KAST⇄KORE conversion, the KORE JSON-RPC client, the `KCFG` proof graph and its on-disk
format, `kdist` (build orchestration), and wrappers for `kompile`, `krun`, `kprove`.

### 3.4 Framework side in `mx-sdk-rs`

- `framework/base/src/api/test_api.rs`: `TestApi` / `TestApiImpl` traits, part of `VMApi`.
- `framework/wasm-adapter/src/api/test_api_node.rs`: the `extern "C"` imports from `env`.
- `framework/base/.../test_raw_wrapper.rs`: `self.test_raw()` with `create_account`,
  `register_new_address`, `assume`, `assert`, `set_storage`, …
- `tx_exec_test_call.rs`: `.test_deploy()` and `.test_call()` on the unified tx syntax, which wrap
  a prank around a deploy or call.
- `framework/meta-lib/src/ei/test_api_hook_signature_list.rs`: hand-written signatures so
  `sc-meta` can validate imports.
- `contracts/parametric-tests/`: `adder-pt`, `multisig-pt`, `testapi-pt`, and helper contracts.

A test contract is an ordinary contract. `init(code_path)` sets up state and deploys the contract
under test. Every endpoint named `test_*` is a parametric test whose arguments are the parameters.

## 4. How it runs today

### 4.1 One-time build

1. `poetry install` pulls `kmultiversx`, `pykwasm` and `pyk` at the pinned versions.
2. `kdist build mx-semantics.llvm-kasmer` copies the K sources of wasm-semantics, the plugin and
   mx-semantics side by side, builds `krypto.a` with `make` (C++, needs OpenSSL, secp256k1 and the
   plugin's own submodules), then runs `kompile --backend llvm`. Output: a native `interpreter`.
3. `kdist build mxwasm-semantics.{haskell,llvm,llvm-library}` produces `definition.kore` for the
   Haskell backend, a second interpreter, and a shared library for the booster.

`kompile` needs 8 to 12 GB of JVM heap (`K_OPTS=-Xmx12288m`).

### 4.2 `kasmer build`

1. `sc-meta all build --wasm-symbols --no-wasm-opt` on the test contract and on each contract in
   `kasmer.json`.
2. Parse every `.wasm` into a KAST `ModuleDecl` with `pykwasm.wasm2kast`.
3. Take the runtime JSON configuration, put the modules in `<wasmStore>`, and put these steps in
   `<k>`: create account `address:k`, register `sc:k-test` as its next address, `deployTx` the test
   contract with the contract paths as arguments, check status OK.
4. Run that concretely on the **LLVM interpreter**. This executes the test contract's `init`,
   including the nested deploys. The result is the post-setup state.
5. Read the test contract's ABI. For each `test_*` endpoint write
   `generated_claims/<test>-spec.json`, a reachability claim:
   - **Left side**: the post-setup state with `<k>` = `callTx(k, k-test, 0, <test>, [ARG_0, …])`
     followed by `checkExpectStatus(OK)`. Each `ARG_i` is a symbolic `Int` encoded as big-endian
     bytes, constrained by its ABI type.
   - **Right side**: `<k>` empty, exit code 0, call stack empty. Accounts, logs, block info and
     VM output are left unconstrained.

   The claims are large: 0.76 MB for adder, 7.6 MB for the DEX pair, because they embed the full
   AST of every deployed contract.

Only `BigUint`, `u32` and `u64` arguments are supported. Anything else raises `TypeError`.

### 4.3 `kasmer fuzz`

Same setup as steps 1 to 4, then for each test endpoint `pyk`'s `KFuzz` uses the `hypothesis`
library to generate argument tuples, substitutes a `callTx` into the configuration and runs the
LLVM interpreter, checking the process exit code. No Haskell backend, no Z3.

### 4.4 `kasmer verify <test> [--booster]`

Proves one claim by all-path symbolic execution. `property_testing/running.py: run_claim`:

1. Start a KORE-RPC server on the Haskell definition: `kore-rpc`, or with `--booster`
   `kore-rpc-booster` plus the LLVM shared library. Build a `KCFG` with an init node and a target
   node from the claim.
2. Loop over open leaves:
   - **Wasm instantiation** (`newWasmInstance` is next): do not run it symbolically. Strip the
     configuration down, run the instantiation concretely with the LLVM interpreter (`krun`), and
     splice the resulting `<wasm>` cell back in. Cached per contract address.
   - **Otherwise**: replace three bulky cells with fresh variables (`<callStack>`,
     `<interimStates>`, every `<code>`), send an `execute` request for up to `--step` rewrite
     steps (default 10,000) with a list of cut-point rules, then put the original contents back.
     The cut points are exactly the rules that read or write those cells; when one is next, a
     single step is executed without abstraction. This keeps megabytes of contract AST out of
     every RPC round trip and out of the backend's matching.
   - After each step, check new leaves against the target, first with a cheap syntactic filter,
     then with an RPC `implies`. A match becomes a cover edge.
3. Result: **Success** (all leaves covered), **Stuck** (a leaf can neither step nor reach the
   target; this is how a failing `assert` shows up), or **RunException**.
4. The graph is saved under `.property/<test>` relative to the current directory, so runs can be
   resumed (`--restart`) and inspected with `kmxwasm-property --tree / --show-node / --bisect-after
   / --profile`.

Booster flags used: `--index-cells instrs,commands,k --smt-timeout 1000
--equation-max-iterations 1000 --equation-max-recursion 100 --interim-simplification 2000
--no-fallback-simplify --no-post-exec-simplify`.

What CI exercised: `test_call_add` and `test_fund` on every pull request. A weekly `run-proofs`
job ran pair `add_liquidity` (job timeout 1,000 minutes), pair `swap` and multisig
`change_quorum` (300 minutes each). Those timeouts indicate that proofs of real contracts take
hours, not minutes. Coindrip and `exchange_k` claims exist as test data and `test_runners/*.sh`
scripts but were not in a scheduled job.

## 5. Dependency inventory

| Dependency | Language | Pinned | Latest | Needed for | Can it go? |
|---|---|---|---|---|---|
| K frontend `kompile` | Java 17 | 7.1.85 (kasmer), 7.1.140 (mx-semantics) | 7.1.337 | Compiling `.k`/`.md` into `definition.kore`, interpreter, shared library | Not at build time. Could be confined to CI that publishes prebuilt artifacts |
| Haskell backend (`kore-rpc`, `kore-rpc-booster`) | Haskell | with K | with K | Symbolic execution, `implies`, simplification | No, this is the engine |
| LLVM backend | C++ / LLVM ≥ 15 | with K | with K | Concrete interpreter (setup, fuzzing, Wasm instantiation) and booster library | No. A Rust concrete executor would be a second, unverified model |
| Z3 | C++ | 4.12.1 recommended by K | | SMT queries from the Haskell backend | No |
| `pyk` | Python | with K | with K | All glue | Yes, by reimplementing the subset in section 6 |
| wasm-semantics | K | v0.1.95 (kasmer lockfile), v0.1.106 | v0.1.157 | Wasm model | No |
| `pykwasm` | Python | same | same | `.wasm` → KAST (via an old `py-wasm` fork; upstream replaced it with its own parser in July 2026, still Python) | Yes, `wasmparser` in Rust |
| blockchain-k-plugin | K + C++ | commit `c9264b2` | active | `KRYPTO` hooks: only `Sha256` and `Keccak256` are used | Likely reducible; see open questions |
| `hypothesis` | Python | 6.x | | Fuzz input generation | Yes, `proptest` or similar |
| Poetry, Python 3.10 | | | | Packaging | Yes |
| Nix, `kup`, Docker | | | | The only supported ways to install K; CI runs in an Ubuntu Jammy image | Replace with our own distribution of the binaries |
| `wabt` (`wasm2wat`) | | | | Debug `.wat` dumps only | Yes |
| `sc-meta`, Rust | | 0.50 | 0.66 | Building contracts | Already ours |
| GNU make, cmake, autoconf, libtool | | | | Building `krypto.a` | Goes with the plugin |

## 6. What a Rust replacement of the Python has to do

These are the capabilities on the live path, in rough order of effort:

1. **Term model and serialisation.** Today the glue works on KAST (the frontend's JSON AST) and
   converts to KORE through `pyk`, which needs the compiled definition's metadata to insert sort
   injections. The backends speak KORE only: text for the LLVM interpreter, JSON for RPC. A Rust
   tool should work in KORE directly and skip KAST. That removes the largest chunk of `pyk`
   but means the checked-in runtime JSON and the claim format change.
2. **Configuration editing.** Find and replace cells by path in a large term
   (`ast/mx.py`, `ast/generic.py`, `cell_abstracter.py`).
3. **Wasm binary → term.** `wasmparser` to the `ModuleDecl` constructors of wasm-semantics.
4. **Scenario value encoding.** `address:`, `sc:`, numeric and bytes formats. Already exists in
   `mx-sdk-rs` (`multiversx-chain-scenario-format`).
5. **KORE JSON-RPC client.** `execute`, `implies`, `simplify`, `add-module`, `get-model` over
   JSON-RPC 2.0 on a TCP socket. Small.
6. **Proof graph.** Nodes, edges, splits, covers, persistence, resume. `pyk`'s `KCFG` is general;
   kasmer uses a narrow slice.
7. **The proof loop** of section 4.4, including abstraction, cut points and the concrete Wasm
   initialisation shortcut.
8. **Process management.** Spawn the interpreter and RPC server, pick ports, clean up.
9. **Output.** Pretty-printing configurations is done by `pyk` using the definition's syntax.
   Reimplementing a general unparser is significant work; a purpose-built report (failing branch,
   path condition, counterexample via `get-model`) is cheaper and more useful to contract authors.
10. **Fuzzing driver.** Generate arguments, run the interpreter, shrink.

Not needed: the function-summarisation code, the `mandos` scenario runner (unless we want it as a
conformance check of the K model against the Rust and Go VMs), and the lemma generators (needed
only when maintaining lemmas).

## 7. State of health and risks

**Staleness**

- Last real commits: kasmer-multiversx July 2024, mx-semantics October 2024. Later activity is
  automated dependency bumps and branch pushes.
- The two repos are out of step with each other: kasmer pins mx-semantics v0.1.107 while
  mx-semantics is at v0.1.119.
- K has moved from 7.1.85 to 7.1.337; wasm-semantics from v0.1.95/106 to v0.1.157. Expect changes
  in attributes, hooks, `pyk` APIs and booster behaviour. The semantics will probably need
  mechanical fixes. The lemmas may need real work, since proofs can stop going through when the
  backend simplifies terms differently.
- Test contracts in the RV repos target `multiversx-sc 0.50`; the framework is at 0.66 and the VM
  has gained hooks since. Any host function a current contract imports that the K model lacks
  makes execution get stuck. The gap has not been measured yet.

**Portability**

- K publishes binaries only as Ubuntu `.deb` packages. On macOS the routes are Nix/`kup` (the
  flakes list `aarch64-darwin`, and there is an `APPLE_SILICON` switch in the build) or a source
  build with Homebrew. The K README is self-contradictory about Apple Silicon support.
- On this Mac: Z3 4.15.4, `wasm2wat`, Poetry, Docker and `sc-meta 0.66.3` are present. There is no
  K, no Nix, no Java runtime, no Haskell toolchain. Python is 3.12; the projects ask for `^3.10`.
- `property_testing/paths.py` calls `git rev-parse` at import time, so the tool only runs inside a
  git checkout (the Nix build patches this out with `sed`).

**Functional gaps**

- Argument types limited to `BigUint`, `u32`, `u64`.
- The framework declares 12 test hooks; the K model has 14. `setESDTRole` and `checkESDTRole` are
  missing from `test_api_node.rs` and the signature list.
- `adder-pt/sc-config.toml` uses `ei = "ignore"` with a TODO for a proper parametric-test EI option.
- `multisig-pt` and `testapi-pt` still carry a legacy `foundry.json` in the old `contract_paths`
  format.
- Loops over symbolic bounds need hand-written invariants; there is no user-facing way to supply
  them in the `kasmer` flow.
- Two lemma files are specific to particular contracts, a sign that new contracts will tend to
  need new lemmas.
- There appears to be no gas metering: the gas limit is carried in the VM input and
  `gasLimit × gasPrice` is deducted from the sender, but no rule was found that consumes gas
  during execution. Out-of-gas behaviour is therefore outside what can be tested.

**Open questions to settle on first build**

- kmxwasm's own LLVM targets pass no `krypto.a` link flags, unlike the mx-semantics ones. Whether
  they link without the plugin, and whether the Haskell backend's built-in `KRYPTO` hooks are
  enough to drop the C++ plugin entirely, needs checking.
- Whether the checked-in runtime JSON still loads against a current K, or must be regenerated.

## 8. Implications for the plan

The realistic end state has this shape:

- **Kept as is**: the K sources (semantics and lemmas), vendored into one place with pinned
  versions of wasm-semantics, instead of two repos plus submodules plus git-tag dependencies.
- **Built in CI, shipped as artifacts** per platform: `definition.kore`, the LLVM interpreter, the
  booster shared library, and the `kore-rpc-booster` binary. Users then need no Java, Haskell,
  Nix or Python, only the artifacts and Z3. `sc-meta install`-style download fits our ecosystem.
- **New Rust crate** replacing `kmxwasm` + the used parts of `kmultiversx` + the used subset of
  `pyk`, per section 6. Natural home is a `sc-meta` subcommand or sibling tool.

A sensible order of work, each step being verifiable on its own:

1. Reproduce the current stack unchanged in Docker (Ubuntu Jammy, K 7.1.85) and get
   `test_call_add` to pass. This gives a known-good baseline to compare against.
2. Upgrade K and wasm-semantics while still on the Python tooling, so regressions in the semantics
   and lemmas are separated from regressions introduced by a rewrite.
3. Measure the host-function gap against contracts built with the current framework.
4. Write the Rust driver against the upgraded, working definition, starting with `build` and
   `fuzz` (LLVM only), then `verify`.
5. Settle distribution, including native macOS.

Doing the upgrade (2) before the rewrite (4) is the main recommendation: it is the step with the
most unknowns, and the existing Python test suite is the only safety net for it.
