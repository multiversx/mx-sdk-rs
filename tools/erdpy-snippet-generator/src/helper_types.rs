use crate::constants::*;

pub enum WalletType {
    PemPath(String),
    KeyFile {
        keyfile_path: String,
        passfile_path: String,
    },
}

pub enum ChainConfig {
    Testnet,
    Devnet,
    Mainnet,
    Custom { proxy: String, chain_id: String },
}

impl ChainConfig {
    pub fn to_strings(&self) -> (String, String) {
        match &*self {
            ChainConfig::Testnet => (TESTNET_PROXY.to_owned(), TESTNET_CHAIN_ID.to_owned()),
            ChainConfig::Devnet => (DEVNET_PROXY.to_owned(), DEVNET_CHAIN_ID.to_owned()),
            ChainConfig::Mainnet => (MAINNET_PROXY.to_owned(), MAINNET_CHAIN_ID.to_owned()),
            ChainConfig::Custom { proxy, chain_id } => (proxy.clone(), chain_id.clone()),
        }
    }
}

pub enum DeployType {
    ProjectPath(String),
    WasmFilePath(String),
}

pub enum TransactionType {
    Deploy {
        deploy_type: DeployType,
        opt_json_out_file: Option<String>,
    },
    Upgrade {
        dest_address_bech32: String,
        deploy_type: DeployType,
        opt_json_out_file: Option<String>,
    },
    Call {
        sender_address_bech32: String,
        dest_address_bech32: String,
        function: String,
        esdt_transfers: Vec<(String, u64, num_bigint::BigUint)>,
    },
    Query {
        dest_address_bech32: String,
        function: String,
        esdt_transfers: Vec<(String, u64, num_bigint::BigUint)>,
    },
}
