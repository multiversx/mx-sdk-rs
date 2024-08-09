use std::{
    fs::{read_dir, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use crate::{
    cli::{CompareArgs, CompileArgs, ConvertArgs},
    folder_structure::RelevantDirectories,
};

use multiversx_sc_meta_lib::{
    self, code_report_json::CodeReportJson, mxsc_file_json::MxscFileJson,
};

use super::render_code_report::CodeReportRender;

const JSON: &str = ".json";
const MD: &str = ".md";

pub fn compare_report(compare_args: &CompareArgs) {
    if !is_path_ends_with(&compare_args.output, MD) {
        panic!("Compare output is available only for Markdown file extension.");
    }

    if !is_path_ends_with(&compare_args.baseline, JSON)
        && !is_path_ends_with(&compare_args.new, JSON)
    {
        panic!("Compare baseline and new is available only for JSON file extension.");
    }

    let mut output_file = create_file(&compare_args.output);

    let baseline_reports: Vec<CodeReportJson> = match File::open(&compare_args.baseline) {
        Ok(_) => extract_reports_from_json(&compare_args.baseline),
        Err(_) => vec![],
    };

    let new_reports: Vec<CodeReportJson> = extract_reports_from_json(&compare_args.new);

    let mut render_code_report =
        CodeReportRender::new(&mut output_file, &baseline_reports, &new_reports);
    render_code_report.compare_reports();
}

pub fn convert_report(convert_args: &ConvertArgs) {
    if !is_path_ends_with(&convert_args.output, MD) {
        panic!("Conversion output is available only for Markdown file extension");
    }

    if !is_path_ends_with(&convert_args.input, JSON) {
        panic!("Conversion available only from JSON file extension");
    }

    let mut output_file = create_file(&convert_args.output);

    let reports: Vec<CodeReportJson> = extract_reports_from_json(&convert_args.input);

    let mut convert_code_report = CodeReportRender::new_without_compare(&mut output_file, &reports);

    convert_code_report.render_report();
}

pub fn create_report(compile_args: &CompileArgs) {
    if !is_path_ends_with(&compile_args.output, JSON)
        && !is_path_ends_with(&compile_args.output, MD)
    {
        panic!("Create report is available only for Markdown or JSON output file.")
    }

    let reports = generate_new_report(&compile_args.path);

    let mut file = create_file(&compile_args.output);

    if is_path_ends_with(&compile_args.output, MD) {
        let mut render_code_report = CodeReportRender::new_without_compare(&mut file, &reports);
        render_code_report.render_report();
    } else {
        let json_output = serde_json::to_string(&reports).unwrap();
        file.write_all(json_output.as_bytes()).unwrap();
    }
}

fn generate_new_report(path: &PathBuf) -> Vec<CodeReportJson> {
    let directors = RelevantDirectories::find_all(path, &["".to_owned()]);

    assemble_report_vec(directors)
}

fn assemble_report_vec(directors: RelevantDirectories) -> Vec<CodeReportJson> {
    let mut reports: Vec<CodeReportJson> = Vec::new();

    for director in directors.iter() {
        let output_path: PathBuf = director.path.join("output");

        collect_reports(&output_path, &mut reports);
        sanitize_output_path_from_report(&mut reports);
    }

    reports
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

fn collect_reports(path: &PathBuf, reports: &mut Vec<CodeReportJson>) {
    for mxsc_path in find_mxsc_files(path) {
        let mxsc_file = match File::open(mxsc_path) {
            Ok(file) => file,
            Err(_) => continue,
        };
        let data: MxscFileJson = serde_json::from_reader(mxsc_file).unwrap();
        reports.push(data.report.code_report);
    }
}

fn create_file(file_path: &PathBuf) -> File {
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

fn is_path_ends_with(path: &Path, extension: &str) -> bool {
    path.to_path_buf()
        .into_os_string()
        .into_string()
        .unwrap()
        .ends_with(extension)
}

fn extract_reports_from_json(path: &PathBuf) -> Vec<CodeReportJson> {
    let file =
        File::open(path).unwrap_or_else(|_| panic!("file with path {} not found", path.display()));
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
}
