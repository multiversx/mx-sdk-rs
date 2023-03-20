use std::{
    path::{Path, PathBuf},
    process::Command,
};

use multiversx_sc_meta::{cli_args::TemplateArgs, template::TemplateCreator};

#[tokio::test]
async fn test_serialize_multi_contract() {
    let args = TemplateArgs {
        name: "adder".to_string(),
        template: "adder".to_string(),
    };

    let test_path = Path::new(".")
        .canonicalize()
        .unwrap_or_else(|err| {
            panic!("error canonicalizing input path: {err}",);
        })
        .join("tests");

    let build_dir = test_path.join("adder");

    let _ = TemplateCreator::with_path(test_path)
        .download_contract_template(&args)
        .await;
    cargo_build(build_dir);
}

pub fn cargo_build(contract_location: PathBuf) {
    let exit_status = Command::new("cargo")
        .args(["build"])
        .current_dir(contract_location)
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract clean process was not running");

    assert!(exit_status.success(), "contract build process failed");
}
