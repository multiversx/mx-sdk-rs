use crate::cmd::template::RepoSource;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

pub const PATH_TO_SCRIPT: &str = "tools/rust-debugger/pretty-printers";
pub const SCRIPT_NAME: &str = "multiversx_sc_lldb_pretty_printers.py";

pub const TARGET_PATH: &str = ".vscode/extensions/";

pub fn install_debugger(custom_path: Option<PathBuf>) {
    let testing = custom_path.is_some();
    let _ = install_lldb_extension();
    install_script(custom_path);
    if !testing {
        // if we are testing we skip the configuration path, not to mess up with the current vscode configuration
        configure_vscode();
    }
}

fn install_lldb_extension() -> io::Result<()> {
    let extension_id = "vadimcn.vscode-lldb";

    // Run the VSCode command to install the extension
    let status = Command::new("code")
        .arg("--install-extension")
        .arg(extension_id)
        .status()?;

    // Check if the command was successful
    if status.success() {
        println!("Extension {} installed successfully.", extension_id);
    } else {
        eprintln!("Failed to install extension {}.", extension_id);
    }

    Ok(())
}

fn install_script(custom_path: Option<PathBuf>) {
    let repo_temp_download = RepoSource::download_from_github(
        crate::cmd::template::RepoVersion::Master,
        std::env::temp_dir(),
    );

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

    let target_path = if custom_path.is_none() {
        home::home_dir().unwrap().join(TARGET_PATH)
    } else {
        custom_path.unwrap()
    };
    let _ = fs::create_dir_all(&target_path);
    if let Ok(_) = fs::copy(&script, get_script_path(target_path)) {
        println!("debugger script imported successfully");
    }
}

fn get_script_path(path: PathBuf) -> PathBuf {
    path.join(SCRIPT_NAME)
}

fn configure_vscode() {
    let os = env::consts::OS;
    let user_home = home::home_dir().unwrap();

    let path_to_settings = match os {
        "macos" => {
            // For macOS
            Path::new(&user_home)
                .join("Library")
                .join("Application Support")
                .join("Code")
                .join("User")
                .join("settings.json")
        },
        "linux" => {
            // For Linux
            Path::new(&user_home)
                .join(".config")
                .join("Code")
                .join("User")
                .join("settings.json")
        },
        _ => panic!("OS not supported"),
    };

    let script_full_path = get_script_path(home::home_dir().unwrap().join(TARGET_PATH));
    let json = fs::read_to_string(&path_to_settings).expect("Unable to read settings.json");
    let mut sub_values: serde_json::Value = serde_json::from_str(&json).unwrap();

    let init_commands = sub_values
        .as_object_mut()
        .unwrap()
        .entry("lldb.launch.initCommands")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));

    if let serde_json::Value::Array(ref mut array) = init_commands {
        array.clear();
        array.push(serde_json::Value::String(
            script_full_path.to_str().unwrap().to_string(),
        ));
    }

    let _ = fs::write(
        path_to_settings,
        serde_json::to_string(&sub_values).unwrap(),
    );

    println!("debugger script installed successfully");
}
