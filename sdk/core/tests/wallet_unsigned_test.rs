use multiversx_sdk::{
    chain_core::std::{Bech32Address, Bech32Hrp},
    data::transaction::{Transaction, TransactionVersion},
    test_wallets,
    wallet::{Signer, Wallet, WalletSource},
};

const ALICE: &str = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";

fn unsigned_alice() -> Wallet {
    Bech32Address::from_bech32_str(ALICE).into()
}

fn tx_from(sender: &Wallet) -> Transaction {
    Transaction {
        nonce: 7,
        value: "0".to_owned(),
        receiver: sender.to_bech32(),
        sender: sender.to_bech32(),
        gas_price: 1_000_000_000,
        gas_limit: 50_000,
        data: None,
        signature: None,
        chain_id: "D".to_owned(),
        version: TransactionVersion::V2,
        options: None,
        relayer: None,
        relayer_signature: None,
    }
}

#[test]
fn unsigned_wallet_keeps_address_and_hrp() {
    let wallet = unsigned_alice();

    assert_eq!(wallet.signer, Signer::None);
    assert_eq!(
        wallet.source,
        WalletSource::AddressOnly(Bech32Hrp::default())
    );
    assert_eq!(wallet.to_bech32().bech32, ALICE);
    assert_eq!(wallet.to_address(), test_wallets::alice().to_address());
}

#[test]
fn unsigned_wallet_keeps_non_default_hrp() {
    let address = test_wallets::alice().to_address();
    let hrp = Bech32Hrp::parse_unchecked("test");
    let wallet = Wallet::new_unsigned(address, hrp);

    assert!(wallet.to_bech32().bech32.starts_with("test1"));
}

#[test]
fn unsigned_wallet_cannot_sign() {
    let wallet = unsigned_alice();

    assert!(!wallet.can_sign());
    assert!(test_wallets::alice().can_sign());

    assert!(wallet.private_key().is_none());
    assert!(wallet.private_key_hex().is_none());
    assert!(wallet.public_key().is_none());
    assert!(wallet.public_key_hex().is_none());
    assert!(wallet.to_pem(Bech32Hrp::default()).is_err());
}

#[test]
fn unsigned_wallet_signs_tx_with_zeros() {
    let wallet = unsigned_alice();
    let signature = wallet.sign_tx(&tx_from(&wallet)).unwrap();

    assert!(signature.is_zero());
    assert_eq!(signature.to_hex(), "0".repeat(128));
}

#[test]
fn unsigned_wallet_signs_bytes_with_zeros() {
    let wallet = unsigned_alice();

    assert!(wallet.sign_bytes(b"hello multiversx").unwrap().is_zero());
    assert!(wallet.sign_bytes(b"").unwrap().is_zero());
}

#[test]
fn real_signatures_are_not_zero() {
    let alice = test_wallets::alice();

    assert!(!alice.sign_bytes(b"hello multiversx").unwrap().is_zero());
    assert!(!alice.sign_tx(&tx_from(&alice)).unwrap().is_zero());
}
