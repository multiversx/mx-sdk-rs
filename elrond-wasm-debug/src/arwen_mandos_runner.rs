use std::path::Path;
use std::{path::PathBuf, process::Command};

/// Runs the Arwen executable,
/// which reads parses and executes one or more mandos tests.
pub fn mandos_go<P: AsRef<Path>>(relative_path: P) {
	if cfg!(not(feature = "arwen-tests")) {
		return;
	}

	let mut absolute_path = std::env::current_dir().unwrap();
	absolute_path.push(relative_path);

	let exec_path = arwen_mandos_full_path();

	let output = Command::new(exec_path)
		.arg(absolute_path)
		.output()
		.expect("failed to execute process");

	if output.status.success() {
		println!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
	} else {
		panic!(
			"Mandos-go output:\n{}\n{}",
			String::from_utf8_lossy(output.stdout.as_slice()),
			String::from_utf8_lossy(output.stderr.as_slice())
		);
	}
}

fn arwen_mandos_full_path() -> PathBuf {
	let crate_dir = env!("CARGO_MANIFEST_DIR");
	let mut am_exec_path = PathBuf::from(crate_dir);
	am_exec_path.push("arwenmandos");
	am_exec_path
}
