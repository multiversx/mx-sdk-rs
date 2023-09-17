use clap::{ArgAction, Args};

use super::CliArgsToRaw;

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct BuildArgs {
    /// Require that the Cargo.lock in the wasm crates is up to date.
    #[arg(long = "locked", verbatim_doc_comment)]
    pub locked: bool,

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

    /// Also emit MIR files when building.
    #[arg(long = "mir", verbatim_doc_comment)]
    pub emit_mir: bool,

    /// Also emit LL (LLVM) files when building.
    #[arg(long = "llvm-ir", verbatim_doc_comment)]
    pub emit_llvm_ir: bool,

    #[arg(
        long = "no-imports",
        help = "Skips extracting the EI imports after building the contracts.",
        action = ArgAction::SetFalse,
    )]
    pub extract_imports: bool,

    /// For the wasm crate, allows specifying the target directory where the Rust compiler will build the intermediary files.
    /// Sharing the same target directory can speed up building multiple contract crates at once.
    /// Has alias `target-dir` for backwards compatibility.
    #[arg(long = "target-dir-wasm", alias = "target-dir", verbatim_doc_comment)]
    pub target_dir_wasm: Option<String>,

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
            locked: false,
            wasm_symbols: false,
            wasm_name_override: None,
            wasm_name_suffix: None,
            wasm_opt: true,
            wat: false,
            emit_mir: false,
            emit_llvm_ir: false,
            extract_imports: true,
            target_dir_wasm: None,
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
    pub fn has_twiggy_call(&self) -> bool {
        self.twiggy_top || self.twiggy_paths || self.twiggy_monos || self.twiggy_dominators
    }
}

impl CliArgsToRaw for BuildArgs {
    fn to_raw(&self) -> Vec<String> {
        let mut raw = Vec::new();
        if self.locked {
            raw.push("--locked".to_string());
        }
        if self.wasm_symbols {
            raw.push("--wasm-symbols".to_string());
        }
        if let Some(wasm_name_override) = &self.wasm_name_override {
            raw.push("--wasm-name".to_string());
            raw.push(wasm_name_override.clone())
        }
        if let Some(wasm_name_suffix) = &self.wasm_name_suffix {
            raw.push("--wasm-suffix".to_string());
            raw.push(wasm_name_suffix.clone())
        }
        if !self.wasm_opt {
            raw.push("--no-wasm-opt".to_string());
        }
        if self.wat {
            raw.push("--wat".to_string());
        }
        if self.emit_mir {
            raw.push("--mir".to_string());
        }
        if self.emit_llvm_ir {
            raw.push("--llvm-ir".to_string());
        }
        if !self.extract_imports {
            raw.push("--no-imports".to_string());
        }
        if let Some(target_dir_wasm) = &self.target_dir_wasm {
            // not using --target-dir-wasm, for backward compatibility
            raw.push("--target-dir".to_string());
            raw.push(target_dir_wasm.clone());
        }
        if self.twiggy_top {
            raw.push("--twiggy-top".to_string());
        }
        if self.twiggy_paths {
            raw.push("--twiggy-paths".to_string());
        }
        if self.twiggy_monos {
            raw.push("--twiggy-monos".to_string());
        }
        if self.twiggy_dominators {
            raw.push("--twiggy-dominators".to_string());
        }
        raw
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct BuildDbgArgs {
    /// For the wasm crate, allows specifying the target directory where the Rust compiler will build the intermediary files.
    /// Sharing the same target directory can speed up building multiple contract crates at once.
    /// Has alias `target-dir` for backwards compatibility.
    #[arg(long = "target-dir-wasm", alias = "target-dir", verbatim_doc_comment)]
    pub target_dir_wasm: Option<String>,

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
}

impl BuildDbgArgs {
    /// Base config when calling `cargo run build-dbg`, with no additional configs.
    pub fn into_build_args(self) -> BuildArgs {
        BuildArgs {
            wasm_symbols: true,
            wasm_name_override: None,
            wasm_name_suffix: Some("dbg".to_string()),
            wasm_opt: false,
            wat: true,
            extract_imports: false,
            target_dir_wasm: self.target_dir_wasm,
            twiggy_top: self.twiggy_top,
            twiggy_paths: self.twiggy_paths,
            twiggy_monos: self.twiggy_monos,
            twiggy_dominators: self.twiggy_dominators,
            ..BuildArgs::default()
        }
    }
}

impl CliArgsToRaw for BuildDbgArgs {
    fn to_raw(&self) -> Vec<String> {
        let mut raw = Vec::new();
        if let Some(target_dir_wasm) = &self.target_dir_wasm {
            // not using --target-dir-wasm, for backward compatibility
            raw.push("--target-dir".to_string());
            raw.push(target_dir_wasm.clone());
        }
        if self.twiggy_top {
            raw.push("--twiggy-top".to_string());
        }
        if self.twiggy_paths {
            raw.push("--twiggy-paths".to_string());
        }
        if self.twiggy_monos {
            raw.push("--twiggy-monos".to_string());
        }
        if self.twiggy_dominators {
            raw.push("--twiggy-dominators".to_string());
        }
        raw
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct TwiggyArgs {
    /// For the wasm crate, allows specifying the target directory where the Rust compiler will build the intermediary files.
    /// Sharing the same target directory can speed up building multiple contract crates at once.
    /// Has alias `target-dir` for backwards compatibility.
    #[arg(long = "target-dir-wasm", alias = "target-dir", verbatim_doc_comment)]
    pub target_dir_wasm: Option<String>,
}

impl TwiggyArgs {
    pub fn into_build_args(self) -> BuildArgs {
        BuildDbgArgs {
            target_dir_wasm: self.target_dir_wasm,
            twiggy_top: true,
            twiggy_paths: true,
            twiggy_monos: true,
            twiggy_dominators: true,
        }
        .into_build_args()
    }
}

impl CliArgsToRaw for TwiggyArgs {
    fn to_raw(&self) -> Vec<String> {
        let mut raw = Vec::new();
        if let Some(target_dir) = &self.target_dir_wasm {
            // not using --target-dir-wasm, for backward compatibility
            raw.push("--target-dir".to_string());
            raw.push(target_dir.clone());
        }
        raw
    }
}
