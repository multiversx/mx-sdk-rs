use clap::{Args, Parser, Subcommand};
use multiversx_sc_snippets::imports::Address;

/// Lottery Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Lottery Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "create_lottery_pool", about = "Create Lottery Pool")]
    CreateLotteryPool(CreateLotteryPollArgs),
    #[command(name = "buy_ticket", about = "Buy Ticket")]
    BuyTicket(LotteryNameArg),
    #[command(name = "determine_winner", about = "Determine Winner")]
    DetermineWinner(LotteryNameArg),
    #[command(name = "claim_rewards", about = "Claim Rewards")]
    ClaimRewards(ClaimRewardsArg),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CreateLotteryPollArgs {
    /// The value to add
    #[arg(short = 'n', long = "name")]
    pub lottery_name: String,
    #[arg(short = 'n', long = "name")]
    pub token_identifier: String,
    #[arg(short = 'n', long = "name")]
    pub ticket_price: u128,
    #[arg(short = 'n', long = "name")]
    pub opt_total_tickets: Option<usize>,
    #[arg(short = 'n', long = "name")]
    pub opt_deadline: Option<u64>,
    #[arg(short = 'n', long = "name")]
    pub opt_max_entries_per_user: Option<usize>,
    #[arg(short = 'n', long = "name")]
    pub opt_prize_distribution: Option<Vec<u8>>,
    #[arg(short = 'n', long = "name")]
    pub opt_whitelist: Option<Vec<String>>,
    #[arg(short = 'n', long = "name")]
    pub opt_burn_percentage: Option<u128>,
}

impl CreateLotteryPollArgs {
    pub fn get_opt_whitelist_arg(&self) -> Option<Vec<Address>> {
        let mut opt_whitelist_with_addresses = Vec::new();
        if self.opt_whitelist.is_none() {
            return Option::None;
        }

        for str_address in self.opt_whitelist.as_ref().unwrap() {
            opt_whitelist_with_addresses.push(Address::from_slice(str_address.as_bytes()));
        }

        return Some(opt_whitelist_with_addresses);
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct LotteryNameArg {
    /// The caller address
    #[arg(short = 'c', long = "address")]
    pub caller: String,
    /// The name of the lottery
    #[arg(short = 'n', long = "name")]
    pub name: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ClaimRewardsArg {
    /// The name of the lottery
    #[arg(short = 'n', long = "name")]
    pub tokens: Vec<String>,
}
