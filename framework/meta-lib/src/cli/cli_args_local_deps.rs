use clap::Args;

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct LocalDepsArgs {
    /// Target directory where to generate local deps reports.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,
}
