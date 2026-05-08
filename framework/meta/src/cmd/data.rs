use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use serde::Serialize;

use crate::cli::{DataAction, DataArgs, DataLoadArgs, DataParseArgs, DataStoreArgs};

/// Name of the data storage file — matches mxpy for cross-tool compatibility.
const STORAGE_FILE: &str = "sc-meta.data-storage.json";

/// Global storage lives in ~/multiversx-sdk, same as mxpy.
fn global_storage_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join("multiversx-sdk").join(STORAGE_FILE)
}

fn local_storage_path() -> PathBuf {
    PathBuf::from(STORAGE_FILE)
}

fn storage_path(use_global: bool) -> PathBuf {
    if use_global {
        global_storage_path()
    } else {
        local_storage_path()
    }
}

pub fn data_cli(args: &DataArgs) {
    let result = match &args.command {
        DataAction::Store(a) => cmd_store(a),
        DataAction::Load(a) => cmd_load(a),
        DataAction::Parse(a) => cmd_parse(a),
    };
    if let Err(e) = result {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn cmd_store(args: &DataStoreArgs) -> Result<()> {
    let path = storage_path(args.use_global);
    let mut data = load_storage(&path)?;
    data.entry(args.partition.clone())
        .or_default()
        .insert(args.key.clone(), args.value.clone());
    save_storage(&path, &data)
}

fn cmd_load(args: &DataLoadArgs) -> Result<()> {
    let path = storage_path(args.use_global);
    let data = load_storage(&path)?;
    let value = data
        .get(&args.partition)
        .and_then(|p| p.get(&args.key))
        .map(|s| s.as_str())
        .unwrap_or("");
    print!("{value}");
    Ok(())
}

fn cmd_parse(args: &DataParseArgs) -> Result<()> {
    let raw = fs::read_to_string(&args.file)
        .with_context(|| format!("failed to read {}", args.file.display()))?;
    let json: serde_json::Value =
        serde_json::from_str(&raw).context("failed to parse JSON file")?;

    let value = eval_expression(&json, &args.expression)?;
    print!("{value}");
    Ok(())
}

/// Evaluate a Python-style dict-access expression against a JSON value.
///
/// Supports chains of `['key']` lookups, e.g. `data['foo']['bar']`.
/// The root object is referred to as `data`.
fn eval_expression(root: &serde_json::Value, expression: &str) -> Result<String> {
    let expr = expression.trim();
    let rest = expr
        .strip_prefix("data")
        .ok_or_else(|| anyhow!("expression must start with 'data', got: {expr}"))?;

    let mut current = root;
    let mut remaining = rest;

    while !remaining.is_empty() {
        // Expect `['key']`
        let after_bracket = remaining
            .strip_prefix("['")
            .ok_or_else(|| anyhow!("unexpected expression fragment: {remaining}"))?;
        let close = after_bracket
            .find("']")
            .ok_or_else(|| anyhow!("unclosed ['  in expression: {remaining}"))?;
        let key = &after_bracket[..close];
        remaining = &after_bracket[close + 2..]; // skip ']

        current = current
            .get(key)
            .ok_or_else(|| anyhow!("key '{key}' not found in JSON"))?;
    }

    match current {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Null => Ok(String::new()),
        other => Ok(other.to_string()),
    }
}

type StorageMap = HashMap<String, HashMap<String, String>>;

fn load_storage(path: &Path) -> Result<StorageMap> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
}

fn save_storage(path: &Path, data: &StorageMap) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory {}", parent.display()))?;
        }
    }
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    data.serialize(&mut ser)
        .context("failed to serialize storage")?;
    fs::write(path, buf).with_context(|| format!("failed to write {}", path.display()))
}
