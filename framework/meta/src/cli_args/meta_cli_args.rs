use super::BuildArgs;

pub type CliArgsParseError = String;

#[derive(Default)]
pub struct CliArgs {
    pub action: CliAction,
    pub load_abi_git_version: bool,
}

impl CliArgs {
    pub fn parse(args: &[String]) -> Result<Self, CliArgsParseError> {
        let no_abi_git_version = args.iter().any(|arg| arg == "--no-abi-git-version");
        Ok(CliArgs {
            action: CliAction::parse(args)?,
            load_abi_git_version: !no_abi_git_version,
        })
    }
}

#[derive(Default)]
pub enum CliAction {
    #[default]
    Nothing,
    Build(BuildArgs),
    Clean,
    GenerateSnippets(GenerateSnippetsArgs),
}

impl CliAction {
    pub fn parse(args: &[String]) -> Result<Self, CliArgsParseError> {
        if args.len() < 2 {
            return Ok(CliAction::Nothing);
        }

        let command = args[1].as_str();
        let additional_args = &args[2..];
        match command {
            "build" => Ok(CliAction::Build(BuildArgs::parse(additional_args)?)),
            "build-dbg" => Ok(CliAction::Build(BuildArgs::parse_dbg(additional_args)?)),
            "clean" => Ok(CliAction::Clean),
            "snippets" => Ok(CliAction::GenerateSnippets(GenerateSnippetsArgs::parse(
                additional_args,
            )?)),
            other => Err(format!("unknown command: {other}")),
        }
    }
}

#[derive(Default)]
pub struct GenerateSnippetsArgs {
    pub overwrite: bool,
}

impl GenerateSnippetsArgs {
    #[allow(clippy::while_let_on_iterator)]
    pub fn parse(args: &[String]) -> Result<Self, CliArgsParseError> {
        let mut result = GenerateSnippetsArgs::default();
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--overwrite" => {
                    result.overwrite = true;
                },
                other => return Err(format!("unknown snippets argument: {other}")),
            }
        }
        Ok(result)
    }
}
