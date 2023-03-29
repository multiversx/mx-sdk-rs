use multiversx_sc_meta::template::{template_names_from_repo, RepoSource, TemplateDownloader};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const TEMPLATE_TEMP_DIR_NAME: &str = "template-test";

#[test]
fn test_template_list() {
    let workspace_path = find_workspace();
    let repo_source = RepoSource::from_local_path(&workspace_path);
    let template_names = template_names_from_repo(&repo_source);
    assert_eq!(template_names.len(), 2);
}

#[tokio::test]
// #[ignore]
async fn test_template_download() {
    let workspace_path = find_workspace();
    let repo_source = RepoSource::from_local_path(&workspace_path);

    let template_temp_path = find_workspace().join(TEMPLATE_TEMP_DIR_NAME);
    if template_temp_path.exists() {
        fs::remove_dir_all(&template_temp_path).unwrap();
    }
    fs::create_dir_all(&template_temp_path).unwrap();

    let target_dir = template_temp_path.join("new-adder");

    // let args = TemplateArgs {
    //     name: target_dir.clone(),
    //     template: "adder".to_string(),
    // };
    let downloader = TemplateDownloader::new(&repo_source, "adder".to_string(), target_dir);
    downloader.template_download();

    // let _ = TemplateCreator::with_path(template_temp_path)
    //     .download_contract_template(&args)
    //     .await;

    // cargo_test(build_dir);
}

pub fn cargo_test(contract_location: PathBuf) {
    let exit_status = Command::new("cargo")
        .args(["test"])
        .current_dir(contract_location)
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract test process failed");
}

/// Finds the workspace by taking the `current_exe` and working its way up.
/// Works in debug mode too.
///
/// TODO: duplicated code from scenario_world. De-duplicate after dependencies are reorganized.
pub fn find_workspace() -> PathBuf {
    let current_exe = std::env::current_exe().unwrap();
    let mut path = current_exe.as_path();
    while !is_target(path) {
        path = path.parent().unwrap();
    }

    path.parent().unwrap().into()
}

fn is_target(path_buf: &Path) -> bool {
    path_buf.file_name().unwrap() == "target"
}
