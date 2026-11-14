use clap::{Args, Subcommand, ValueEnum};
use multiversx_sc_snippets::imports::RustBigUint;

use super::cli_args_tx::GatewayArgs;
use super::cli_args_tx_system::{SystemQueryArgs, SystemTxCommonArgs};

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceCliArgs {
    #[command(subcommand)]
    pub command: GovernanceCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum GovernanceCliAction {
    #[command(about = "Submits a new governance proposal.")]
    Propose(GovernanceProposeArgs),

    #[command(about = "Votes on a proposal, with the full voting power of the sender.")]
    Vote(GovernanceVoteArgs),

    #[command(about = "Closes a proposal, after its voting period has ended.")]
    CloseProposal(GovernanceProposalNonceTxArgs),

    #[command(about = "Clears the ended proposals of the given proposers.")]
    ClearEndedProposals(GovernanceClearEndedProposalsArgs),

    #[command(about = "Claims the accumulated proposal fees. Owner only.")]
    ClaimAccumulatedFees(GovernanceNoArgsTxArgs),

    #[command(about = "Changes the governance configuration. Owner only.")]
    ChangeConfig(GovernanceChangeConfigArgs),

    // --- queries ---
    #[command(about = "Queries the governance configuration (no transaction, no wallet required).")]
    Config(SystemQueryArgs),

    #[command(about = "Queries a proposal (no transaction, no wallet required).")]
    Proposal(GovernanceProposalQueryArgs),

    #[command(
        about = "Queries the voting power of an address (no transaction, no wallet required)."
    )]
    VotingPower(GovernanceVotingPowerArgs),

    #[command(
        about = "Queries the votes cast by a contract on behalf of a user (no transaction, no wallet required)."
    )]
    DelegatedVoteInfo(GovernanceDelegatedVoteInfoArgs),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
pub enum GovernanceVoteArg {
    #[value(name = "yes")]
    Yes,
    #[value(name = "no")]
    No,
    #[value(name = "abstain")]
    Abstain,
    #[value(name = "veto")]
    Veto,
}

impl GovernanceVoteArg {
    pub fn as_str(&self) -> &'static str {
        match self {
            GovernanceVoteArg::Yes => "yes",
            GovernanceVoteArg::No => "no",
            GovernanceVoteArg::Abstain => "abstain",
            GovernanceVoteArg::Veto => "veto",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceProposeArgs {
    /// The git commit hash of the proposed change (40 hex characters).
    #[arg(long)]
    pub commit_hash: String,

    /// The epoch when voting starts.
    #[arg(long)]
    pub start_vote_epoch: u64,

    /// The epoch when voting ends.
    #[arg(long)]
    pub end_vote_epoch: u64,

    /// The proposal fee, in EGLD atto-units. It must be exactly the configured fee.
    /// If omitted, it is read from the governance configuration.
    #[arg(long)]
    pub proposal_fee: Option<RustBigUint>,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceVoteArgs {
    /// The proposal nonce.
    #[arg(long)]
    pub proposal_nonce: u64,

    /// The vote.
    #[arg(long, value_enum)]
    pub vote: GovernanceVoteArg,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceProposalNonceTxArgs {
    /// The proposal nonce.
    #[arg(long)]
    pub proposal_nonce: u64,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceClearEndedProposalsArgs {
    /// Bech32 addresses of the proposers.
    #[arg(long, num_args = 1.., required = true)]
    pub proposers: Vec<String>,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceNoArgsTxArgs {
    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceChangeConfigArgs {
    /// The proposal fee, in EGLD atto-units.
    #[arg(long)]
    pub proposal_fee: RustBigUint,

    /// The part of the proposal fee lost if the proposal is vetoed, in EGLD atto-units.
    #[arg(long)]
    pub lost_proposal_fee: RustBigUint,

    /// Minimum quorum, in basis points (10000 = 100%).
    #[arg(long)]
    pub min_quorum: u32,

    /// Minimum veto threshold, in basis points (10000 = 100%).
    #[arg(long)]
    pub min_veto_threshold: u32,

    /// Minimum pass threshold, in basis points (10000 = 100%).
    #[arg(long)]
    pub min_pass_threshold: u32,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceProposalQueryArgs {
    /// The proposal nonce.
    pub proposal_nonce: u64,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceVotingPowerArgs {
    /// Bech32 address.
    pub address: String,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GovernanceDelegatedVoteInfoArgs {
    /// Bech32 address of the contract voting on behalf of users (e.g. a liquid staking contract).
    #[arg(long)]
    pub contract: String,

    /// Bech32 address of the user.
    #[arg(long)]
    pub user: String,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}
