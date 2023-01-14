use std::path::Path;

use multiversx_sc_meta::output_contract::CargoTomlContents;
use toml::Value;

use crate::upgrade_0_39::upgrade_39;

pub fn upgrade_sc(sc_crate_path: impl AsRef<Path>) {
    let version = find_framework_version(sc_crate_path.as_ref());
    println!("Current version: {}", version);

    match version.as_str() {
        "0.38.0" => {
            upgrade_39(sc_crate_path.as_ref());
        },
        _ => {
            println!("Unsupported version.");
        },
    }
}

fn find_framework_version(sc_crate_path: &Path) -> String {
    let cargo_toml_path = sc_crate_path.clone().join("Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let meta_cargo_toml = CargoTomlContents::load_from_file(cargo_toml_path);
    let deps = meta_cargo_toml.dependencies();

    if let Some(old_base) = deps.get("elrond-wasm") {
        if let Some(Value::String(s)) = old_base.get("version") {
            return s.clone();
        } else {
            panic!("missing or invalid version found for the elrond-wasm dependency")
        }
    }

    if let Some(base) = deps.get("multiversx-sc") {
        if let Some(Value::String(s)) = base.get("version") {
            return s.clone();
        } else {
            panic!("missing or invalid version found for the multiversx-sc dependency")
        }
    }

    panic!("no SC framework dependency found")
}
