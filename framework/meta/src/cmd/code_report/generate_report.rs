use std::{
    fs::{read_dir, File},
    io::{BufReader, Write},
    path::PathBuf,
    process::Command,
};

use crate::{cli::OutputFormat, folder_structure::RelevantDirectories};

use multiversx_sc_meta_lib::{
    self, code_report_json::CodeReportJson, mxsc_file_json::MxscFileJson,
};

use super::render_code_report::CodeReportRender;

pub fn run_code_report(
    path: &str,
    output_path: &str,
    output_format: &OutputFormat,
    compare: Vec<String>,
) {
    let reports = if compare.is_empty() {
        generate_new_report(path)
    } else {
        let file = File::open(&compare[0]).expect("file not found");
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).expect("Cannot deserialize")
    };

    let mut compared_to = String::new();
    if compare.len() == 2 {
        compare[1].clone_into(&mut compared_to);
    }

    let mut file = create_file(output_path);

    match output_format {
        OutputFormat::Markdown => {
            let mut render_code_report = CodeReportRender::new(&mut file, &compared_to, &reports);
            render_code_report.render_report();
        },
        OutputFormat::Json => {
            let json_output = serde_json::to_string(&reports).unwrap();
            file.write_all(json_output.as_bytes()).unwrap();
        },
    };
}

fn generate_new_report(path: &str) -> Vec<CodeReportJson> {
    let directors = RelevantDirectories::find_all(path, &["".to_owned()]);

    extract_report(directors)
}

fn extract_report(directors: RelevantDirectories) -> Vec<CodeReportJson> {
    let mut reports: Vec<CodeReportJson> = Vec::new();

    for director in directors.iter() {
        build_contract(&director.path);

        let output_path: PathBuf = director.path.join("output");

        extract_reports(&output_path, &mut reports);

        sanitize_output_path_from_report(&mut reports);

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

fn create_file(file_path: &str) -> File {
    File::create(file_path).expect("could not write report file")
}

fn sanitize_output_path_from_report(reports: &mut [CodeReportJson]) {
    reports.iter_mut().for_each(|report| {
        report.path = report
            .path
            .split('/')
            .last()
            .unwrap_or(&report.path)
            .to_string();
    })
}
