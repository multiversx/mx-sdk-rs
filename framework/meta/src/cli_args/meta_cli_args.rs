use super::BuildArgs;

/// Parsed arguments of the meta crate CLI.
#[derive(Default)]
pub struct CliArgs {
    pub action: CliAction,
    pub load_abi_git_version: bool,
}

impl CliArgs {
    pub fn parse(args: &[String]) -> Self {
        let no_abi_git_version = args.iter().any(|arg| arg == "--no-abi-git-version");
        CliArgs {
            action: CliAction::parse(args),
            load_abi_git_version: !no_abi_git_version,
        }
    }
}

pub enum CliAction {
    Build(BuildArgs),
    Clean,
    GenerateSnippets(GenerateSnippetsArgs),
    Nothing,
}

impl Default for CliAction {
    fn default() -> Self {
        CliAction::Nothing
    }
}

impl CliAction {
    pub fn parse(args: &[String]) -> Self {
        if args.len() < 2 {
            return CliAction::Nothing;
        }

        match args[1].as_str() {
            "build" => CliAction::Build(BuildArgs::parse(&args[2..])),
            "build-dbg" => CliAction::Build(BuildArgs::parse_dbg(&args[2..])),
            "twiggy" => CliAction::Build(BuildArgs::parse_twiggy(&args[2..])),
            "clean" => CliAction::Clean,
            "snippets" => CliAction::GenerateSnippets(GenerateSnippetsArgs::parse(&args[2..])),
            _ => CliAction::Nothing,
        }
    }
}

#[derive(Default)]
pub struct GenerateSnippetsArgs {
    pub overwrite: bool,
}

impl GenerateSnippetsArgs {
    pub fn parse(args: &[String]) -> Self {
        let overwrite = match args.get(2) {
            Some(arg) => arg.as_str() == "--overwrite",
            None => false,
        };
        GenerateSnippetsArgs { overwrite }
    }
}
