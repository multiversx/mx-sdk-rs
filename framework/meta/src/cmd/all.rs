use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};
use petgraph::{
    graph::NodeIndex,
    prelude::StableGraph,
    visit::{Dfs, EdgeRef},
    Direction,
};

use super::print_util::{print_all_command, print_all_count, print_all_index};
use crate::{
    cli::AllArgs,
    folder_structure::{dir_pretty_print, RelevantDirectories},
};
use std::{
    collections::{HashMap, HashSet},
    env,
    ffi::OsString,
    path::Path,
    process::{Command, Stdio},
};

pub struct Graph {
    pub graph: StableGraph<Package, DependencyKind>,
    pub nodes: HashMap<PackageId, NodeIndex>,
    pub root: Option<PackageId>,
}

pub fn call_all_meta(args: &AllArgs) {
    let path = if let Some(some_path) = &args.path {
        Path::new(some_path)
    } else {
        Path::new("./")
    };

    perform_call_all_meta(path, args.ignore.as_slice(), args.to_cargo_run_args());
}

fn perform_call_all_meta(path: &Path, ignore: &[String], raw_args: Vec<String>) {
    let dirs = RelevantDirectories::find_all(path, ignore);

    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let rustc_output = output(Command::new(rustc).arg("-Vv"));

    let default_target = rustc_output
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(str::trim))
        .map(str::to_string)
        .unwrap_or_default();

    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let mut command = Command::new(cargo);
    command
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--all-features")
        .arg("--filter-platform")
        .arg(default_target);

    let metadata_command = output(&mut command);

    let metadata: Metadata =
        serde_json::from_str(&metadata_command).expect("unable to parse metadata");

    let resolve = metadata.resolve.unwrap();

    let mut graph = Graph {
        graph: StableGraph::new(),
        nodes: HashMap::new(),
        root: resolve.root,
    };

    for package in metadata.packages {
        let id = package.id.clone();
        let index = graph.graph.add_node(package);
        graph.nodes.insert(id, index);
    }

    for node in resolve.nodes {
        let from = graph.nodes[&node.id];
        for dep in node.deps {
            let mut kinds = vec![];
            for kind in dep.dep_kinds {
                if !kinds.iter().any(|k| *k == kind.kind) {
                    kinds.push(kind.kind);
                }
            }

            let to = graph.nodes[&dep.pkg];
            for kind in kinds {
                graph.graph.add_edge(from, to, kind);
            }
        }
    }

    // prune nodes not reachable from the root package (directionally)
    if let Some(root) = &graph.root {
        let mut dfs = Dfs::new(&graph.graph, graph.nodes[root]);
        while dfs.next(&graph.graph).is_some() {}

        let g = &mut graph.graph;
        graph.nodes.retain(|_, idx| {
            if !dfs.discovered.contains(idx.index()) {
                g.remove_node(*idx);
                false
            } else {
                true
            }
        });
    }

    let root = graph.root.as_ref().unwrap();
    let root = &graph.graph[graph.nodes[root]];

    let direction = Direction::Outgoing;

    check_tree(&graph, root, direction);

    dir_pretty_print(dirs.iter_contract_crates(), "", &|_| {});

    let num_contract_crates = dirs.iter_contract_crates().count();
    print_all_count(num_contract_crates);

    if dirs.is_empty() {
        return;
    }

    for (i, contract_crate) in dirs.iter_contract_crates().enumerate() {
        print_all_index(i + 1, num_contract_crates);
        call_contract_meta(contract_crate.path.as_path(), raw_args.as_slice());
    }
}

pub fn call_contract_meta(contract_crate_path: &Path, cargo_run_args: &[String]) {
    let meta_path = contract_crate_path.join("meta");
    assert!(
        meta_path.exists(),
        "Contract meta crate not found at {}",
        meta_path.as_path().display()
    );

    print_all_command(meta_path.as_path(), cargo_run_args);

    let exit_status = Command::new("cargo")
        .current_dir(&meta_path)
        .args(cargo_run_args)
        .spawn()
        .expect("failed to spawn cargo run process in meta crate")
        .wait()
        .expect("cargo run process in meta crate was not running");

    assert!(exit_status.success(), "contract meta process failed");
}

fn output(command: &mut Command) -> String {
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        panic!("failed");
    }

    String::from_utf8(output.stdout).expect("error parsing output")
}

fn check_tree<'a>(graph: &'a Graph, root: &'a Package, direction: Direction) {
    let mut visited_deps = HashSet::new();
    let mut levels_continue = vec![];

    check_package(
        graph,
        root,
        direction,
        &mut visited_deps,
        &mut levels_continue,
    );
}

fn check_package<'a>(
    graph: &'a Graph,
    package: &'a Package,
    direction: Direction,
    visited_deps: &mut HashSet<&'a PackageId>,
    levels_continue: &mut Vec<bool>,
) {
    let new = visited_deps.insert(&package.id);

    println!("{}", package.name);

    if !new {
        return;
    }

    for kind in &[
        DependencyKind::Normal,
        DependencyKind::Build,
        DependencyKind::Development,
    ] {
        check_dependencies(
            graph,
            package,
            direction,
            visited_deps,
            levels_continue,
            *kind,
        );
    }
}

fn check_dependencies<'a>(
    graph: &'a Graph,
    package: &'a Package,
    direction: Direction,
    visited_deps: &mut HashSet<&'a PackageId>,
    levels_continue: &mut Vec<bool>,
    kind: DependencyKind,
) {
    let idx = graph.nodes[&package.id];
    let mut deps = vec![];
    for edge in graph.graph.edges_directed(idx, direction) {
        if *edge.weight() != kind {
            continue;
        }

        let dep = match direction {
            Direction::Incoming => &graph.graph[edge.source()],
            Direction::Outgoing => &graph.graph[edge.target()],
        };
        deps.push(dep);
    }

    if deps.is_empty() {
        return;
    }

    // ensure a consistent output ordering
    deps.sort_by_key(|p| &p.id);

    let name = match kind {
        DependencyKind::Normal => None,
        DependencyKind::Build => Some("[build-dependencies]"),
        DependencyKind::Development => Some("[dev-dependencies]"),
        _ => unreachable!(),
    };

    // println!("{}", name.unwr());

    // if let Prefix::Indent = prefix {
    if let Some(name) = name {
        //         for continues in &**levels_continue {
        //             let c = if *continues { symbols.down } else { " " };
        //             print!("{}   ", c);
        //         }

        println!("{}", name);
    }
    // }

    let mut it = deps.iter().peekable();
    while let Some(dependency) = it.next() {
        levels_continue.push(it.peek().is_some());
        check_package(graph, dependency, direction, visited_deps, levels_continue);
        levels_continue.pop();
    }
}
