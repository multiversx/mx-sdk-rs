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
const SOCRATES_PEM: &str = include_str!("test_wallets/s0crates.pem");
const SOPHOCLES_PEM: &str = include_str!("test_wallets/s0phocles.pem");
const SOPHIE_PEM: &str = include_str!("test_wallets/s0phie.pem");
const SILVIA_PEM: &str = include_str!("test_wallets/s1lvia.pem");
const SIOBHAN_PEM: &str = include_str!("test_wallets/s1obhan.pem");
const SIMON_PEM: &str = include_str!("test_wallets/s1mon.pem");
const SZABOLCS_PEM: &str = include_str!("test_wallets/s2abolcs.pem");
const SZILARD_PEM: &str = include_str!("test_wallets/s2ilard.pem");
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
    ("s0crates", SOCRATES_PEM),
    ("socrates", SOCRATES_PEM),
    ("s0phocles", SOPHOCLES_PEM),
    ("sophocles", SOPHOCLES_PEM),
    ("sophie", SOPHIE_PEM),
    ("s0phie", SOPHIE_PEM),
    ("siobhan", SIOBHAN_PEM),
    ("s1obhan", SIOBHAN_PEM),
    ("simon", SIMON_PEM),
    ("s1mon", SIMON_PEM),
    ("szabolcs", SZABOLCS_PEM),
    ("s2abolcs", SZABOLCS_PEM),
    ("szilard", SZILARD_PEM),
    ("s2ilard", SZILARD_PEM),
    ("szonja", SZONJA_PEM),
    ("s2onja", SZONJA_PEM),
];

/// Test wallet. Do not use on mainnet.
pub fn alice() -> Wallet {
    Wallet::new_test_wallet("alice", ALICE_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn bob() -> Wallet {
    Wallet::new_test_wallet("bob", BOB_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn carol() -> Wallet {
    Wallet::new_test_wallet("carol", CAROL_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn dan() -> Wallet {
    Wallet::new_test_wallet("dan", DAN_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn eve() -> Wallet {
    Wallet::new_test_wallet("eve", EVE_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn frank() -> Wallet {
    Wallet::new_test_wallet("frank", FRANK_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn grace() -> Wallet {
    Wallet::new_test_wallet("grace", GRACE_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn heidi() -> Wallet {
    Wallet::new_test_wallet("heidi", HEIDI_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn ivan() -> Wallet {
    Wallet::new_test_wallet("ivan", IVAN_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn judy() -> Wallet {
    Wallet::new_test_wallet("judy", JUDY_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn mallory() -> Wallet {
    Wallet::new_test_wallet("mallory", MALLORY_PEM)
}

/// Test wallet. Do not use on mainnet.
pub fn mike() -> Wallet {
    Wallet::new_test_wallet("mike", MIKE_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// S0crates' wallet will always be in shard 0.
///
/// Address: 0x3d55ffd949781d5f0b5eaf57a3f0797d1db2d76a759ae6df7e335302b7d90000
pub fn socrates() -> Wallet {
    Wallet::new_test_wallet("s0crates", SOCRATES_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// S0phocles' wallet will always be in shard 0.
///
/// Address: 0x35e2358aa3191bcedf6eddf6e14f138765b38a704647ed691130703852620000
pub fn sophocles() -> Wallet {
    Wallet::new_test_wallet("s0phocles", SOPHOCLES_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Sophie's wallet will always be in shard 0.
///
/// Address: 0x14af28ce7d79117f689228b1af89d16e8b0c16a3d36062a2b6eeb8fbab6c0000
pub fn sophie() -> Wallet {
    Wallet::new_test_wallet("sophie", SOPHIE_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// S1obhan's wallet will always be in shard 1.
///
/// Address: 0xe624b62f5dcad961ceaf9ce23e56db72377ea8a8dcc7065b73089778522d0001
pub fn siobhan() -> Wallet {
    Wallet::new_test_wallet("s1obhan", SIOBHAN_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Silvia's wallet will always be in shard 1.
///
/// Address: 0x7bbaef4fae6aa454929e0038bf01da4907e7814609db6e46c9990b5ae9d30001
pub fn silvia() -> Wallet {
    Wallet::new_test_wallet("silvia", SILVIA_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Simon's wallet will always be in shard 1.
///
/// Address: 0x4b9ab2524a7d15416fb78d4d88249dc30272bd6ee1b8a07d4342c33a40a00001
pub fn simon() -> Wallet {
    Wallet::new_test_wallet("simon", SIMON_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Szabolcs' wallet will always be in shard 2.
///
/// Address: 0x99337fe8455f5798fc548037c1ceea4d95d8f89ca468663877719f7d31eb0002
pub fn szabolcs() -> Wallet {
    Wallet::new_test_wallet("szabolcs", SZABOLCS_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// S2ilard's wallet will always be in shard 2.
///
/// Address: 0xf7a7c49bb4d2f63fd82ca0859f2e01c13f320a79c0962dfdc43fcb621cde0002
pub fn szilard() -> Wallet {
    Wallet::new_test_wallet("s2ilard", SZILARD_PEM)
}

/// Test wallet. Do not use on mainnet.
///
/// Szonja's wallet will always be in shard 2.
///
/// Address: 0x5ea3f378aaaa9f51cef76093b62e1041c90b415016dfa49760d7a846a8d90002
pub fn szonja() -> Wallet {
    Wallet::new_test_wallet("szonja", SZONJA_PEM)
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

pub fn valid_names() -> Vec<&'static str> {
    WALLETS.iter().map(|(n, _)| *n).collect()
}

/// Returns the raw PEM file contents for the named test wallet, or `None` if the name is unknown.
///
/// For the list of valid names see [`valid_names()`].
pub fn pem_contents(name: &str) -> Option<&'static str> {
    WALLETS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, pem)| *pem)
}
