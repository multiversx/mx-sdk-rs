use colored::Colorize;

use super::system_info::{SystemInfo, get_system_info};
use crate::cmd::template::RepoSource;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

pub const PATH_TO_SCRIPT: &str = "tools/rust-debugger/pretty-printers";
pub const SCRIPT_NAME: &str = "multiversx_sc_lldb_pretty_printers.py";

pub const TARGET_PATH: &str = ".vscode/extensions/";

pub async fn install_debugger(custom_path: Option<PathBuf>) {
    let testing = custom_path.is_some();
    let _ = install_lldb_extension();
    if get_system_info() == SystemInfo::Windows {
        println!(
            "{}",
            "On Windows, the VS Code window opened by this tool can be safely closed after installation."
                .yellow()
        );
    }
    install_script(custom_path).await;
    if !testing {
        // if we are testing we skip the configuration path, not to mess up with the current vscode configuration
        configure_vscode();
    }
}

// home_dir is deprecated on older version of Rust (1.85 ...), but we are keeping it
// not deprecated on newer versions (1.90 for sure)
#[allow(deprecated)]
fn home_dir() -> PathBuf {
    std::env::home_dir().expect("Could not find home directory")
}

fn install_lldb_extension() -> io::Result<()> {
    let extension_id = "vadimcn.vscode-lldb";

    // Run the VSCode command to install the extension
    let install_lldb_command = Command::new("code")
        .arg("--install-extension")
        .arg(extension_id)
        .arg("--force")
        .status()?;

    if install_lldb_command.success() {
        println!("Extension {} installed successfully.", install_lldb_command);
    } else {
        eprintln!("Failed to install extension {}.", install_lldb_command);
    }

    Ok(())
    // Check if the command was successful
}

async fn install_script(custom_path: Option<PathBuf>) {
    let repo_temp_download = RepoSource::download_from_github(
        crate::cmd::template::RepoVersion::Master,
        std::env::temp_dir(),
    )
    .await;

    let script_path = repo_temp_download.repo_path().join(PATH_TO_SCRIPT);
    let path_ref: &Path = script_path.as_ref();
    let canonicalized = fs::canonicalize(path_ref).unwrap_or_else(|err| {
        panic!(
            "error canonicalizing input path {}: {}",
            path_ref.display(),
            err,
        )
    });
    let script = canonicalized.join(SCRIPT_NAME);

    let target_path = if let Some(unwrapped_custom_path) = custom_path {
        unwrapped_custom_path
    } else {
        home_dir().join(TARGET_PATH)
    };

    let _ = fs::create_dir_all(&target_path);
    if fs::copy(script, get_script_path(target_path)).is_ok() {
        println!("debugger script imported successfully");
    }
}

fn get_script_path(path: PathBuf) -> PathBuf {
    path.join(SCRIPT_NAME)
}

fn get_path_to_settings() -> PathBuf {
    let user_home = home_dir();
    match get_system_info() {
        SystemInfo::MacOs => {
            // For macOS
            Path::new(&user_home)
                .join("Library")
                .join("Application Support")
                .join("Code")
                .join("User")
                .join("settings.json")
        }
        SystemInfo::Linux => {
            // For Linux
            Path::new(&user_home)
                .join(".config")
                .join("Code")
                .join("User")
                .join("settings.json")
        }
        SystemInfo::Windows => {
            // For Windows
            let appdata = env::var("APPDATA").expect("Could not find APPDATA environment variable");
            Path::new(&appdata)
                .join("Code")
                .join("User")
                .join("settings.json")
        }
    }
}

fn configure_vscode() {
    let path_to_settings = get_path_to_settings();

    let script_full_path = get_script_path(home_dir().join(TARGET_PATH));
    let json = match fs::read_to_string(&path_to_settings) {
        Err(_) => {
            eprintln!(
                "{}",
                format!(
                "WARNING: Could not find settings.json at {}. Debugger configuration will be skipped.",
                path_to_settings.display()
            )
                .bright_yellow()
                .bold(),
            );
            return;
        }
        Ok(json) => json,
    };

    let mut sub_values: serde_json::Value = serde_json::from_str(&json).unwrap_or_else(
        |err: serde_json::Error| panic!("Incorrectly formatted VSCode settings.json file. The error is located at line {}, column {}. This error might be caused either by a trailing comma in the settings file (which is, actually, pretty usual), or the settings file was not correctly edited and saved. Please check your file via a JSON linter and fix the settings file before attempting to run the install command again.", err.line(), err.column())
    );

    let init_commands = sub_values
        .as_object_mut()
        .unwrap()
        .entry("lldb.launch.preRunCommands")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    let command_script_line =
        "command script import ".to_owned() + script_full_path.to_str().unwrap();

    if let serde_json::Value::Array(array) = init_commands {
        if let Some(pos) = array.iter().position(|v| {
            if let serde_json::Value::String(s) = v {
                s.contains(SCRIPT_NAME) // Replace with your substring
            } else {
                false
            }
        }) {
            array.remove(pos); // Remove the element if the substring is found
        }

        array.push(serde_json::Value::String(command_script_line));
    }

    let _ = fs::write(
        path_to_settings,
        serde_json::to_string_pretty(&sub_values).unwrap(),
    );

    println!("debugger script installed successfully");
}
