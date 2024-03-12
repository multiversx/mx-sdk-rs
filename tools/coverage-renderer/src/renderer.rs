use crate::llvm_cov::{Coverage, FileSummary, Summary};

pub fn render_coverage(coverage: &Coverage, root: &str) {
    render_header();
    render_totals(&coverage.totals);
    render_files(&coverage.files, root);
}

fn render_header() {
    println!("# Coverage Summary");
    println!();
}

fn render_totals(summary: &Summary) {
    println!("## Totals");

    println!("| | Count | Covered | % |");
    println!("|---|---|---|---|");
    println!(
        "| Lines | {} | {} | {:.2} |",
        summary.lines.count, summary.lines.covered, summary.lines.percent
    );
    println!(
        "| Regions | {} | {} | {:.2} |",
        summary.regions.count, summary.regions.covered, summary.regions.percent
    );
    println!(
        "| Functions | {} | {} | {:.2} |",
        summary.functions.count, summary.functions.covered, summary.functions.percent
    );
    println!(
        "| Instantiations | {} | {} | {:.2} |",
        summary.instantiations.count,
        summary.instantiations.covered,
        summary.instantiations.percent
    );

    println!();
}

fn render_files(files: &[FileSummary], root: &str) {
    println!("## Files");
    println!("<details><summary>Expand</summary>\n");
    println!("| File | Lines | Regions | Functions | Instantiations |");
    println!("|---|---|---|---|---|");
    for file in files {
        render_file(file, root);
    }

    println!("</details>");
}

fn render_file(file: &FileSummary, root: &str) {
    let summary = &file.summary;
    let filename = file.filename.strip_prefix(root).unwrap();

    println!(
        "| {} | {:.2}% | {:.2}% | {:.2}% | {:.2}% |",
        filename,
        summary.lines.percent,
        summary.regions.percent,
        summary.functions.percent,
        summary.instantiations.percent
    );
}
