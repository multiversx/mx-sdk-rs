use crate::{
    cli_args::LocalDepsArgs,
    folder_structure::{
        dir_pretty_print, RelevantDirectories, CARGO_TOML_FILE_NAME, FRAMEWORK_CRATE_NAMES,
    },
    CargoTomlContents,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, LinkedList},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalDeps {
    pub dependencies: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalDep {
    pub path: String,
}

pub fn local_deps(args: &LocalDepsArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    perform_local_deps(path, args.ignore.as_slice());
}

fn perform_local_deps(path: impl AsRef<Path>, ignore: &[String]) {
    let path_ref = path.as_ref();
    let dirs = RelevantDirectories::find_all(path_ref, ignore);
    dir_pretty_print(dirs.iter_contract_crates(), "", &|_| {});

    let mut unique_paths = HashSet::new();

    for contract_dir in dirs.iter_contract_crates() {
        let local_paths = contract_dir
            .cargo_toml_contents()
            .expect("error retrieving contract Cargo.toml file")
            .local_dependency_paths(FRAMEWORK_CRATE_NAMES);

        let output_dir_path = contract_dir.path.join("output");
        std::fs::create_dir_all(&output_dir_path).unwrap();

        for local_path in local_paths {
            let full_path = contract_dir.path.join(&local_path).canonicalize().unwrap();
            unique_paths.insert(full_path.clone());
        }

        expand_deps(&mut unique_paths);

        let mut deps_contents = LocalDeps::default();
        for path in &unique_paths {
            deps_contents
                .dependencies
                .push(path.to_string_lossy().to_string());
        }
        let mut deps_file = File::create(output_dir_path.join("local_deps.txt")).unwrap();
        writeln!(deps_file, "{}", serialize_local_deps_json(&deps_contents)).unwrap();
    }
}

fn expand_deps(unique_paths: &mut HashSet<PathBuf>) {
    let mut queue: LinkedList<PathBuf> = unique_paths.iter().cloned().collect();
    while let Some(first) = queue.pop_front() {
        let cargo_toml_path = first.join(CARGO_TOML_FILE_NAME);
        let cargo_toml_contents = CargoTomlContents::load_from_file(cargo_toml_path);
        let local_paths = cargo_toml_contents.local_dependency_paths(FRAMEWORK_CRATE_NAMES);
        for local_path in &local_paths {
            let full_path = first.join(local_path).canonicalize().unwrap();
            if !unique_paths.contains(&full_path) {
                // println!("from expand {}", full_path.display());
                queue.push_back(full_path);
            }
        }
    }
}

fn serialize_local_deps_json(deps_contents: &LocalDeps) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    deps_contents.serialize(&mut ser).unwrap();
    let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
    serialized.push('\n');
    serialized
}
