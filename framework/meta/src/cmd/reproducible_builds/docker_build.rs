use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::cli::DockerBuildArgs;

/// Host directory used for Cargo caches that are shared across Docker runs.
const CARGO_CACHE_BASE: &str = "/tmp/multiversx_sc_meta_builder";

/// Runs the reproducible build inside Docker.
///
/// Mirrors `build_with_docker.py` from `mx-sdk-rust-contract-builder` and
/// `mxpy contract reproducible-build`, but uses the `sc-meta`-based Docker
/// image whose entrypoint is:
///
///   sc-meta reproducible-build local-build --output /output --target-dir /rust/cargo-target-dir
///
/// Volume layout (host → container):
///   <project>           → /project   (source; read-only in practice)
///   <output>            → /output    (artifacts written here)
///   cargo-target-dir    → /rust/cargo-target-dir  (optional cache)
///   cargo-registry      → /rust/registry          (optional cache)
///   cargo-git           → /rust/git               (optional cache)
pub fn docker_build(args: &DockerBuildArgs) {
    check_docker_available();

    let project = resolve_project(args.project.as_deref());
    let output = resolve_output(&project, args.output.as_deref());

    fs::create_dir_all(&output).unwrap();
    let output = output.canonicalize().unwrap();

    // Shared Cargo cache directories — created once, reused across runs.
    let cache_base = Path::new(CARGO_CACHE_BASE);
    let cargo_target = cache_base.join("cargo-target-dir");
    let cargo_registry = cache_base.join("cargo-registry");
    let cargo_git = cache_base.join("cargo-git");
    fs::create_dir_all(&cargo_target).unwrap();
    fs::create_dir_all(&cargo_registry).unwrap();
    fs::create_dir_all(&cargo_git).unwrap();

    let mut cmd = Command::new("docker");
    cmd.arg("run");

    if !args.no_default_platform {
        cmd.args(["--platform", "linux/amd64"]);
    }
    if !args.no_docker_interactive {
        cmd.arg("--interactive");
    }
    if !args.no_docker_tty {
        cmd.arg("--tty");
    }

    #[cfg(unix)]
    if let Some(user_arg) = get_unix_user() {
        cmd.args(["--user", &user_arg]);
    }

    cmd.arg("--rm");

    // Volume mounts
    cmd.args(["--volume", &format!("{}:/project", project.display())]);
    cmd.args(["--volume", &format!("{}:/output", output.display())]);
    cmd.args([
        "--volume",
        &format!("{}:/rust/cargo-target-dir", cargo_target.display()),
    ]);
    cmd.args([
        "--volume",
        &format!("{}:/rust/registry", cargo_registry.display()),
    ]);
    cmd.args(["--volume", &format!("{}:/rust/git", cargo_git.display())]);

    let verbose = if args.cargo_verbose { "true" } else { "false" };
    cmd.args(["--env", &format!("CARGO_TERM_VERBOSE={verbose}")]);

    // Image name
    cmd.arg(&args.docker_image);

    // Entrypoint args — appended after the ENTRYPOINT baked into the image.
    // The image already provides: --output /output --target-dir /rust/cargo-target-dir
    cmd.args(["--path", "/project"]);
    if let Some(contract) = &args.contract {
        cmd.args(["--contract", contract]);
    }
    if args.no_wasm_opt {
        cmd.arg("--no-wasm-opt");
    }
    if let Some(build_root) = &args.build_root {
        cmd.args(["--build-root", build_root]);
    }

    println!("Running: {}", format_command(&cmd));

    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap_or_else(|e| panic!("Failed to launch docker: {e}"));

    if !status.success() {
        panic!(
            "Docker build failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }

    println!("Output written to: {}", output.display());
}

fn check_docker_available() {
    let ok = Command::new("docker")
        .args(["info"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !ok {
        panic!(
            "Docker is not available or the daemon is not running.\n\
             Install Docker and ensure `docker info` succeeds before retrying."
        );
    }
}

fn resolve_project(path: Option<&str>) -> PathBuf {
    let p = path.unwrap_or(".");
    Path::new(p)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
}

fn resolve_output(project: &Path, output: Option<&str>) -> PathBuf {
    match output {
        Some(o) => PathBuf::from(o),
        None => project.join("output-docker"),
    }
}

/// Formats a `Command` as a readable shell string for logging.
fn format_command(cmd: &Command) -> String {
    let prog = cmd.get_program().to_string_lossy().into_owned();
    let args: Vec<String> = cmd
        .get_args()
        .map(|a| {
            let s = a.to_string_lossy();
            if s.contains(' ') {
                format!("\"{s}\"")
            } else {
                s.into_owned()
            }
        })
        .collect();
    format!("{prog} {}", args.join(" "))
}

#[cfg(unix)]
fn get_unix_user() -> Option<String> {
    let uid = Command::new("id").arg("-u").output().ok()?;
    let gid = Command::new("id").arg("-g").output().ok()?;
    let uid = String::from_utf8(uid.stdout).ok()?.trim().to_string();
    let gid = String::from_utf8(gid.stdout).ok()?.trim().to_string();
    if uid.is_empty() || gid.is_empty() {
        None
    } else {
        Some(format!("{uid}:{gid}"))
    }
}
