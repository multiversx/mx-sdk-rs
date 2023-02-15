use crate::{
    cli_args::LocalDepsArgs,
    folder_structure::{
        dir_pretty_print, RelevantDirectories, CARGO_TOML_FILE_NAME, FRAMEWORK_CRATE_NAMES,
    },
    CargoTomlContents,
};
use common_path::common_path_all;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, LinkedList},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalDeps {
    pub root: String,
    pub contract_path: String,
    pub common_dependency_path: Option<String>,
    pub dependencies: Vec<LocalDep>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LocalDep {
    pub path: String,
    pub depth: usize,
}

impl LocalDep {
    fn new(path: &Path, depth: usize) -> Self {
        LocalDep {
            path: path.to_string_lossy().to_string(),
            depth,
        }
    }
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
    let root_path = path.as_ref();
    let dirs = RelevantDirectories::find_all(root_path, ignore);
    dir_pretty_print(dirs.iter_contract_crates(), "", &|_| {});

    for contract_dir in dirs.iter_contract_crates() {
        let mut dep_map = BTreeMap::new();

        let output_dir_path = contract_dir.path.join("output");
        std::fs::create_dir_all(&output_dir_path).unwrap();

        expand_deps(root_path, contract_dir.path.clone(), &mut dep_map);

        let common_dependency_path = common_path_all(dep_map.keys().map(|pbuf| pbuf.as_path()));

        let deps_contents = LocalDeps {
            root: root_path.to_string_lossy().to_string(),
            contract_path: contract_dir.path.to_string_lossy().to_string(),
            common_dependency_path: common_dependency_path
                .map(|pbuf| pbuf.to_string_lossy().to_string()),
            dependencies: dep_map.values().cloned().collect(),
        };

        let mut deps_file = File::create(output_dir_path.join("local_deps.txt")).unwrap();
        writeln!(deps_file, "{}", serialize_local_deps_json(&deps_contents)).unwrap();
    }
}

fn expand_deps(
    root_path: &Path,
    starting_path: PathBuf,
    dep_map: &mut BTreeMap<PathBuf, LocalDep>,
) {
    let mut queue: LinkedList<PathBuf> = LinkedList::new();
    queue.push_back(starting_path);
    while let Some(parent) = queue.pop_front() {
        let cargo_toml_path = parent.join(CARGO_TOML_FILE_NAME);
        let cargo_toml_contents = CargoTomlContents::load_from_file(cargo_toml_path);
        let local_paths = cargo_toml_contents.local_dependency_paths(FRAMEWORK_CRATE_NAMES);
        for child in &local_paths {
            let full_path = parent.join(child).canonicalize().unwrap();
            let child_path = pathdiff::diff_paths(&full_path, root_path).unwrap();
            let parent_depth = if let Some(parent_dep) = dep_map.get(&parent) {
                parent_dep.depth
            } else {
                0
            };
            let child_depth = parent_depth + 1;
            if let Some(local_dep) = dep_map.get_mut(&full_path) {
                if child_depth < local_dep.depth {
                    local_dep.depth = child_depth;
                }
            } else {
                dep_map.insert(
                    full_path.clone(),
                    LocalDep::new(child_path.as_path(), child_depth),
                );
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
