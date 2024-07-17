use std::{fs::File, path::PathBuf, process::Command};

use crate::folder_structure::RelevantDirectories;

use multiversx_sc_meta_lib::{
    self, code_report_json::CodeReportJson, mxsc_file_json::MxscFileJson,
};

use super::render_code_report::render_report;

pub fn run_code_report(
    path: &str,
    // output_path: &str,
    // output_format: &OutputFormat,
) {
    let directors = RelevantDirectories::find_all(path, &["".to_owned()]);

    let reports = extract_report(directors);
    let mut output = String::new();
    render_report(&mut output, &reports);

    println!("{output}");
}

fn build_contract(path: &PathBuf) {
    Command::new("sc-meta")
        .arg("all")
        .arg("build")
        .arg("--path")
        .arg(path)
        .output()
        .expect(&format!(
            "Failed to build the contract for path: {}",
            path.display()
        ));
}

fn clean_contract(path: &PathBuf) {
    Command::new("sc-meta")
        .arg("all")
        .arg("clean")
        .arg("--path")
        .arg(path)
        .output()
        .expect(&format!(
            "Failed to clean the contract for path: {}",
            path.display()
        ));
}

fn extract_report(directors: RelevantDirectories) -> Vec<CodeReportJson> {
    let mut reports: Vec<CodeReportJson> = Vec::new();

    for director in directors.iter() {
        build_contract(&director.path);

        // find only one wasm file

        let contract_name = director.path.to_str().unwrap().split("/").last().unwrap();
        println!(">> {contract_name}");

        let mxsc_path = format!(
            "{}/output/{contract_name}.mxsc.json",
            director.path.display()
        );
        let mxsc_file = File::open(mxsc_path);

        match mxsc_file {
            Ok(f) => {
                let data: MxscFileJson = serde_json::from_reader(f).unwrap();

                reports.push(data.report.code_report);

                clean_contract(&director.path);
            },
            Err(_) => continue,
        }
    }

    reports
}
