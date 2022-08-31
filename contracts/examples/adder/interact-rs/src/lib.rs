use adder::ProxyTrait as _;
use elrond_interact_snippets::{
    elrond_wasm::{
        elrond_codec::multi_types::MultiValueVec,
        storage::mappers::SingleValue,
        types::{Address, CodeMetadata},
    },
    elrond_wasm_debug::{
        bech32, mandos::interpret_trait::InterpreterContext, mandos_system::model::*, ContractInfo,
        DebugApi,
    },
    env_logger,
    erdrs::interactors::wallet::Wallet,
    tokio, Interactor,
};
use std::{
    env::Args,
    io::{Read, Write},
};

const GATEWAY: &str = elrond_interact_snippets::erdrs::blockchain::rpc::DEVNET_GATEWAY;
const PEM: &str = "alice.pem";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const DEFAULT_ADDRESS_EXPR: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

type ContractType = ContractInfo<adder::Proxy<DebugApi>>;

