use std::path::PathBuf;
use std::process::{Child, Command, ExitCode, Stdio};
use std::time::Duration;

use multiversx_sc_meta_lib::tools::find_current_workspace;

struct CsTest {
    /// Contract directories to build with `sc-meta all build` (relative to workspace root).
    /// Empty if no contracts need to be built for this test.
    build_paths: &'static [&'static str],
    /// Cargo package name.
    package: &'static str,
    /// Integration test file name (without .rs extension).
    test_file: &'static str,
}

static CS_TESTS: &[CsTest] = &[
    // The adder wasm is pre-built and checked in under framework/meta/tests/cs_tx_cli_test/.
    CsTest {
        build_paths: &[],
        package: "multiversx-sc-meta",
        test_file: "cs_tx_cli_test",
    },
    CsTest {
        build_paths: &["contracts/examples/adder"],
        package: "basic-interactor",
        test_file: "basic_interactor_cs_test",
    },
    CsTest {
        build_paths: &["contracts/examples/ping-pong-egld"],
        package: "ping-pong-egld-interact",
        test_file: "ping_pong_interact_cs_test",
    },
    CsTest {
        build_paths: &["contracts/feature-tests/basic-features"],
        package: "basic-features-interact",
        test_file: "bf_interact_cs_test",
    },
    CsTest {
        build_paths: &["contracts/feature-tests/composability/forwarder"],
        package: "forwarder-interact",
        test_file: "interact_cs_tests",
    },
    CsTest {
        build_paths: &["contracts/feature-tests/payable-features"],
        package: "payable-interactor",
        test_file: "payable_interactor_cs_test",
    },
    // Tests built-in system contracts — no wasm build step required.
    CsTest {
        build_paths: &[],
        package: "system-sc-interact",
        test_file: "chain_simulator_token_tests",
    },
];

fn run(program: &str, args: &[&str], cwd: &PathBuf) -> bool {
    println!("+ {program} {}", args.join(" "));
    Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .unwrap_or_else(|e| panic!("failed to launch `{program}`: {e}"))
        .success()
}

/// Starts the chain simulator and stops it when dropped, ensuring cleanup on both normal exit and panic.
struct ChainSimulatorGuard<'a> {
    workspace: &'a PathBuf,
    child: Child,
}

impl<'a> ChainSimulatorGuard<'a> {
    fn start(workspace: &'a PathBuf) -> Self {
        println!("\n=== Starting chain simulator ===");
        let child = Command::new("sc-meta")
            .args(["cs", "start"])
            .current_dir(workspace)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to start chain simulator");
        std::thread::sleep(Duration::from_secs(5));
        Self { workspace, child }
    }
}

impl Drop for ChainSimulatorGuard<'_> {
    fn drop(&mut self) {
        println!("\n=== Stopping chain simulator ===");
        let _ = Command::new("sc-meta")
            .args(["cs", "stop"])
            .current_dir(self.workspace)
            .status();
        let _ = self.child.wait();
    }
}

fn main() -> ExitCode {
    let workspace = find_current_workspace().expect("could not locate workspace root");
    let mut failed = 0u32;

    println!("=== Building contracts ===\n");
    for test in CS_TESTS {
        for path in test.build_paths {
            let abs_path = workspace.join(path);
            let abs_path_str = abs_path.to_str().unwrap();
            if !run(
                "sc-meta",
                &["all", "build", "--path", abs_path_str],
                &workspace,
            ) {
                eprintln!("ERROR: build failed for {path}");
                failed += 1;
            }
        }
    }

    println!("\n=== Running chain simulator tests ===\n");
    let _cs_guard = ChainSimulatorGuard::start(&workspace);
    for test in CS_TESTS {
        println!("--- {} / {} ---", test.package, test.test_file);

        if !run(
            "cargo",
            &[
                "test",
                "-p",
                test.package,
                "--test",
                test.test_file,
                "--features",
                "chain-simulator-tests",
            ],
            &workspace,
        ) {
            eprintln!("ERROR: test failed for package {}", test.package);
            failed += 1;
        }
    }

    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        eprintln!("\n{failed} step(s) failed.");
        ExitCode::FAILURE
    }
}
