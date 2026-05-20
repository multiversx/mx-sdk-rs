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
    #[command(name = "swap1", about = "Swap WEGLD for USDC")]
    Swap1(Swap1Args),
    #[command(name = "swap2", about = "Swap USDC for WEGLD")]
    Swap2(Swap2Args),
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
    #[command(
        name = "drain",
        about = "Drain WEGLD and USDC balances from the forwarder contract back to the owner"
    )]
    Drain,
    #[command(
        name = "balances",
        about = "Display EGLD (wallets only), WEGLD, and USDC balances for all wallets and contracts"
    )]
    Balances,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct Swap1Args {
    #[command(subcommand)]
    pub method: SwapWegldForUsdcMethod,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct Swap2Args {
    #[command(subcommand)]
    pub method: SwapUsdcForWegldMethod,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum SwapWegldForUsdcMethod {
    #[command(name = "direct", about = "Swap directly on the DEX pair")]
    Direct(SwapWegldForUsdcArgs),
    #[command(name = "sync", about = "Swap via forwarder-blind using blind_sync")]
    Sync(SwapWegldForUsdcArgs),
    #[command(
        name = "async1",
        about = "Swap via forwarder-blind using blind_async_v1"
    )]
    Async1(SwapWegldForUsdcArgs),
    #[command(
        name = "async2",
        about = "Swap via forwarder-blind using blind_async_v2"
    )]
    Async2(SwapWegldForUsdcArgs),
    #[command(
        name = "te",
        about = "Swap via forwarder-blind using blind_transf_exec"
    )]
    Te(SwapWegldForUsdcArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum SwapUsdcForWegldMethod {
    #[command(name = "direct", about = "Swap directly on the DEX pair")]
    Direct(SwapUsdcForWegldArgs),
    #[command(name = "sync", about = "Swap via forwarder-blind using blind_sync")]
    Sync(SwapUsdcForWegldArgs),
    #[command(
        name = "async1",
        about = "Swap via forwarder-blind using blind_async_v1"
    )]
    Async1(SwapUsdcForWegldArgs),
    #[command(
        name = "async2",
        about = "Swap via forwarder-blind using blind_async_v2"
    )]
    Async2(SwapUsdcForWegldArgs),
    #[command(
        name = "te",
        about = "Swap via forwarder-blind using blind_transf_exec"
    )]
    Te(SwapUsdcForWegldArgs),
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
