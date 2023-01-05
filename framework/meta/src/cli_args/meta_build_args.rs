use super::CliArgsParseError;

/// `erdpy` still sends unnecessary arguments when building.
///
/// Set to true when the issue has been resolved.
const PARSE_BUILD_ARGS_STRICT: bool = false;

#[derive(PartialEq, Eq, Debug)]
pub struct BuildArgs {
    pub debug_symbols: bool,
    pub wasm_name_override: Option<String>,
    pub wasm_name_suffix: Option<String>,
    pub wasm_opt: bool,
    pub wat: bool,
    pub extract_imports: bool,
    pub target_dir: Option<String>,
    pub twiggy_top: bool,
    pub twiggy_paths: bool,
    pub twiggy_monos: bool,
    pub twiggy_dominators: bool,
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
            twiggy_top: false,
            twiggy_paths: false,
            twiggy_monos: false,
            twiggy_dominators: false,
        }
    }
}

impl BuildArgs {
    /// Base config when calling `cargo run build`, with no additional configs.
    pub fn build_base_config() -> Self {
        Self::default()
    }

    /// Parses all arguments and sets them in a given BuildArgs object.
    ///
    /// Configuring a pre-existing object allows different defaults to be set.
    fn iter_parse<S>(args: &[S], result: &mut BuildArgs) -> Result<(), CliArgsParseError>
    where
        S: AsRef<str>,
    {
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            match arg.as_ref() {
                "--wasm-symbols" => {
                    result.debug_symbols = true;
                },
                "--wasm-name" => {
                    let name = iter
                        .next()
                        .expect("argument `--wasm-name` must be followed by the desired name");
                    result.wasm_name_override = Some(name.as_ref().to_string());
                },
                "--wasm-suffix" => {
                    let suffix = iter
                        .next()
                        .expect("argument `--wasm-suffix` must be followed by the desired suffix");
                    result.wasm_name_suffix = Some(suffix.as_ref().to_string());
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
                    result.target_dir = Some(arg.as_ref().to_string());
                },
                "--twiggy-top" => {
                    result.twiggy_top = true;
                },
                "--twiggy-paths" => {
                    result.twiggy_paths = true;
                },
                "--twiggy-monos" => {
                    result.twiggy_monos = true;
                },
                "--twiggy-dominators" => {
                    result.twiggy_dominators = true;
                },
                other if PARSE_BUILD_ARGS_STRICT => {
                    return Err(format!("unknown build argument: {other}"))
                },
                _ => {},
            }
        }

        Ok(())
    }

    pub fn parse<S>(args: &[S]) -> Result<Self, CliArgsParseError>
    where
        S: AsRef<str>,
    {
        let mut result = BuildArgs::build_base_config();
        BuildArgs::iter_parse(args, &mut result)?;
        Ok(result)
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
            twiggy_top: false,
            twiggy_paths: false,
            twiggy_monos: false,
            twiggy_dominators: false,
        }
    }

    pub fn parse_dbg<S>(args: &[S]) -> Result<Self, CliArgsParseError>
    where
        S: AsRef<str>,
    {
        let mut result = BuildArgs::build_dbg_base_config();
        BuildArgs::iter_parse(args, &mut result)?;
        Ok(result)
    }

    pub fn twiggy_base_config() -> Self {
        BuildArgs {
            twiggy_top: true,
            twiggy_paths: true,
            twiggy_monos: true,
            twiggy_dominators: true,
            ..BuildArgs::build_dbg_base_config()
        }
    }

    pub fn parse_twiggy<S>(args: &[S]) -> Result<Self, CliArgsParseError>
    where
        S: AsRef<str>,
    {
        let mut result = BuildArgs::twiggy_base_config();
        BuildArgs::iter_parse(args, &mut result)?;
        Ok(result)
    }

    pub fn has_twiggy_call(&self) -> bool {
        self.twiggy_top || self.twiggy_paths || self.twiggy_monos || self.twiggy_dominators
    }
}
