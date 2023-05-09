use std::{
    fs,
    path::{Component, Path, PathBuf},
};
use serde::{Deserialize, Serialize};


use crate::interpret_trait::InterpreterContext;

#[derive(Serialize, Deserialize)]
pub struct MxscFileJson {
    pub code: String,
}

pub fn load_file(file_path: &str, context: &InterpreterContext) -> Vec<u8> {
    let mut path_buf = context.context_path.clone();
    path_buf.push(file_path);
    path_buf = normalize_path(path_buf);
    fs::read(&path_buf).unwrap_or_else(|_| {
        if context.allow_missing_files {
            missing_file_value(&path_buf)
        } else {
            panic!("not found: {path_buf:#?}")
        }
    })
}

pub fn load_mxsc_file_json(mxsc_file_path: &str, context: &InterpreterContext) -> Vec<u8> {
    let mxsc_json_file = load_file(mxsc_file_path, context);
    let mxsc_json_file = match std::str::from_utf8(&mxsc_json_file) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let mxsc_json: MxscFileJson = serde_json::from_str(mxsc_json_file).unwrap();
    mxsc_json.code.into()
}

fn missing_file_value(path_buf: &Path) -> Vec<u8> {
    let expr_str = format!("MISSING:{path_buf:?}");
    expr_str.into_bytes()
}

/// Improve the path to try remove and solve .. token.
///
/// This assumes that `a/b/../c` is `a/c` which might be different from
/// what the OS would have chosen when b is a link. This is OK
/// for broot verb arguments but can't be generally used elsewhere
///
/// This function ensures a given path ending with '/' still
/// ends with '/' after normalization.
///
/// Source: https://stackoverflow.com/questions/68231306/stdfscanonicalize-for-files-that-dont-exist
fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let ends_with_slash = path.as_ref().to_str().map_or(false, |s| s.ends_with('/'));
    let mut normalized = PathBuf::new();
    for component in path.as_ref().components() {
        match &component {
            Component::ParentDir => {
                if !normalized.pop() {
                    normalized.push(component);
                }
            },
            _ => {
                normalized.push(component);
            },
        }
    }
    if ends_with_slash {
        normalized.push("");
    }
    normalized
}
