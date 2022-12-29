use super::BuildArgs;

/// Parsed arguments of the meta crate CLI.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct CliArgs {
    pub action: CliAction,
    pub load_abi_git_version: bool,
}

impl CliArgs {
    pub fn parse<S>(args: &[S]) -> Self
    where
        S: AsRef<str>,
    {
        let no_abi_git_version = args
            .iter()
            .any(|arg| arg.as_ref() == "--no-abi-git-version");
        CliArgs {
            action: CliAction::parse(args),
            load_abi_git_version: !no_abi_git_version,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
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
    pub fn parse<S>(args: &[S]) -> Self
    where
        S: AsRef<str>,
    {
        if args.len() < 2 {
            return CliAction::Nothing;
        }

        match args[1].as_ref() {
            "build" => CliAction::Build(BuildArgs::parse(&args[2..])),
            "build-dbg" => CliAction::Build(BuildArgs::parse_dbg(&args[2..])),
            "twiggy" => CliAction::Build(BuildArgs::parse_twiggy(&args[2..])),
            "clean" => CliAction::Clean,
            "snippets" => CliAction::GenerateSnippets(GenerateSnippetsArgs::parse(&args[2..])),
            _ => CliAction::Nothing,
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct GenerateSnippetsArgs {
    pub overwrite: bool,
}

impl GenerateSnippetsArgs {
    pub fn parse<S>(args: &[S]) -> Self
    where
        S: AsRef<str>,
    {
        let overwrite = match args.get(0) {
            Some(arg) => arg.as_ref() == "--overwrite",
            None => false,
        };
        GenerateSnippetsArgs { overwrite }
    }
}
