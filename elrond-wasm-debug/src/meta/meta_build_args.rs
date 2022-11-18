#[derive(Debug)]
pub struct BuildArgs {
    pub debug_symbols: bool,
    pub wasm_name_override: Option<String>,
    pub wasm_name_suffix: Option<String>,
    pub wasm_opt: bool,
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
            target_dir: None,
            abi_git_version: true,
        }
    }
}

impl BuildArgs {
    // pub fn wasm_name(&self, contract_metadata: &ContractMetadata) -> String {
    //     if let Some(wasm_name_override) = &self.wasm_name_override {
    //         return wasm_name_override.clone();
    //     }
    //     if let Some(wasm_suffix) = &self.wasm_name_suffix {
    //         format!(
    //             "{}-{}.wasm",
    //             contract_metadata.output_base_name, wasm_suffix
    //         )
    //     } else {
    //         contract_metadata.wasm_output_name()
    //     }
    // }

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
