use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc_scenario::{ContractInfo, DebugApi},
    tokio,
};

#[allow(dead_code)]
/// Default adder address
const DEFAULT_ADDER_ADDRESS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

pub type AdderContract = ContractInfo<adder::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    DebugApi::dummy();
    env_logger::init();
}
