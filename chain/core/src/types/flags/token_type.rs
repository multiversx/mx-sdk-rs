#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Fungible,
    SemiFungible,
    Meta,
    NonFungible,
}

impl TokenType {
    pub fn from_system_sc_arg(raw: &[u8]) -> Self {
        match raw {
            b"FNG" => TokenType::Fungible,
            b"SFT" => TokenType::SemiFungible,
            b"META" => TokenType::Meta,
            b"NFT" => TokenType::NonFungible,
            _ => panic!("invalid token type"),
        }
    }
}
