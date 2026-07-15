use crate::cli::{LedgerAction, LedgerArgs};

pub fn ledger_cmd(args: &LedgerArgs) {
    if let Err(e) = ledger_cmd_inner(args) {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn ledger_cmd_inner(args: &LedgerArgs) -> anyhow::Result<()> {
    match &args.command {
        LedgerAction::Addresses(a) => print_addresses(a.num_addresses),
        LedgerAction::Version => print_version(),
    }
}

#[cfg(feature = "ledger")]
fn print_addresses(num_addresses: u32) -> anyhow::Result<()> {
    use multiversx_sc_snippets::sdk::wallet::ledger::LedgerApp;

    let app = LedgerApp::new()?;
    for i in 0..num_addresses {
        let address = app.get_address(i)?;
        println!("account index = 0 | address index = {i} | address: {address}");
    }
    Ok(())
}

#[cfg(not(feature = "ledger"))]
fn print_addresses(_num_addresses: u32) -> anyhow::Result<()> {
    anyhow::bail!("Ledger support is not available; recompile with the `ledger` feature enabled")
}

#[cfg(feature = "ledger")]
fn print_version() -> anyhow::Result<()> {
    use multiversx_sc_snippets::sdk::wallet::ledger::LedgerApp;

    let app = LedgerApp::new()?;
    let version = app.get_version()?;
    println!("MultiversX App version: {version}");
    Ok(())
}

#[cfg(not(feature = "ledger"))]
fn print_version() -> anyhow::Result<()> {
    anyhow::bail!("Ledger support is not available; recompile with the `ledger` feature enabled")
}
