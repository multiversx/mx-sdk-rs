use multiversx_sdk::wallet::Wallet;

fn test_wallet(pem_file_contents: &str) -> Wallet {
    Wallet::from_pem_file_contents(pem_file_contents.to_string()).unwrap()
}

/// Test wallet. Do not use on mainnet.
pub fn alice() -> Wallet {
    test_wallet(include_str!("alice.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn bob() -> Wallet {
    test_wallet(include_str!("bob.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn carol() -> Wallet {
    test_wallet(include_str!("carol.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn dan() -> Wallet {
    test_wallet(include_str!("dan.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn eve() -> Wallet {
    test_wallet(include_str!("eve.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn frank() -> Wallet {
    test_wallet(include_str!("frank.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn grace() -> Wallet {
    test_wallet(include_str!("grace.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn heidi() -> Wallet {
    test_wallet(include_str!("heidi.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn ivan() -> Wallet {
    test_wallet(include_str!("ivan.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn judy() -> Wallet {
    test_wallet(include_str!("judy.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn mallory() -> Wallet {
    test_wallet(include_str!("mallory.pem"))
}

/// Test wallet. Do not use on mainnet.
pub fn mike() -> Wallet {
    test_wallet(include_str!("mike.pem"))
}
