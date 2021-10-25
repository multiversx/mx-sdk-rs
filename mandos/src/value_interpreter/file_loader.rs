use std::{fs, path::PathBuf};

use crate::interpret_trait::InterpreterContext;

pub fn load_file(file_path: &str, context: &InterpreterContext) -> Vec<u8> {
    let mut path_buf = PathBuf::new();
    path_buf.push(context.context_path.as_str());
    path_buf.push(file_path);
    fs::read(&path_buf).unwrap_or_else(|_| missing_file_value(&path_buf))
}

fn missing_file_value(path_buf: &PathBuf) -> Vec<u8> {
    let expr_str = format!("MISSING:{:?}", path_buf);
    expr_str.into_bytes()
}
