use clap::{Args, Parser, Subcommand};
use multiversx_sc_snippets::imports::RustBigUint;

/// Ping Pong Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Ping Pong Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract.")]
    Deploy(DeployArgs),
    #[command(name = "upgrade", about = "Upgrade contract.")]
    Upgrade(DeployArgs),
    #[command(
        name = "ping",
        about = "User sends some EGLD to be locked in the contract for a period of time."
    )]
    Ping(PingArgs),
    #[command(name = "pong", about = "User can take back funds from the contract.")]
    Pong,
    #[command(name = "pong-all", about = "Send back funds to all users who pinged.")]
    PongAll,
    #[command(
        name = "user-addresses",
        about = "Lists the addresses of all users that have `ping`-ed in the order they have `ping`-ed."
    )]
    GetUserAddresses,
    #[command(name = "contract-state", about = "Returns the current contract state.")]
    GetContractState,
    #[command(name = "ping-amount", about = "Returns the ping amount.")]
    GetPingAmount,
    #[command(name = "deadline", about = "Return deadline.")]
    GetDeadline,
    #[command(
        name = "activation-timestamp",
        about = "Block timestamp of the block where the contract got activated. If not specified in the constructor it is the the deploy block timestamp."
    )]
    GetActivationTimestamp,
    #[command(name = "max-funds", about = "Optional funding cap.")]
    GetMaxFunds,
    #[command(name = "user-status", about = "State of user funds.")]
    GetUserStatus(UserStatusArgs),
    #[command(
        name = "pong-all-last-user",
        about = "`pongAll` status, the last user to be processed. 0 if never called `pongAll` or `pongAll` completed."
    )]
    PongAllLastUser,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct DeployArgs {
    #[arg(short = 'p', long = "ping-amount")]
    pub ping_amount: RustBigUint,

    #[arg(short = 'd', long = "duration-in-seconds")]
    pub duration_in_seconds: u64,

    #[arg(short = 'a', long = "activation-timestamp")]
    pub opt_activation_timestamp: Option<u64>,

    #[arg(short = 'm', long = "max-funds")]
    pub max_funds: Option<RustBigUint>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct PingArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: Option<u64>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct UserStatusArgs {
    #[arg(short = 'i')]
    pub id: usize,
}
