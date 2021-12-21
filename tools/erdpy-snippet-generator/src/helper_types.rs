use elrond_wasm::types::Address;

use crate::constants::*;

pub type EsdtTransferTuple = (String, u64, num_bigint::BigUint);
const ADDRESS_LEN: usize = 32;

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
        esdt_transfers: Vec<EsdtTransferTuple>,
    },
    Query {
        dest_address_bech32: String,
        function: String,
    },
}

impl TransactionType {
    pub fn add_esdt_transfer(
        &mut self,
        token_id: String,
        token_nonce: u64,
        amount: num_bigint::BigUint,
    ) {
        match self {
            TransactionType::Call {
                sender_address_bech32: _,
                dest_address_bech32: _,
                function: _,
                esdt_transfers,
            } => {
                esdt_transfers.push((token_id, token_nonce, amount));
            },
            _ => {},
        }
    }
}

pub fn bech32_to_bytes(bech32_address: &str) -> Address {
    let (_, dest_address_bytes, _) = bech32::decode(bech32_address).unwrap();
    if dest_address_bytes.len() != ADDRESS_LEN {
        panic!("Invalid address length after decoding")
    }

    let mut addr_bytes = [0u8; ADDRESS_LEN];
    for (i, byte) in dest_address_bytes.iter().enumerate() {
        addr_bytes[i] = byte.to_u8();
    }

    Address::from(addr_bytes)
}
