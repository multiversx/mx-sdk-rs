use elrond_wasm::elrond_codec::TopEncode;
use num_traits::Zero;

mod cmd_builder;
mod constants;
mod helper_types;

use cmd_builder::*;
use constants::*;
use helper_types::*;

pub struct ErdpySnippetGenerator {
    wallet_type: WalletType,
    sender_nonce: Option<u64>,
    tx: TransactionType,
    gas_limit: u64,
    egld_value: num_bigint::BigUint,
    arguments: Vec<Vec<u8>>,
    proxy: String,
    chain_id: String,
}

impl ErdpySnippetGenerator {
    pub fn new_sc_deploy(
        chain_config: ChainConfig,
        wallet_type: WalletType,
        deploy_type: DeployType,
        opt_json_out_file: Option<String>,
        gas_limit: u64,
    ) -> Self {
        let bounded_gas_limit = core::cmp::min(gas_limit, MAX_GAS_LIMIT);
        let (proxy, chain_id) = chain_config.to_strings();

        ErdpySnippetGenerator {
            wallet_type,
            sender_nonce: None,
            tx: TransactionType::Deploy {
                deploy_type,
                opt_json_out_file,
            },
            gas_limit: bounded_gas_limit,
            egld_value: num_bigint::BigUint::zero(),
            arguments: Vec::new(),
            proxy,
            chain_id,
        }
    }

    pub fn set_egld_value(&mut self, egld_value: &num_bigint::BigUint) {
        self.egld_value = egld_value.clone();
    }

    pub fn set_sender_nonce(&mut self, nonce: u64) {
        self.sender_nonce = Some(nonce);
    }

    pub fn add_argument<T: TopEncode>(&mut self, arg: &T) {
        let mut arg_bytes = Vec::new();
        arg.top_encode(&mut arg_bytes).unwrap();

        self.arguments.push(arg_bytes);
    }

    pub fn print(self) {
        let mut cmd_builder = CmdBuilder::new(ERDPY_PROGRAM_NAME);
        cmd_builder.add_flag(VERBOSE_FLAG);
        cmd_builder.add_command(CONTRACT_COMMAND_NAME);

        match self.tx {
            TransactionType::Deploy {
                deploy_type,
                opt_json_out_file,
            } => {
                cmd_builder.add_command(DEPLOY_COMMAND_NAME);

                match deploy_type {
                    DeployType::ProjectPath(path) => {
                        cmd_builder.add_raw_named_argument(PROJECT_ARG_NAME, &path);
                    },
                    DeployType::WasmFilePath(path) => {
                        cmd_builder.add_raw_named_argument(WASM_PATH_ARG_NAME, &path);
                    },
                }

                if let Some(json_out_file) = opt_json_out_file {
                    cmd_builder.add_raw_named_argument(OUT_FILE_PATH_ARG_NAME, &json_out_file);
                }
            },
            // TODO: Remember to handle sc query differently
            _ => {},
        }

        match self.wallet_type {
            WalletType::PemPath(path) => {
                cmd_builder.add_raw_named_argument(PEM_PATH_ARG_NAME, &path);
            },
            WalletType::KeyFile {
                keyfile_path,
                passfile_path,
            } => {
                cmd_builder.add_raw_named_argument(KEYFILE_PATH_ARG_NAME, &keyfile_path);
                cmd_builder.add_raw_named_argument(PASSFILE_PATH_ARG_NAME, &passfile_path);
            },
        }

        match self.sender_nonce {
            Some(nonce) => {
                cmd_builder
                    .add_numerical_argument(NONCE_ARG_NAME, &num_bigint::BigUint::from(nonce));
            },
            None => {
                cmd_builder.add_flag(RECALL_NONCE_FLAG);
            },
        }

        if self.egld_value > num_bigint::BigUint::zero() {
            cmd_builder.add_numerical_argument(EGLD_VALUE_ARG_NAME, &self.egld_value);
        }

        cmd_builder.add_numerical_argument(
            GAS_LIMIT_ARG_NAME,
            &num_bigint::BigUint::from(self.gas_limit),
        );

        if !self.arguments.is_empty() {
            cmd_builder.add_flag(ARGUMENTS_ARG_NAME);
            for arg in self.arguments {
                cmd_builder.add_standalone_argument(&arg);
            }
        }

        cmd_builder.add_raw_named_argument(PROXY_ARG_NAME, &self.proxy);
        cmd_builder.add_raw_named_argument(CHAIN_ID_ARG_NAME, &self.chain_id);
        cmd_builder.add_flag(SEND_FLAG);

        cmd_builder.print();
    }
}

fn main() {
    let mut generator = ErdpySnippetGenerator::new_sc_deploy(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        DeployType::WasmFilePath("../path_to_wasm/file.wasm".to_owned()),
        Some("some_out_file.json".to_owned()),
        10_000_000,
    );

    let my_val = 5u64;
    let other_arg = "some string";

    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    generator.print();
}
