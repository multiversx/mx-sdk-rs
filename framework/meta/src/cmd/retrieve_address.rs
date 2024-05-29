use multiversx_sc_scenario::{imports::Bech32Address, standalone::account_tool};
use multiversx_sdk::blockchain::CommunicationProxy;

use crate::cli::AccountArgs;

pub async fn retrieve_address(args: &AccountArgs) {
    let api_string = args.api.clone().expect("API needs to be specified");
    let api = CommunicationProxy::new(api_string);
    account_tool::print_account_as_scenario_set_state(
        &api,
        &Bech32Address::from_bech32_string(args.address.to_string()),
    )
    .await;
}
