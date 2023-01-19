use clap::{ArgAction, Args};

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct BuildArgs {
    /// Adds debug symbols in the resulting WASM binary. Adds bloat, but helps with debugging. Do not use in production.
    #[arg(long = "wasm-symbols", verbatim_doc_comment)]
    pub wasm_symbols: bool,

    /// Overrides the main contract output name.
    #[arg(long = "wasm-name", verbatim_doc_comment)]
    pub wasm_name_override: Option<String>,

    /// Adds given suffix to all built contracts.
    #[arg(long = "wasm-suffix", verbatim_doc_comment)]
    pub wasm_name_suffix: Option<String>,

    /// True if wasm-opt should be used.
    #[arg(
        long = "no-wasm-opt",
        help = "Skips applying the wasm-opt tool after building the contracts.",
        action = ArgAction::SetFalse,
    )]
    pub wasm_opt: bool,

    /// Also generate a WAT file when building.
    #[arg(long = "wat", verbatim_doc_comment)]
    pub wat: bool,

    #[arg(
        long = "no-imports",
        help = "Skips extracting the EI imports after building the contracts.",
        action = ArgAction::SetFalse,
    )]
    pub extract_imports: bool,

    /// Allows specifying the target directory where the Rust compiler will build the intermediary files.
    /// Sharing the same target directory can speed up building multiple contract crates at once.
    #[arg(long = "target-dir", verbatim_doc_comment)]
    pub target_dir: Option<String>,

    /// Generate a twiggy top report after building.
    #[arg(long = "twiggy-top", verbatim_doc_comment)]
    pub twiggy_top: bool,

    /// Generate a twiggy paths report after building.
    #[arg(long = "twiggy-paths", verbatim_doc_comment)]
    pub twiggy_paths: bool,

    /// Generate a twiggy monos report after building.
    #[arg(long = "twiggy-monos", verbatim_doc_comment)]
    pub twiggy_monos: bool,

    /// Generate a twiggy dominators report after building.
    #[arg(long = "twiggy-dominators", verbatim_doc_comment)]
    pub twiggy_dominators: bool,

    /// Backwards compatibility with mxpy, delete when github actions are fixed.
    #[deprecated]
    #[arg(long = "target", verbatim_doc_comment)]
    pub target: Option<String>,

    /// Backwards compatibility with mxpy, delete when github actions are fixed.
    #[deprecated]
    #[arg(long, verbatim_doc_comment)]
    pub release: bool,

    /// Backwards compatibility with mxpy, delete when github actions are fixed.
    #[deprecated]
    #[arg(long = "out-dir", verbatim_doc_comment)]
    pub out_dir: Option<String>,
}

impl Default for BuildArgs {
    #[allow(deprecated)]
    fn default() -> Self {
        BuildArgs {
            wasm_symbols: false,
            wasm_name_override: None,
            wasm_name_suffix: None,
            wasm_opt: true,
            wat: false,
            extract_imports: true,
            target_dir: None,
            twiggy_top: false,
            twiggy_paths: false,
            twiggy_monos: false,
            twiggy_dominators: false,
            target: None,
            release: false,
            out_dir: None,
        }
    }
}

impl BuildArgs {
    /// Base config when calling `cargo run build-dbg`, with no additional configs.
    pub fn into_dbg(self) -> Self {
        BuildArgs {
            wasm_symbols: true,
            wasm_name_suffix: Some("dbg".to_string()),
            wasm_opt: false,
            wat: true,
            extract_imports: false,
            ..self
        }
    }

    /// Base config when calling `cargo run build-dbg`, with no additional configs.
    pub fn into_twiggy(self) -> Self {
        BuildArgs {
            twiggy_top: true,
            twiggy_paths: true,
            twiggy_monos: true,
            twiggy_dominators: true,
            ..self.into_dbg()
        }
    }

    pub fn has_twiggy_call(&self) -> bool {
        self.twiggy_top || self.twiggy_paths || self.twiggy_monos || self.twiggy_dominators
    }
}
