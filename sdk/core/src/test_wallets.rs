use crate::wallet::Wallet;

fn test_wallet(pem_file_contents: &str) -> Wallet {
    Wallet::from_pem_file_contents(pem_file_contents.to_string()).unwrap()
}

/// Test wallet. Do not use on mainnet.
pub fn alice() -> Wallet {
    test_wallet(include_str!("test_wallets/alice.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn bob() -> Wallet {
    test_wallet(include_str!("test_wallets/bob.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn carol() -> Wallet {
    test_wallet(include_str!("test_wallets/carol.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn dan() -> Wallet {
    test_wallet(include_str!("test_wallets/dan.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn eve() -> Wallet {
    test_wallet(include_str!("test_wallets/eve.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn frank() -> Wallet {
    test_wallet(include_str!("test_wallets/frank.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn grace() -> Wallet {
    test_wallet(include_str!("test_wallets/grace.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn heidi() -> Wallet {
    test_wallet(include_str!("test_wallets/heidi.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn ivan() -> Wallet {
    test_wallet(include_str!("test_wallets/ivan.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn judy() -> Wallet {
    test_wallet(include_str!("test_wallets/judy.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn mallory() -> Wallet {
    test_wallet(include_str!("test_wallets/mallory.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn mike() -> Wallet {
    test_wallet(include_str!("test_wallets/mike.pem"))
}

/// Test wallet. Do not use on mainnet.
///
/// Sophie's wallet will always be in shard 0.
///
/// Address: 0x14af28ce7d79117f689228b1af89d16e8b0c16a3d36062a2b6eeb8fbab6c0000
pub fn sophie() -> Wallet {
    test_wallet(include_str!("test_wallets/s0phie.pem"))
}

/// Test wallet. Do not use on mainnet.
///
/// Simon's wallet will always be in shard 1.
///
/// Address: 0x4b9ab2524a7d15416fb78d4d88249dc30272bd6ee1b8a07d4342c33a40a00001
pub fn simon() -> Wallet {
    test_wallet(include_str!("test_wallets/s1mon.pem"))
}

/// Test wallet. Do not use on mainnet.
///
/// Szonja's wallet will always be in shard 2.
///
/// Address: 0x5ea3f378aaaa9f51cef76093b62e1041c90b415016dfa49760d7a846a8d90002
pub fn szonja() -> Wallet {
    test_wallet(include_str!("test_wallets/s2onja.pem"))
}

/// Test wallets. Do not use on mainnet.
///
/// Yields a wallet for the given shard id. Only shard ids 0, 1, and 2 are supported.
pub fn for_shard(shard_id: u32) -> Wallet {
    match shard_id {
        0 => sophie(),
        1 => simon(),
        2 => szonja(),
        _ => panic!("No test wallet for shard id {shard_id}"),
    }
}
