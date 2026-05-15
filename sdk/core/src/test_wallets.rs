use multiversx_chain_core::types::ShardId;

use crate::wallet::Wallet;

const ALICE_PEM: &str = include_str!("test_wallets/alice.pem");
const BOB_PEM: &str = include_str!("test_wallets/bob.pem");
const CAROL_PEM: &str = include_str!("test_wallets/carol.pem");
const DAN_PEM: &str = include_str!("test_wallets/dan.pem");
const EVE_PEM: &str = include_str!("test_wallets/eve.pem");
const FRANK_PEM: &str = include_str!("test_wallets/frank.pem");
const GRACE_PEM: &str = include_str!("test_wallets/grace.pem");
const HEIDI_PEM: &str = include_str!("test_wallets/heidi.pem");
const IVAN_PEM: &str = include_str!("test_wallets/ivan.pem");
const JUDY_PEM: &str = include_str!("test_wallets/judy.pem");
const MALLORY_PEM: &str = include_str!("test_wallets/mallory.pem");
const MIKE_PEM: &str = include_str!("test_wallets/mike.pem");
const SOPHIE_PEM: &str = include_str!("test_wallets/s0phie.pem");
const SIMON_PEM: &str = include_str!("test_wallets/s1mon.pem");
const SZONJA_PEM: &str = include_str!("test_wallets/s2onja.pem");

const WALLETS: &[(&str, &str)] = &[
    ("alice", ALICE_PEM),
    ("bob", BOB_PEM),
    ("carol", CAROL_PEM),
    ("dan", DAN_PEM),
    ("eve", EVE_PEM),
    ("frank", FRANK_PEM),
    ("grace", GRACE_PEM),
    ("heidi", HEIDI_PEM),
    ("ivan", IVAN_PEM),
    ("judy", JUDY_PEM),
    ("mallory", MALLORY_PEM),
    ("mike", MIKE_PEM),
    ("sophie", SOPHIE_PEM),
    ("s0phie", SOPHIE_PEM),
    ("simon", SIMON_PEM),
    ("s1mon", SIMON_PEM),
    ("szonja", SZONJA_PEM),
    ("s2onja", SZONJA_PEM),
];

fn test_wallet(pem_file_contents: &str) -> Wallet {
    Wallet::from_pem_file_contents(pem_file_contents.to_string()).unwrap()
}

/// Test wallet. Do not use on mainnet.
pub fn alice() -> Wallet {
    test_wallet(ALICE_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn bob() -> Wallet {
    test_wallet(BOB_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn carol() -> Wallet {
    test_wallet(CAROL_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn dan() -> Wallet {
    test_wallet(DAN_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn eve() -> Wallet {
    test_wallet(EVE_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn frank() -> Wallet {
    test_wallet(FRANK_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn grace() -> Wallet {
    test_wallet(GRACE_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn heidi() -> Wallet {
    test_wallet(HEIDI_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn ivan() -> Wallet {
    test_wallet(IVAN_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn judy() -> Wallet {
    test_wallet(JUDY_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn mallory() -> Wallet {
    test_wallet(MALLORY_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn mike() -> Wallet {
    test_wallet(MIKE_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Sophie's wallet will always be in shard 0.
///
/// Address: 0x14af28ce7d79117f689228b1af89d16e8b0c16a3d36062a2b6eeb8fbab6c0000
pub fn sophie() -> Wallet {
    test_wallet(SOPHIE_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Simon's wallet will always be in shard 1.
///
/// Address: 0x4b9ab2524a7d15416fb78d4d88249dc30272bd6ee1b8a07d4342c33a40a00001
pub fn simon() -> Wallet {
    test_wallet(SIMON_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Szonja's wallet will always be in shard 2.
///
/// Address: 0x5ea3f378aaaa9f51cef76093b62e1041c90b415016dfa49760d7a846a8d90002
pub fn szonja() -> Wallet {
    test_wallet(SZONJA_PEM)
}

/// Test wallets. Do not use on mainnet.
///
/// Yields a wallet for the given shard id. Only shard ids 0, 1, and 2 are supported.
pub fn for_shard(shard_id: ShardId) -> Wallet {
    let shard_id_num = shard_id.as_u32();
    match shard_id_num {
        0 => sophie(),
        1 => simon(),
        2 => szonja(),
        _ => panic!("No test wallet for shard id {shard_id_num}"),
    }
}

fn valid_names() -> Vec<&'static str> {
    WALLETS.iter().map(|(n, _)| *n).collect()
}

/// Returns the raw PEM file contents for the named test wallet.
///
/// Valid names: `alice`, `bob`, `carol`, `dan`, `eve`, `frank`, `grace`, `heidi`,
/// `ivan`, `judy`, `mallory`, `mike`, `sophie`, `simon`, `szonja`.
///
/// Panics if the name is not one of the above.
pub fn pem_contents(name: &str) -> &'static str {
    WALLETS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, pem)| *pem)
        .unwrap_or_else(|| {
            panic!(
                "Unknown test wallet name: '{name}'. Valid names: {}",
                valid_names().join(", ")
            )
        })
}
