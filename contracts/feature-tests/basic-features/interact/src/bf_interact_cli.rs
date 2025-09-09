use clap::{Args, Parser, Subcommand};

/// Basic Features Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Basic Features Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploys basic-features contract")]
    Deploy,
    #[command(
        name = "deploy-storage-bytes",
        about = "Deploys storage-bytes contract variant"
    )]
    DeployStorageBytes,
    #[command(
        name = "large-storage",
        about = "Experiment with large storage on storage-bytes contract variant"
    )]
    LargeStorage(LargeStorageArgs),
    #[command(
        name = "egld-decimals",
        about = "Experiment with returns_egld_decimals on basic-features contract"
    )]
    ReturnsEGLDDecimals(ReturnsEGLDDecimalsArgs),
    #[command(
        name = "echo-mo",
        about = "Experiment with echo_managed_option on basic-features contract"
    )]
    EchoManagedOption(EchoManagedOptionArgs),
    #[command(name = "epoch", about = "Epoch info")]
    EpochInfo,
    #[command(name = "blockt", about = "Epoch info")]
    BlockTimestamps,
    #[command(name = "codehash", about = "Code hash test")]
    CodeHash(CodeHashArgs),
    #[command(name = "token", about = "Token data test")]
    TokenData(TokenDataArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct LargeStorageArgs {
    /// The value to add
    #[arg(long = "kb")]
    pub size_kb: usize,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ReturnsEGLDDecimalsArgs {
    /// The amount of EGLD
    #[arg(short = 'e')]
    pub egld: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct EchoManagedOptionArgs {
    /// The value of ManagedOption
    #[arg(short = 'm')]
    pub managed_option: Option<u64>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodeHashArgs {
    #[arg(short = 'a', long = "address")]
    pub address: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TokenDataArgs {
    #[arg(short = 'a', long = "address")]
    pub address: String,

    #[arg(short = 't', long = "token-id")]
    pub token_id: String,

    #[arg(short = 'n', long = "nonce", default_value = "0")]
    pub nonce: u64,
}
