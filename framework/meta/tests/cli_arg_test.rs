use mx_sc_meta::cli_args::{BuildArgs, CliAction, CliArgs, GenerateSnippetsArgs};

#[test]
fn test_parse_args_nothing() {
    assert_eq!(
        CliArgs::parse(&[""]).unwrap(),
        CliArgs {
            action: CliAction::Nothing,
            load_abi_git_version: true
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "--no-abi-git-version"]).unwrap(),
        CliArgs {
            action: CliAction::Nothing,
            load_abi_git_version: false,
        }
    );
}

#[test]
fn test_parse_args_build() {
    assert_eq!(
        CliArgs::parse(&["", "build"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs::default()),
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "build", "--no-abi-git-version"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs::default()),
            load_abi_git_version: false,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "build", "--wasm-symbols"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
                debug_symbols: true,
                ..Default::default()
            }),
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "build", "--wasm-name", "custom-name", "--no-imports"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
                wasm_name_override: Some("custom-name".to_string()),
                extract_imports: false,
                ..Default::default()
            }),
            load_abi_git_version: true,
        }
    );
}

#[test]
fn test_parse_args_build_dbg() {
    assert_eq!(
        CliArgs::parse(&["", "build-dbg"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
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
                twiggy_dominators: false
            }),
            load_abi_git_version: true,
        }
    );
}

#[test]
fn test_parse_args_twiggy() {
    assert_eq!(
        CliArgs::parse(&["", "twiggy"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
                debug_symbols: true,
                wasm_name_override: None,
                wasm_name_suffix: Some("dbg".to_string()),
                wasm_opt: false,
                wat: true,
                extract_imports: false,
                target_dir: None,
                twiggy_top: true,
                twiggy_paths: true,
                twiggy_monos: true,
                twiggy_dominators: true,
            }),
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "build", "--twiggy-top"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
                twiggy_top: true,
                ..BuildArgs::default()
            }),
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "build", "--twiggy-paths"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
                twiggy_paths: true,
                ..BuildArgs::default()
            }),
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "build", "--twiggy-monos"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
                twiggy_monos: true,
                ..BuildArgs::default()
            }),
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "build", "--twiggy-dominators"]).unwrap(),
        CliArgs {
            action: CliAction::Build(BuildArgs {
                twiggy_dominators: true,
                ..BuildArgs::default()
            }),
            load_abi_git_version: true,
        }
    );
}

#[test]
fn test_parse_args_clean() {
    assert_eq!(
        CliArgs::parse(&["", "clean"]).unwrap(),
        CliArgs {
            action: CliAction::Clean,
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "clean", "--no-abi-git-version"]).unwrap(),
        CliArgs {
            action: CliAction::Clean,
            load_abi_git_version: false,
        }
    );
}

#[test]
fn test_parse_args_generate_snippets() {
    assert_eq!(
        CliArgs::parse(&["", "snippets"]).unwrap(),
        CliArgs {
            action: CliAction::GenerateSnippets(GenerateSnippetsArgs { overwrite: false }),
            load_abi_git_version: true,
        }
    );

    assert_eq!(
        CliArgs::parse(&["", "snippets", "--overwrite"]).unwrap(),
        CliArgs {
            action: CliAction::GenerateSnippets(GenerateSnippetsArgs { overwrite: true }),
            load_abi_git_version: true,
        }
    );
}

#[test]
fn test_parse_args_error() {
    assert_eq!(
        CliArgs::parse(&["", "no-command"]),
        Err("unknown command: no-command".to_string())
    );

    assert_eq!(
        CliArgs::parse(&["", "clean", "--extra"]),
        Err("clean accepts no arguments".to_string())
    );

    assert_eq!(
        CliArgs::parse(&["", "snippets", "--unknown"]),
        Err("unknown snippets argument: --unknown".to_string())
    );
}