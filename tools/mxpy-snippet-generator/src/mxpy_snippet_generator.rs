use std::str::FromStr;

use multiversx_sc::codec::{top_encode_to_vec_u8_or_panic, TopEncode};
use num_traits::Zero;

mod cmd_builder;
mod constants;
mod helper_types;

use cmd_builder::*;
use constants::*;
use helper_types::*;

#[derive(PartialEq, Eq)]
pub enum PrintOption {
    MxpySnippet,
    TxData,
    Both,
}

pub struct MxpySnippetGenerator {
    wallet_type: WalletType,
    sender_nonce: Option<u64>,
    tx: TransactionType,
    gas_limit: u64,
    egld_value: num_bigint::BigUint,
    arguments: Vec<Vec<u8>>,
    proxy: String,
    chain_id: String,
}

impl MxpySnippetGenerator {
    pub fn new_sc_deploy(
        chain_config: ChainConfig,
        wallet_type: WalletType,
        deploy_type: DeployType,
        opt_json_out_file: Option<String>,
        gas_limit: u64,
    ) -> Self {
        let bounded_gas_limit = core::cmp::min(gas_limit, MAX_GAS_LIMIT);
        let (proxy, chain_id) = chain_config.to_strings();

        MxpySnippetGenerator {
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

    pub fn new_sc_upgrade(
        chain_config: ChainConfig,
        wallet_type: WalletType,
        dest_address_bech32: String,
        deploy_type: DeployType,
        opt_json_out_file: Option<String>,
        gas_limit: u64,
    ) -> Self {
        let bounded_gas_limit = core::cmp::min(gas_limit, MAX_GAS_LIMIT);
        let (proxy, chain_id) = chain_config.to_strings();

        MxpySnippetGenerator {
            wallet_type,
            sender_nonce: None,
            tx: TransactionType::Upgrade {
                dest_address_bech32,
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

    pub fn new_sc_call(
        chain_config: ChainConfig,
        wallet_type: WalletType,
        sender_address_bech32: String,
        dest_address_bech32: String,
        function: String,
        gas_limit: u64,
    ) -> Self {
        let bounded_gas_limit = core::cmp::min(gas_limit, MAX_GAS_LIMIT);
        let (proxy, chain_id) = chain_config.to_strings();

        MxpySnippetGenerator {
            wallet_type,
            sender_nonce: None,
            tx: TransactionType::Call {
                sender_address_bech32,
                dest_address_bech32,
                function,
                esdt_transfers: Vec::new(),
            },
            gas_limit: bounded_gas_limit,
            egld_value: num_bigint::BigUint::zero(),
            arguments: Vec::new(),
            proxy,
            chain_id,
        }
    }

    pub fn new_sc_query(
        chain_config: ChainConfig,
        dest_address_bech32: String,
        function: String,
    ) -> Self {
        let (proxy, chain_id) = chain_config.to_strings();

        MxpySnippetGenerator {
            wallet_type: WalletType::PemPath(String::new()),
            sender_nonce: None,
            tx: TransactionType::Query {
                dest_address_bech32,
                function,
            },
            gas_limit: 0,
            egld_value: num_bigint::BigUint::zero(),
            arguments: Vec::new(),
            proxy,
            chain_id,
        }
    }

    pub fn set_egld_value(&mut self, egld_value: &num_bigint::BigUint) {
        self.egld_value = egld_value.clone();
    }

    pub fn add_esdt_transfer(
        &mut self,
        token_id: String,
        token_nonce: u64,
        amount: num_bigint::BigUint,
    ) {
        self.tx.add_esdt_transfer(token_id, token_nonce, amount);
    }

    pub fn set_sender_nonce(&mut self, nonce: u64) {
        self.sender_nonce = Some(nonce);
    }

    fn encode_arg<T: TopEncode>(&mut self, arg: &T) -> Vec<u8> {
        top_encode_to_vec_u8_or_panic(arg)
    }

    pub fn add_argument<T: TopEncode>(&mut self, arg: &T) {
        let arg_bytes = self.encode_arg(arg);
        self.arguments.push(arg_bytes);
    }

    pub fn print(mut self, print_option: PrintOption) {
        let mut cmd_builder = CmdBuilder::new(MXPY_PROGRAM_NAME);
        cmd_builder.add_flag(VERBOSE_FLAG);
        cmd_builder.add_command(CONTRACT_COMMAND_NAME);

        let function_name: Option<String>;
        match &self.tx {
            TransactionType::Deploy {
                deploy_type,
                opt_json_out_file,
            } => {
                cmd_builder.add_command(DEPLOY_COMMAND_NAME);

                match deploy_type {
                    DeployType::ProjectPath(path) => {
                        cmd_builder.add_raw_named_argument(PROJECT_ARG_NAME, path);
                    },
                    DeployType::WasmFilePath(path) => {
                        cmd_builder.add_raw_named_argument(WASM_PATH_ARG_NAME, path);
                    },
                }

                if let Some(json_out_file) = opt_json_out_file {
                    cmd_builder.add_raw_named_argument(OUT_FILE_PATH_ARG_NAME, json_out_file);
                }

                self.handle_common_non_query_steps(&mut cmd_builder);
                function_name = None;
            },
            TransactionType::Upgrade {
                dest_address_bech32,
                deploy_type,
                opt_json_out_file,
            } => {
                cmd_builder.add_command(UPGRADE_COMMAND_NAME);
                cmd_builder.append_string_no_quotes(dest_address_bech32);

                match deploy_type {
                    DeployType::ProjectPath(path) => {
                        cmd_builder.add_raw_named_argument(PROJECT_ARG_NAME, path);
                    },
                    DeployType::WasmFilePath(path) => {
                        cmd_builder.add_raw_named_argument(WASM_PATH_ARG_NAME, path);
                    },
                }

                if let Some(json_out_file) = opt_json_out_file {
                    cmd_builder.add_raw_named_argument(OUT_FILE_PATH_ARG_NAME, json_out_file);
                }

                self.handle_common_non_query_steps(&mut cmd_builder);
                function_name = None;
            },
            TransactionType::Call {
                sender_address_bech32,
                dest_address_bech32,
                function,
                esdt_transfers,
            } => {
                cmd_builder.add_command(CALL_COMMAND_NAME);

                match esdt_transfers.len() {
                    0 => {
                        let dest_clone = dest_address_bech32.clone();
                        let function_clone = function.clone();

                        function_name = Some(function.clone());
                        self.contract_call_no_esdt(&mut cmd_builder, dest_clone, function_clone);
                    },
                    1 => {
                        let sender_clone = sender_address_bech32.clone();
                        let dest_clone = dest_address_bech32.clone();
                        let function_clone = function.clone();
                        let esdt_transfer_clone = esdt_transfers[0].clone();

                        let transfer_func_name = self.contract_call_one_esdt(
                            &mut cmd_builder,
                            sender_clone,
                            dest_clone,
                            function_clone,
                            esdt_transfer_clone,
                        );
                        function_name = Some(transfer_func_name);
                    },
                    _ => {
                        let sender_clone = sender_address_bech32.clone();
                        let dest_clone = dest_address_bech32.clone();
                        let function_clone = function.clone();
                        let transfers_clone = esdt_transfers.clone();

                        self.contract_call_multi_transfer(
                            &mut cmd_builder,
                            sender_clone,
                            dest_clone,
                            function_clone,
                            transfers_clone,
                        );
                        function_name = Some(MULTI_TRANSFER_FUNC_NAME.to_owned());
                    },
                }

                self.handle_common_non_query_steps(&mut cmd_builder);
            },
            TransactionType::Query {
                dest_address_bech32,
                function,
            } => {
                cmd_builder.add_command(QUERY_COMMAND_NAME);

                let dest_clone = dest_address_bech32.clone();
                let function_clone = function.clone();

                function_name = Some(function.clone());
                self.contract_call_no_esdt(&mut cmd_builder, dest_clone, function_clone);
            },
        }

        if !self.arguments.is_empty() {
            cmd_builder.add_flag(ARGUMENTS_ARG_NAME);
            for arg in &self.arguments {
                cmd_builder.add_standalone_argument(arg);
            }
        }

        cmd_builder.add_raw_named_argument(PROXY_ARG_NAME, &self.proxy);
        cmd_builder.add_raw_named_argument(CHAIN_ID_ARG_NAME, &self.chain_id);
        cmd_builder.add_flag(SEND_FLAG);

        match print_option {
            PrintOption::MxpySnippet => cmd_builder.print(),
            PrintOption::TxData => Self::print_tx_data(function_name, &self.arguments),
            PrintOption::Both => {
                Self::print_tx_data(function_name, &self.arguments);
                println!();
                cmd_builder.print();
            },
        }
    }

    fn print_tx_data(opt_function_name: Option<String>, arguments: &[Vec<u8>]) {
        let function_name = match opt_function_name {
            Some(f) => f,
            None => return,
        };

        let mut tx_data = function_name;
        for arg in arguments {
            let arg_as_hex = CmdBuilder::to_hex(arg);
            tx_data += "@";
            tx_data += &arg_as_hex;
        }

        println!("{tx_data}");
    }

    fn handle_common_non_query_steps(&self, cmd_builder: &mut CmdBuilder) {
        match &self.wallet_type {
            WalletType::PemPath(path) => {
                cmd_builder.add_raw_named_argument(PEM_PATH_ARG_NAME, path);
            },
            WalletType::KeyFile {
                keyfile_path,
                passfile_path,
            } => {
                cmd_builder.add_raw_named_argument(KEYFILE_PATH_ARG_NAME, keyfile_path);
                cmd_builder.add_raw_named_argument(PASSFILE_PATH_ARG_NAME, passfile_path);
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
    }

    fn contract_call_no_esdt(
        &mut self,
        cmd_builder: &mut CmdBuilder,
        dest_address_bech32: String,
        function: String,
    ) {
        cmd_builder.append_string_no_quotes(&dest_address_bech32);
        cmd_builder.add_raw_named_argument(FUNCTION_ARG_NAME, &function);
    }

    fn contract_call_one_esdt(
        &mut self,
        cmd_builder: &mut CmdBuilder,
        sender_address_bech32: String,
        dest_address_bech32: String,
        function: String,
        esdt_transfer: EsdtTransferTuple,
    ) -> String {
        let (token_id, token_nonce, amount) = esdt_transfer;
        if token_nonce == 0 {
            cmd_builder.append_string_no_quotes(&dest_address_bech32);
            cmd_builder.add_raw_named_argument(FUNCTION_ARG_NAME, ESDT_TRANSFER_FUNC_NAME);

            let mut args = Vec::new();
            let token_id_encoded = self.encode_arg(&token_id);
            let amount_encoded = self.encode_arg(&amount.to_bytes_be());
            let function_encoded = self.encode_arg(&function);

            args.push(token_id_encoded);
            args.push(amount_encoded);
            args.push(function_encoded);
            args.extend_from_slice(&self.arguments);

            self.arguments = args;

            ESDT_TRANSFER_FUNC_NAME.to_owned()
        } else {
            cmd_builder.append_string_no_quotes(&sender_address_bech32);
            cmd_builder.add_raw_named_argument(FUNCTION_ARG_NAME, NFT_TRANSFER_FUNC_NAME);

            let dest_addr = bech32_to_bytes(&dest_address_bech32);

            let mut args = Vec::new();
            let token_id_encoded = self.encode_arg(&token_id);
            let token_nonce_encoded = self.encode_arg(&token_nonce);
            let amount_encoded = self.encode_arg(&amount.to_bytes_be());
            let dest_encoded = self.encode_arg(&dest_addr);
            let function_encoded = self.encode_arg(&function);

            args.push(token_id_encoded);
            args.push(token_nonce_encoded);
            args.push(amount_encoded);
            args.push(dest_encoded);
            args.push(function_encoded);
            args.extend_from_slice(&self.arguments);

            self.arguments = args;

            NFT_TRANSFER_FUNC_NAME.to_owned()
        }
    }

    fn contract_call_multi_transfer(
        &mut self,
        cmd_builder: &mut CmdBuilder,
        sender_address_bech32: String,
        dest_address_bech32: String,
        function: String,
        esdt_transfers: Vec<EsdtTransferTuple>,
    ) {
        cmd_builder.append_string_no_quotes(&sender_address_bech32);
        cmd_builder.add_raw_named_argument(FUNCTION_ARG_NAME, MULTI_TRANSFER_FUNC_NAME);

        let dest_addr = bech32_to_bytes(&dest_address_bech32);

        let mut args = Vec::new();
        let dest_encoded = self.encode_arg(&dest_addr);
        let nr_transfers_encoded = self.encode_arg(&esdt_transfers.len());
        let function_encoded = self.encode_arg(&function);

        args.push(dest_encoded);
        args.push(nr_transfers_encoded);

        for (token_id, token_nonce, amount) in esdt_transfers {
            let token_id_encoded = self.encode_arg(&token_id);
            let token_nonce_encoded = self.encode_arg(&token_nonce);
            let amount_encoded = self.encode_arg(&amount.to_bytes_be());

            args.push(token_id_encoded);
            args.push(token_nonce_encoded);
            args.push(amount_encoded);
        }

        args.push(function_encoded);
        args.extend_from_slice(&self.arguments);

        self.arguments = args;
    }
}

fn main() {
    // sc deploy
    let mut generator = MxpySnippetGenerator::new_sc_deploy(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        DeployType::WasmFilePath("../path_to_wasm/file.wasm".to_owned()),
        Some("some_out_file.json".to_owned()),
        100_000_000,
    );

    let my_val = 5u64;
    let other_arg = "some string";

    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    println!("SC Deploy:");
    generator.print(PrintOption::MxpySnippet);
    println!();
    println!();

    // sc upgrade
    generator = MxpySnippetGenerator::new_sc_upgrade(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        "erd1qqqqqqqqqqqqqpgqju6muu3kj2uqpqwz798g2jeepyn8jwn5rkqsgwvu0x".to_owned(),
        DeployType::WasmFilePath("../path_to_wasm/file.wasm".to_owned()),
        Some("some_out_file.json".to_owned()),
        100_000_000,
    );
    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    println!("SC Upgrade:");
    generator.print(PrintOption::MxpySnippet);
    println!();
    println!();

    // sc call
    generator = MxpySnippetGenerator::new_sc_call(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        "erd1dyxrt6ky32hpvqh9w9kgt262z4c6su65myzy33styw47m9nkrplqrtnc5r".to_owned(),
        "erd1qqqqqqqqqqqqqpgqju6muu3kj2uqpqwz798g2jeepyn8jwn5rkqsgwvu0x".to_owned(),
        "someEndpointName".to_owned(),
        100_000_000,
    );
    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    println!("SC Call:");
    generator.print(PrintOption::Both);
    println!();
    println!();

    // sc call with EGLD

    generator = MxpySnippetGenerator::new_sc_call(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        "erd1dyxrt6ky32hpvqh9w9kgt262z4c6su65myzy33styw47m9nkrplqrtnc5r".to_owned(),
        "erd1qqqqqqqqqqqqqpgqju6muu3kj2uqpqwz798g2jeepyn8jwn5rkqsgwvu0x".to_owned(),
        "someEndpointName".to_owned(),
        100_000_000,
    );
    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    generator.set_egld_value(&num_bigint::BigUint::from_str("10_000_000_000_000_000_000").unwrap());

    println!("SC Call with EGLD transfer:");
    generator.print(PrintOption::Both);
    println!();
    println!();

    // sc call with one ESDT transfer

    generator = MxpySnippetGenerator::new_sc_call(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        "erd1dyxrt6ky32hpvqh9w9kgt262z4c6su65myzy33styw47m9nkrplqrtnc5r".to_owned(),
        "erd1qqqqqqqqqqqqqpgqju6muu3kj2uqpqwz798g2jeepyn8jwn5rkqsgwvu0x".to_owned(),
        "someEndpointName".to_owned(),
        100_000_000,
    );
    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    let amount = num_bigint::BigUint::from_str("10_000_000_000_000_000_000").unwrap();
    generator.add_esdt_transfer("MYTOKEN-abcdef".to_owned(), 0, amount);

    println!("SC Call with one ESDT transfer:");
    generator.print(PrintOption::Both);
    println!();
    println!();

    // sc call with one NFT transfer

    generator = MxpySnippetGenerator::new_sc_call(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        "erd1dyxrt6ky32hpvqh9w9kgt262z4c6su65myzy33styw47m9nkrplqrtnc5r".to_owned(),
        "erd1qqqqqqqqqqqqqpgqju6muu3kj2uqpqwz798g2jeepyn8jwn5rkqsgwvu0x".to_owned(),
        "someEndpointName".to_owned(),
        100_000_000,
    );
    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    let amount = num_bigint::BigUint::from_str("10_000_000_000_000_000_000").unwrap();
    generator.add_esdt_transfer("MYTOKEN-abcdef".to_owned(), 5, amount);

    println!("SC Call with one NFT transfer:");
    generator.print(PrintOption::Both);
    println!();
    println!();

    // sc call with multiple ESDT transfers

    generator = MxpySnippetGenerator::new_sc_call(
        ChainConfig::Devnet,
        WalletType::PemPath("../some_path/my_file.pem".to_owned()),
        "erd1dyxrt6ky32hpvqh9w9kgt262z4c6su65myzy33styw47m9nkrplqrtnc5r".to_owned(),
        "erd1qqqqqqqqqqqqqpgqju6muu3kj2uqpqwz798g2jeepyn8jwn5rkqsgwvu0x".to_owned(),
        "someEndpointName".to_owned(),
        100_000_000,
    );
    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    let amount = num_bigint::BigUint::from_str("10_000_000_000_000_000_000").unwrap();
    generator.add_esdt_transfer("MYTOKEN-abcdef".to_owned(), 5, amount.clone());

    generator.add_esdt_transfer("OTHERTOK-123456".to_owned(), 0, amount);

    println!("SC Call multiple ESDT transfers:");
    generator.print(PrintOption::Both);
    println!();
    println!();

    // SC Query

    generator = MxpySnippetGenerator::new_sc_query(
        ChainConfig::Devnet,
        "erd1qqqqqqqqqqqqqpgqju6muu3kj2uqpqwz798g2jeepyn8jwn5rkqsgwvu0x".to_owned(),
        "someEndpointName".to_owned(),
    );
    generator.add_argument(&my_val);
    generator.add_argument(&other_arg);

    println!("SC Query:");
    generator.print(PrintOption::Both);
    println!();
    println!();
}
