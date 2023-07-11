use clap::{Args, Parser, Subcommand};

/// Multisig Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Multisig Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "board", about = "Print board")]
    Board,
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "dns-register", about = "Register DNS")]
    DnsRegister(DnsRegisterArgs),
    #[command(name = "feed", about = "Feed contract EGLD")]
    Feed,
    #[command(name = "multi-deploy", about = "Multiple deploy contracts")]
    MultiDeploy(MultiDeployArgs),
    #[command(
        name = "nft-full-all-roles",
        about = "Issue multisig and collection with all roles"
    )]
    NftFullAllRoles,
    #[command(name = "nft-full", about = "Issue multisig and collection")]
    NftFull,
    #[command(
        name = "nft-issue-all-roles",
        about = "Issue collection with all roles"
    )]
    NftIssueAllRoles,
    #[command(name = "nft-issue", about = "Issue collection")]
    NftIssue,
    #[command(name = "nft-items", about = "Create items")]
    NftItems,
    #[command(name = "nft-special", about = "Set special role")]
    NftSpecial,
    #[command(name = "quorum", about = "Print quorum")]
    Quorum,
    #[command(name = "unwrap-egld", about = "Unwrap EGLD")]
    UnwrapEgld,
    #[command(
        name = "wegld-swap-full",
        about = "Deploy and swap WEGLD with multisig"
    )]
    WEgldSwapFull,
    #[command(name = "wrap-egld", about = "Wrap EGLD")]
    WrapEgld,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct DnsRegisterArgs {
    /// The name used for the registration (herotag)
    #[arg(short = 'n', long = "name", verbatim_doc_comment)]
    pub name: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct MultiDeployArgs {
    /// The number of contracts to deploy
    #[arg(short = 'c', long = "count", verbatim_doc_comment)]
    pub count: u8,
}
