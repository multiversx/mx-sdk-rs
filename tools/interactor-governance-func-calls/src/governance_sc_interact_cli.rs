use clap::{Args, Parser, Subcommand};

/// GovernanceFuncCalls Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// GovernanceFuncCalls Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "propose", about = "Propose")]
    Propose(ProposeArgs),

    #[command(name = "view-config", about = "View config")]
    ViewConfig,

    #[command(name = "view-proposal", about = "View proposal")]
    ViewProposal(ViewProposalArgs),

    #[command(name = "vote", about = "Vote")]
    Vote(VoteArgs),

    #[command(name = "delegate-vote", about = "Delegate vote")]
    DelegateVote(DelegateVoteArgs),

    #[command(name = "stake", about = "Stake via Validator SC")]
    Stake(StakeArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ProposeArgs {
    /// Sender address in Bech32 format
    #[arg(short = 'f')]
    pub from: String,

    /// Git commit hash
    #[arg(short = 'c', long = "commit-hash")]
    pub commit_hash: String,

    /// Start vote epoch
    #[arg(short = 's', long = "start-epoch")]
    pub start_vote_epoch: usize,

    /// End vote epoch
    #[arg(short = 'e', long = "end-epoch")]
    pub end_vote_epoch: usize,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ViewProposalArgs {
    /// Proposal nonce
    #[arg(short = 'n')]
    pub nonce: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct VoteArgs {
    /// Sender address in Bech32 format
    #[arg(short = 'f')]
    pub from: String,

    /// Proposal nonce
    #[arg(short = 'n')]
    pub nonce: usize,

    /// Vote can be "yes", "no", "abstain" or "veto"
    #[arg(short = 'v')]
    pub vote: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegateVoteArgs {
    /// Sender address in Bech32 format
    #[arg(short = 'f')]
    pub from: String,

    /// Proposal nonce
    #[arg(short = 'n')]
    pub nonce: u64,

    /// Vote can be "yes", "no", "abstain" or "veto"
    #[arg(long = "vote")]
    pub vote: String,

    /// Voter address in Bech32 format
    #[arg(long = "voter")]
    pub voter: String,

    /// Stake in EGLD
    #[arg(short = 'v')]
    pub stake: u64,

    /// Optional error message
    #[arg(short = 'e')]
    pub error: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct StakeArgs {
    /// Sender address in Bech32 format
    #[arg(short = 'f')]
    pub from: String,

    /// Maximum number of staked nodes
    #[arg(short = 'm', long = "max-nodes")]
    pub maximum_staked_nodes: usize,

    /// BLS key of the node to stake
    #[arg(short = 'k', long = "bls-key")]
    pub bls_key: String,

    /// Bls signature of the node
    #[arg(short = 's', long = "bls-signature")]
    pub bls_signature: String,

    /// Amount to stake in EGLD
    #[arg(short = 'a')]
    pub amount: u128,
}
