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
