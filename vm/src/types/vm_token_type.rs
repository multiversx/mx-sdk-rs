#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMTokenType {
    Fungible,
    SemiFungible,
    Meta,
    NonFungible,
}

impl VMTokenType {
    pub fn from_system_sc_arg(raw: &[u8]) -> Self {
        match raw {
            b"FNG" => VMTokenType::Fungible,
            b"SFT" => VMTokenType::SemiFungible,
            b"META" => VMTokenType::Meta,
            b"NFT" => VMTokenType::NonFungible,
            _ => panic!("invalid token type"),
        }
    }
}
