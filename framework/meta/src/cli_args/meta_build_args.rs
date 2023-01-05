use super::CliArgsParseError;

#[derive(Debug)]
pub struct BuildArgs {
    pub debug_symbols: bool,
    pub wasm_name_override: Option<String>,
    pub wasm_name_suffix: Option<String>,
    pub wasm_opt: bool,
    pub wat: bool,
    pub extract_imports: bool,
    pub target_dir: Option<String>,
}

impl Default for BuildArgs {
    fn default() -> Self {
        BuildArgs {
            debug_symbols: false,
            wasm_name_override: None,
            wasm_name_suffix: None,
            wasm_opt: true,
            wat: false,
            extract_imports: true,
            target_dir: None,
        }
    }
}

impl BuildArgs {
    /// Base config when calling `cargo run build`, with no additional configs.
    pub fn build_base_config() -> Self {
        Self::default()
    }

    /// Base config when calling `cargo run build-dbg`, with no additional configs.
    pub fn build_dbg_base_config() -> Self {
        BuildArgs {
            debug_symbols: true,
            wasm_name_override: None,
            wasm_name_suffix: Some("dbg".to_string()),
            wasm_opt: false,
            wat: true,
            extract_imports: false,
            target_dir: None,
        }
    }

    fn iter_parse(args: &[String], result: &mut BuildArgs) -> Result<(), CliArgsParseError> {
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--wasm-symbols" => {
                    result.debug_symbols = true;
                },
                "--wasm-name" => {
                    let name = iter
                        .next()
                        .expect("argument `--wasm-name` must be followed by the desired name");
                    result.wasm_name_override = Some(name.clone());
                },
                "--wasm-suffix" => {
                    let suffix = iter
                        .next()
                        .expect("argument `--wasm-suffix` must be followed by the desired suffix");
                    result.wasm_name_suffix = Some(suffix.clone());
                },
                "--no-wasm-opt" => {
                    result.wasm_opt = false;
                },
                "--wat" => {
                    result.wat = true;
                    result.extract_imports = true;
                },
                "--no-imports" => {
                    result.extract_imports = false;
                },
                "--target-dir" => {
                    let arg = iter
                        .next()
                        .expect("argument `--target-dir` must be followed by argument");
                    result.target_dir = Some(arg.clone());
                },
                other => return Err(format!("unknown build argument: {other}")),
            }
        }

        Ok(())
    }

    pub fn parse(args: &[String]) -> Result<Self, CliArgsParseError> {
        let mut result = BuildArgs::build_base_config();
        BuildArgs::iter_parse(args, &mut result)?;
        Ok(result)
    }

    pub fn parse_dbg(args: &[String]) -> Result<Self, CliArgsParseError> {
        let mut result = BuildArgs::build_dbg_base_config();
        BuildArgs::iter_parse(args, &mut result)?;
        Ok(result)
    }
}
