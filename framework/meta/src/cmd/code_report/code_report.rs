use std::{
    fs::{self, read_dir, File},
    path::PathBuf,
    process::Command,
};

use crate::{cli::OutputFormat, folder_structure::RelevantDirectories};

use multiversx_sc_meta_lib::{
    self, code_report_json::CodeReportJson, mxsc_file_json::MxscFileJson,
};

use super::render_code_report::render_report;

pub fn run_code_report(path: &str, output_path: &str, output_format: &OutputFormat) {
    let directors = RelevantDirectories::find_all(path, &["".to_owned()]);

    let reports = extract_report(directors);
    let mut output = String::new();

    match output_format {
        OutputFormat::Markdown => {
            render_report(&mut output, &reports);
        },
        OutputFormat::Json => {
            for report in reports {
                output.insert_str(
                    output.len(),
                    &serde_json::to_string_pretty(&report).unwrap(),
                );
            }
        },
    };

    let Ok(_) = fs::write(output_path, output) else {
        return;
    };
}

fn extract_report(directors: RelevantDirectories) -> Vec<CodeReportJson> {
    let mut reports: Vec<CodeReportJson> = Vec::new();

    for director in directors.iter() {
        build_contract(&director.path);

        let output_path: PathBuf = director.path.join("output");

        extract_reports(&output_path, &mut reports);

        clean_contract(&director.path);
    }

    reports
}

fn build_contract(path: &PathBuf) {
    Command::new("sc-meta")
        .arg("all")
        .arg("build")
        .arg("--path")
        .arg(path)
        .output()
        .unwrap_or_else(|_| panic!("Failed to build the contract for path: {}", path.display()));
}

fn clean_contract(path: &PathBuf) {
    Command::new("sc-meta")
        .arg("all")
        .arg("clean")
        .arg("--path")
        .arg(path)
        .output()
        .unwrap_or_else(|_| panic!("Failed to clean the contract for path: {}", path.display()));
}

fn find_mxsc_files(path: &PathBuf) -> Vec<PathBuf> {
    if !path.is_dir() {
        return vec![];
    }

    let mut mxsc_files = Vec::new();
    for entry in read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        if file_path.to_str().unwrap().ends_with(".mxsc.json") {
            mxsc_files.push(file_path);
        }
    }

    mxsc_files
}

fn extract_reports(path: &PathBuf, reports: &mut Vec<CodeReportJson>) {
    for mxsc_path in find_mxsc_files(path) {
        let mxsc_file =
            File::open(mxsc_path).unwrap_or_else(|_| panic!("Failed to open mxsc file"));
        let data: MxscFileJson = serde_json::from_reader(mxsc_file).unwrap();
        reports.push(data.report.code_report);
    }
}
