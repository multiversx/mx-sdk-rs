use anyhow::{Context, Result, anyhow};
use multiversx_sc_snippets::{
    hex,
    imports::{Bech32Address, GatewayHttpProxy},
    sdk::data::vm::VMQueryInput,
};

use super::tx_cli_common::{build_arg_buffer, to_json_pretty};
use crate::cli::cli_args_tx::QueryArgs;

pub async fn tx_query(args: &QueryArgs) {
    if let Err(e) = tx_query_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_query_inner(args: &QueryArgs) -> Result<()> {
    let contract = Bech32Address::try_from_bech32_string(args.contract.clone())?;

    // Encode arguments as hex strings (the gateway expects hex, no "0x" prefix).
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let hex_args: Vec<String> = arg_buffer
        .raw_arg_iter()
        .map(|buf| hex::encode(buf.to_boxed_bytes().as_slice()))
        .collect();

    let req = VMQueryInput {
        sc_address: contract,
        func_name: args.function.clone(),
        args: hex_args,
    };

    let proxy = GatewayHttpProxy::new(args.gateway.proxy.clone());
    let result = proxy
        .execute_vmquery(&req)
        .await
        .context("VM query failed")?;

    if !result.data.is_ok() {
        return Err(anyhow!(
            "VM query returned error: {} ({})",
            result.data.return_message,
            result.data.return_code,
        ));
    }

    // Decode base64 return values and display as hex strings, matching mxpy output.
    let return_data: Vec<String> = result
        .data
        .return_data_base64_decode()
        .into_iter()
        .map(|bytes| hex::encode(&bytes))
        .collect();

    let json = to_json_pretty(&return_data)?;
    println!("{json}");
    Ok(())
}
