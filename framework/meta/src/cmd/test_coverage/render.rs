use super::llvm_cov::{Coverage, FileSummary, Summary};

use std::fmt::Write;

fn writeln_output_str<S: AsRef<str>>(output: &mut String, input: S) {
    output.write_str(&format!("{}\n", input.as_ref())).ok();
}

pub fn render_coverage(output: &mut String, coverage: &Coverage, root: &str) {
    render_header(output);
    render_totals(output, &coverage.totals);
    render_files(output, &coverage.files, root);
}

fn render_header(output: &mut String) {
    writeln_output_str(output, "# Coverage Summary\n");
}

fn render_totals(output: &mut String, summary: &Summary) {
    writeln_output_str(output, "## Totals");

    writeln_output_str(output, "| | Count | Covered | % |");
    writeln_output_str(output, "|---|---|---|---|");
    writeln_output_str(
        output,
        format!(
            "| Lines | {} | {} | {:.2} |",
            summary.lines.count, summary.lines.covered, summary.lines.percent
        ),
    );
    writeln_output_str(
        output,
        format!(
            "| Regions | {} | {} | {:.2} |",
            summary.regions.count, summary.regions.covered, summary.regions.percent
        ),
    );
    writeln_output_str(
        output,
        format!(
            "| Functions | {} | {} | {:.2} |",
            summary.functions.count, summary.functions.covered, summary.functions.percent
        ),
    );
    writeln_output_str(
        output,
        format!(
            "| Instantiations | {} | {} | {:.2} |\n",
            summary.instantiations.count,
            summary.instantiations.covered,
            summary.instantiations.percent
        ),
    );
}

fn render_files(output: &mut String, files: &[FileSummary], root: &str) {
    writeln_output_str(output, "## Files");
    writeln_output_str(output, "<details><summary>Expand</summary>\n");
    writeln_output_str(
        output,
        "| File | Lines | Regions | Functions | Instantiations |",
    );
    writeln_output_str(output, "|---|---|---|---|---|");
    for file in files {
        render_file(output, file, root);
    }

    writeln_output_str(output, "</details>");
}

fn render_file(output: &mut String, file: &FileSummary, root: &str) {
    let summary = &file.summary;
    let filename = file.filename.strip_prefix(root).unwrap_or(&file.filename);

    writeln_output_str(
        output,
        format!(
            "| {} | {:.2}% | {:.2}% | {:.2}% | {:.2}% |",
            filename,
            summary.lines.percent,
            summary.regions.percent,
            summary.functions.percent,
            summary.instantiations.percent
        ),
    );
}
