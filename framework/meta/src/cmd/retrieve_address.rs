use multiversx_sc_snippets::account_tool;

use crate::cli::AccountArgs;

/// Interprets arguments and call the account tool from `multiversx_sc_snippets`.
pub async fn retrieve_address(args: &AccountArgs) {
    let api_string = args.api.clone().expect("API needs to be specified");
    let use_chain_simulator = args.chain_simulator.unwrap_or_default();
    account_tool::print_account_as_scenario_set_state(
        api_string,
        use_chain_simulator,
        args.address.to_string(),
    )
    .await;
}
