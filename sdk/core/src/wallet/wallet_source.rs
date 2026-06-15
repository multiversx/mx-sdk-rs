use multiversx_chain_core::std::Bech32Hrp;

/// Optional structure that indicates how the [`Wallet`] was created, with additional metadata.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WalletSource {
    Mnemonic,
    PrivateKey,
    PemFile(Bech32Hrp),
    TestWallet(&'static str),
    Keystore(Bech32Hrp),
}
