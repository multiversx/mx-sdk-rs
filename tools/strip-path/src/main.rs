use multiversx_sc_meta::folder_structure::strip_path;
use std::path::Path;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    strip_path(Path::new(&root), &["target".to_string()]);
}
