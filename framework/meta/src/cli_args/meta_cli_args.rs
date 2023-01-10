use super::BuildArgs;

pub type CliArgsParseError = String;

/// Parsed arguments of the meta crate CLI.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct CliArgs {
    pub action: CliAction,
    pub load_abi_git_version: bool,
}

impl CliArgs {
    pub fn parse<S>(args: &[S]) -> Result<Self, CliArgsParseError>
    where
        S: AsRef<str>,
    {
        let mut no_abi_git_version = false;
        let mut remaining_args = Vec::<&S>::new();
        for arg in args {
            if arg.as_ref() == "--no-abi-git-version" {
                no_abi_git_version = true;
            } else {
                remaining_args.push(arg);
            }
        }
        Ok(CliArgs {
            action: CliAction::parse(remaining_args.as_slice())?,
            load_abi_git_version: !no_abi_git_version,
        })
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub enum CliAction {
    #[default]
    Nothing,
    Build(BuildArgs),
    Clean,
    GenerateSnippets(GenerateSnippetsArgs),
}

impl CliAction {
    pub fn parse<S>(args: &[S]) -> Result<Self, CliArgsParseError>
    where
        S: AsRef<str>,
    {
        if args.len() < 2 {
            return Ok(CliAction::Nothing);
        }

        let command = args[1].as_ref();
        let additional_args = &args[2..];
        match command {
            "build" => Ok(CliAction::Build(BuildArgs::parse(additional_args)?)),
            "build-dbg" => Ok(CliAction::Build(BuildArgs::parse_dbg(additional_args)?)),
            "twiggy" => Ok(CliAction::Build(BuildArgs::parse_twiggy(additional_args)?)),
            "clean" => {
                if additional_args.is_empty() {
                    Ok(CliAction::Clean)
                } else {
                    Err("clean accepts no arguments".to_string())
                }
            },
            "snippets" => Ok(CliAction::GenerateSnippets(GenerateSnippetsArgs::parse(
                additional_args,
            )?)),
            other => Err(format!("unknown command: {other}")),
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct GenerateSnippetsArgs {
    pub overwrite: bool,
}

impl GenerateSnippetsArgs {
    #[allow(clippy::while_let_on_iterator)]
    pub fn parse<S>(args: &[S]) -> Result<Self, CliArgsParseError>
    where
        S: AsRef<str>,
    {
        let mut result = GenerateSnippetsArgs::default();
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            match arg.as_ref() {
                "--overwrite" => {
                    result.overwrite = true;
                },
                other => return Err(format!("unknown snippets argument: {other}")),
            }
        }
        Ok(result)
    }
}
