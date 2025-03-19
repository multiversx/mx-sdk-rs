use std::{path::Path, process};

use multiversx_sc_meta_lib::version_history::{validate_template_tag, VERSIONS};

use crate::cmd::code_report::generate_report::{JSON, MD};

use super::{
    AccountArgs, CodeReportAction, CodeReportArgs, CompareArgs, CompileArgs, ConvertArgs,
    InstallArgs, TemplateArgs, TemplateListArgs, UpgradeArgs,
};

impl TemplateArgs {
    pub fn validate_args(&self) {
        if let Some(name) = &self.name {
            if !validate_contract_name(name) {
                user_error(&format!(
                    "Invalid contract name `{}`: Rust crate names must start with a letter or underscore and contain only letters, numbers, and underscores (_). Dots (.) and dashes (-) are not allowed.",
                    name
                ));
            }
        }

        if let Some(tag) = &self.tag {
            if !validate_template_tag(tag) {
                user_error(&format!("Invalid template tag `{}`.", tag));
            }
        }
    }
}

impl InstallArgs {
    pub fn validate_args(&self) {
        if self.command.is_none() {
            user_error("Command expected after `install`");
        }
    }
}

impl TemplateListArgs {
    pub fn validate_args(&self) {
        if let Some(tag) = &self.tag {
            if !validate_template_tag(tag) {
                user_error(&format!("Invalid template tag `{}`.", tag));
            }
        }
    }
}

impl CompileArgs {
    pub fn validate_args(&self) {
        if !matches_extension(&self.output, JSON) && !matches_extension(&self.output, MD) {
            user_error("Create report is only available for Markdown or JSON output file.");
        }
    }
}

impl ConvertArgs {
    pub fn validate_args(&self) {
        if !matches_extension(&self.output, MD) {
            user_error("Conversion output is only available for Markdown file extension");
        }

        if !matches_extension(&self.input, JSON) {
            user_error("Conversion only available from JSON file extension");
        }
    }
}

impl CompareArgs {
    pub fn validate_args(&self) {
        if !matches_extension(&self.output, MD) {
            user_error("Compare output is only available for Markdown file extension.");
        }

        if !matches_extension(&self.baseline, JSON) && !matches_extension(&self.new, JSON) {
            user_error("Compare baseline and new are only available for JSON file extension.");
        }
    }
}

impl CodeReportArgs {
    pub fn validate_args(&self) {
        match &self.command {
            CodeReportAction::Compile(compile_args) => compile_args.validate_args(),
            CodeReportAction::Compare(compare_args) => compare_args.validate_args(),
            CodeReportAction::Convert(convert_args) => convert_args.validate_args(),
        }
    }
}

impl AccountArgs {
    pub fn validate_args(&self) {
        if self.api.is_none() {
            user_error("API needs to be specified");
        }
    }
}

impl UpgradeArgs {
    pub fn validate_args(&self) {
        if let Some(override_target_v) = &self.override_target_version {
            if !VERSIONS.iter().any(|v| v.to_string() == *override_target_v) {
                user_error(&format!("Invalid requested version: {}", override_target_v));
            }
        }
    }
}

// helpers
pub(crate) fn validate_contract_name(name: &str) -> bool {
    let mut chars = name.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

pub fn matches_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == extension)
        .unwrap_or(false)
}

pub(crate) fn user_error(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
