use clap::{Args, Subcommand};

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct CodehashArgs {
    #[command(subcommand)]
    pub command: CodehashCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum CodehashCliAction {
    #[command(
        name = "generate",
        about = "Computes the code hashes and writes a <name>.codehash.txt file next to each .wasm."
    )]
    Generate(CodehashGenerateArgs),

    #[command(
        name = "print",
        about = "Computes the code hashes and prints them to the console, without writing any file."
    )]
    Print(CodehashPrintArgs),

    #[command(
        name = "verify",
        about = "Checks that the existing .codehash.txt files match their .wasm files. Exits with an error if not."
    )]
    Verify(CodehashVerifyArgs),

    #[command(
        name = "clear",
        about = "Deletes the .codehash.txt files. The .wasm files are never touched."
    )]
    Clear(CodehashClearArgs),
}

/// Which .wasm files to process. Shared by all codehash actions.
#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodehashTargetArgs {
    /// A .wasm file, or a folder to search for .wasm files.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Only look at the top level of the folder, do not descend into subfolders.
    #[arg(long, default_value = "false", verbatim_doc_comment)]
    pub no_recursive: bool,

    /// Folder names to skip during the search, e.g. "target".
    #[arg(long, verbatim_doc_comment, num_args = 1..)]
    pub ignore: Vec<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodehashGenerateArgs {
    #[command(flatten)]
    pub target: CodehashTargetArgs,

    /// Write the .codehash.txt files to this folder instead of next to the .wasm files.
    #[arg(long, verbatim_doc_comment)]
    pub output_dir: Option<String>,

    /// Do not touch codehash files that already exist.
    #[arg(long, default_value = "false", verbatim_doc_comment)]
    pub skip_existing: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, clap::ValueEnum)]
pub enum CodehashPrintFormat {
    /// `<hash>  <path>`, one per line.
    #[default]
    Plain,
    /// A JSON array of `{ "path", "codehash" }` objects.
    Json,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodehashPrintArgs {
    #[command(flatten)]
    pub target: CodehashTargetArgs,

    /// Output format.
    #[arg(long, value_enum, default_value = "plain", verbatim_doc_comment)]
    pub format: CodehashPrintFormat,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodehashVerifyArgs {
    #[command(flatten)]
    pub target: CodehashTargetArgs,

    /// Do not fail for .wasm files that have no .codehash.txt file.
    #[arg(long, default_value = "false", verbatim_doc_comment)]
    pub allow_missing: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodehashClearArgs {
    #[command(flatten)]
    pub target: CodehashTargetArgs,

    /// Only list the files that would be deleted.
    #[arg(long, default_value = "false", verbatim_doc_comment)]
    pub dry_run: bool,
}
