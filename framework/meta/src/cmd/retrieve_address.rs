use multiversx_sc_snippets::account_tool;

use crate::cli::AccountArgs;

/// Interprets arguments and call the account tool from `multiversx_sc_snippets`.
pub async fn retrieve_address(args: &AccountArgs) {
    let api_string = args.api.clone().expect("API needs to be specified");
    account_tool::print_account_as_scenario_set_state(api_string, args.address.to_string()).await;
}
