use multiversx_sc::{api::uncallable::UncallableApi, contract_base::ContractAbiProvider};

struct NoAbiProvider;

impl ContractAbiProvider for NoAbiProvider {
    type Api = UncallableApi;

    fn abi() -> multiversx_sc::abi::ContractAbi {
        multiversx_sc::abi::ContractAbi {
            name: "no-abi",
            ..Default::default()
        }
    }
}

fn main() {
    multiversx_sc_meta::cli_main::<NoAbiProvider>();
}
