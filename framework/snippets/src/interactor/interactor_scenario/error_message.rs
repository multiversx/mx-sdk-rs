use colored::Colorize;

pub(super) fn query_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Query failed: ".to_string().red().bold(),
        err.to_string().red().bold()
    );
}

pub(super) fn transfer_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Transfer failed: ".to_string().red().bold(),
        err.to_string().red().bold()
    );
}

pub(super) fn deploy_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Deploy failed: ".to_string().red().bold(),
        err.to_string().red().bold()
    );
}

pub(crate) fn sc_call_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Call failed: ".to_string().red().bold(),
        err.to_string().red().bold()
    );
}
