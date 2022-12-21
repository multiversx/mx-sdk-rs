#[derive(Debug)]
pub struct BuildArgs {
    pub debug_symbols: bool,
    pub wasm_name_override: Option<String>,
    pub wasm_name_suffix: Option<String>,
    pub wasm_opt: bool,
    pub wat: bool,
    pub extract_imports: bool,
    pub target_dir: Option<String>,
    pub abi_git_version: bool,
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
            abi_git_version: true,
        }
    }
}

impl BuildArgs {
    pub fn process(args: &[String]) -> BuildArgs {
        let mut result = BuildArgs::default();
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
                "--no-abi-git-version" => {
                    result.abi_git_version = false;
                },
                _ => {},
            }
        }

        result
    }
}
