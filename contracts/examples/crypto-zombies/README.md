# Crypto Zombies Contract

A MultiversX smart contract adapted from the [CryptoZombies](https://cryptozombies.io) tutorial. Demonstrates multi-module contract composition, NFT-style ownership, combat mechanics, and ESDT kitty feeding — making it a practical reference for contracts split across multiple trait modules.

It is also available as an `sc-meta` template (see "Using as a template" below).

## Contract modules

| Module | Description |
|---|---|
| `ZombieFactory` | Create zombies with random DNA; one zombie per address |
| `ZombieFeeding` | Feed a zombie a CryptoKitty to level it up and mutate its DNA |
| `ZombieHelper` | Level-up fee collection; change zombie name/DNA at higher levels |
| `ZombieAttack` | Attack other zombies; win/loss tracking with cooldown |
| `Storage` | All storage mappers shared across modules |

## Project structure

```
crypto-zombies/
├── src/
│   ├── lib.rs                    # Contract entry point
│   ├── zombie.rs                 # Zombie struct
│   ├── zombie_factory.rs
│   ├── zombie_feeding.rs
│   ├── zombie_helper.rs
│   ├── zombie_attack.rs
│   ├── storage.rs
│   ├── kitty_obj.rs              # CryptoKitty struct (for ESDT callback)
│   ├── kitty_ownership_proxy.rs  # Proxy for the kitty ownership contract
│   └── proxy.rs                  # Auto-generated proxy (sc-meta all proxy)
├── tests/
│   └── crypto_zombies_blackbox_test.rs
├── meta/                         # Build metadata
├── wasm/                         # Wasm build output
├── Cargo.toml
└── multiversx.json
```

## Building

```bash
sc-meta all build
```

## Testing

```bash
cargo test
```

## Using as a template

```bash
sc-meta new --template crypto-zombies --name my-contract
```
