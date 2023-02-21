use clap::{Parser, Subcommand};

/// TODO: Add docs
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCliArgs {
    #[command(subcommand)]
    pub command: Option<InteractCliAction>,
}

/// TODO: Add docs
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliAction {
    #[command(name = "board", about = "placeholder text for board")]
    Board,
    #[command(name = "deploy", about = "placeholder text for deploy")]
    Deploy,
    #[command(name = "dns-register", about = "placeholder text for dns-register")]
    DnsRegister,
    #[command(name = "feed", about = "placeholder text for feed")]
    Feed,
    #[command(name = "nft-full", about = "placeholder text for nft-full")]
    NftFull,
    #[command(name = "nft-issue", about = "placeholder text for nft-issue")]
    NftIssue,
    #[command(name = "nft-items", about = "placeholder text for nft-items")]
    NftItems,
    #[command(name = "nft-special", about = "placeholder text for nft-special")]
    NftSpecial,
    #[command(name = "quorum", about = "placeholder text for quorum")]
    Quorum,
}
