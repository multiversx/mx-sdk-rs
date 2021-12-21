pub const ERDPY_PROGRAM_NAME: &str = "erdpy";
pub const CONTRACT_COMMAND_NAME: &str = "contract";

// deploy/upgrade arg names
pub const DEPLOY_COMMAND_NAME: &str = "deploy";
pub const PROJECT_ARG_NAME: &str = "project";
pub const WASM_PATH_ARG_NAME: &str = "bytecode";
pub const OUT_FILE_PATH_ARG_NAME: &str = "outfile";

// general arg names
pub const PEM_PATH_ARG_NAME: &str = "pem";
pub const KEYFILE_PATH_ARG_NAME: &str = "keyfile";
pub const PASSFILE_PATH_ARG_NAME: &str = "passfile";
pub const PROXY_ARG_NAME: &str = "proxy";
pub const CHAIN_ID_ARG_NAME: &str = "chain";
pub const NONCE_ARG_NAME: &str = "nonce";
pub const GAS_LIMIT_ARG_NAME: &str = "gas-limit";
pub const EGLD_VALUE_ARG_NAME: &str = "value";
pub const ARGUMENTS_ARG_NAME: &str = "arguments";

// general flags
pub const VERBOSE_FLAG: &str = "verbose";
pub const RECALL_NONCE_FLAG: &str = "recall-nonce";
pub const SEND_FLAG: &str = "send";

pub const MAX_GAS_LIMIT: u64 = 500_000_000;

// default proxies
pub const TESTNET_PROXY: &str = "https://testnet-gateway.elrond.com";
pub const DEVNET_PROXY: &str = "https://devnet-gateway.elrond.com";
pub const MAINNET_PROXY: &str = "https://gateway.elrond.com";

// default chain IDs
pub const TESTNET_CHAIN_ID: &str = "T";
pub const DEVNET_CHAIN_ID: &str = "D";
pub const MAINNET_CHAIN_ID: &str = "1";
