use clap::{Args, Parser, Subcommand};

/// ForwarderBlind Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// ForwarderBlind Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "wrap", about = "Wrap EGLD into WEGLD")]
    WrapEgld(WrapEgldArgs),
    #[command(
        name = "swap1",
        about = "Swap WEGLD for USDC via the DEX pair contract"
    )]
    SwapWegldForUsdc(SwapWegldForUsdcArgs),
    #[command(
        name = "swap2",
        about = "Swap USDC for WEGLD via the DEX pair contract"
    )]
    SwapUsdcForWegld(SwapUsdcForWegldArgs),
    #[command(
        name = "get-rate",
        about = "Get the approximate WEGLD -> USDC conversion rate"
    )]
    GetRate(GetRateArgs),
    #[command(
        name = "get-liquidity",
        about = "Show the liquidity reserves in the WEGLD/USDC pair"
    )]
    GetLiquidity,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WrapEgldArgs {
    /// Amount of EGLD to wrap, in denomination (1 EGLD = 10^18)
    #[arg(short = 'a', long = "amount")]
    pub amount: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SwapWegldForUsdcArgs {
    /// Amount of WEGLD to sell
    #[arg(short = 'a', long = "wegld-amount")]
    pub wegld_amount: u64,
    /// Minimum amount of USDC to accept (slippage guard)
    #[arg(short = 'm', long = "usdc-amount-min", default_value = "1")]
    pub usdc_amount_min: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SwapUsdcForWegldArgs {
    /// Amount of USDC to sell
    #[arg(short = 'a', long = "usdc-amount")]
    pub usdc_amount: u64,
    /// Minimum amount of WEGLD to accept (slippage guard)
    #[arg(short = 'm', long = "wegld-amount-min", default_value = "1")]
    pub wegld_amount_min: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct GetRateArgs {
    /// Amount of WEGLD to price (default: 1 EGLD = 10^18)
    #[arg(
        short = 'a',
        long = "wegld-amount",
        default_value = "1000000000000000000"
    )]
    pub wegld_amount: u64,
}
