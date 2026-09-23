use clap::Parser as ClapParser;
use std::{
    collections::HashMap,
    fs,
    io::{self, IsTerminal, Write},
    path::{Path, PathBuf},
};
use wasmparser::{Operator, Parser as WasmParser, Payload};

/// Counts the occurrences of WASM opcodes in all `.wasm` files found
/// (recursively) in a folder.
#[derive(ClapParser, Debug)]
#[command(
    name = "wasm-opcode-counter",
    about = "Counts the occurrences of WASM opcodes in all .wasm files in a folder"
)]
struct Args {
    /// Folder to scan recursively for .wasm files.
    #[arg(default_value = ".")]
    folder: PathBuf,

    /// Also print a per-file opcode breakdown, in addition to the totals.
    #[arg(long)]
    per_file: bool,

    /// Only print the top N most frequent opcodes (applies to the totals,
    /// and to each file when --per-file is set).
    #[arg(long)]
    top: Option<usize>,
}

type OpcodeCounts = HashMap<String, usize>;

fn main() {
    let args = Args::parse();

    let mut scan_errors = 0usize;
    let wasm_files = collect_wasm_files(&args.folder, &mut scan_errors);
    if wasm_files.is_empty() {
        eprintln!("No .wasm files found under {}", args.folder.display());
        if scan_errors > 0 {
            eprintln!("{scan_errors} folder(s) could not be scanned.");
        }
        std::process::exit(1);
    }

    let mut total_counts: OpcodeCounts = HashMap::new();
    let mut total_ops = 0usize;
    let mut failed_files = 0usize;
    let mut progress = Progress::new(wasm_files.len());

    for wasm_path in &wasm_files {
        progress.step(wasm_path);

        match count_opcodes_in_file(wasm_path) {
            Ok(file_counts) => {
                let file_total: usize = file_counts.values().sum();
                total_ops += file_total;

                if args.per_file {
                    progress.clear_line();
                    println!("\n{}", wasm_path.display());
                    print_counts(&file_counts, file_total, args.top);
                }

                for (opcode, count) in file_counts {
                    *total_counts.entry(opcode).or_insert(0) += count;
                }
            }
            Err(err) => {
                progress.clear_line();
                eprintln!("Failed to parse {}: {err}", wasm_path.display());
                failed_files += 1;
            }
        }
    }
    progress.finish();

    let parsed_files = wasm_files.len() - failed_files;
    if failed_files == 0 {
        println!("\nTotals across {} file(s):", wasm_files.len());
    } else {
        println!(
            "\nTotals across {parsed_files} of {} discovered file(s) ({failed_files} failed to parse):",
            wasm_files.len()
        );
    }
    print_counts(&total_counts, total_ops, args.top);

    if failed_files > 0 || scan_errors > 0 {
        eprintln!(
            "\n{failed_files} .wasm file(s) failed to parse and {scan_errors} folder(s)/entries \
             could not be scanned; the totals above are incomplete."
        );
        std::process::exit(1);
    }
}

/// Reports "[i/N]: <path>" progress on stderr while files are processed.
///
/// When stderr is a terminal, each step overwrites the previous line in
/// place; otherwise (e.g. output redirected to a log file) it falls back to
/// printing one line per file, so progress stays visible without the
/// carriage-return control characters cluttering the log.
struct Progress {
    total: usize,
    current: usize,
    is_terminal: bool,
    line_width: usize,
}

impl Progress {
    fn new(total: usize) -> Self {
        Progress {
            total,
            current: 0,
            is_terminal: io::stderr().is_terminal(),
            line_width: 0,
        }
    }

    fn step(&mut self, path: &Path) {
        self.current += 1;
        let message = format!(
            "Processing [{}/{}]: {}",
            self.current,
            self.total,
            path.display()
        );

        if self.is_terminal {
            self.line_width = self.line_width.max(message.len());
            eprint!("\r{message:<width$}", width = self.line_width);
            let _ = io::stderr().flush();
        } else {
            eprintln!("{message}");
        }
    }

    /// Erases the in-place progress line so other output (per-file reports,
    /// error messages) can be printed cleanly above it. No-op when progress
    /// isn't rendered in place.
    fn clear_line(&self) {
        if self.is_terminal {
            eprint!("\r{:<width$}\r", "", width = self.line_width);
            let _ = io::stderr().flush();
        }
    }

    fn finish(&self) {
        self.clear_line();
    }
}

/// Recursively collects all `.wasm` files under `folder`, sorted by path.
///
/// `scan_errors` is incremented for every directory or entry that could not
/// be read, so the caller can tell a clean scan apart from one where some
/// part of the tree (and therefore some `.wasm` files) may be missing from
/// the result entirely.
fn collect_wasm_files(folder: &Path, scan_errors: &mut usize) -> Vec<PathBuf> {
    let mut result = Vec::new();
    collect_wasm_files_recursive(folder, &mut result, scan_errors);
    result.sort();
    result
}

fn collect_wasm_files_recursive(folder: &Path, result: &mut Vec<PathBuf>, scan_errors: &mut usize) {
    let entries = match fs::read_dir(folder) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to read directory {}: {err}", folder.display());
            *scan_errors += 1;
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Failed to read an entry in {}: {err}", folder.display());
                *scan_errors += 1;
                continue;
            }
        };

        let path = entry.path();
        if path.is_dir() {
            collect_wasm_files_recursive(&path, result, scan_errors);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("wasm") {
            result.push(path);
        }
    }
}

/// Parses a single `.wasm` file and counts how many times each opcode
/// appears across all of its function bodies.
fn count_opcodes_in_file(path: &Path) -> Result<OpcodeCounts, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    let mut counts: OpcodeCounts = HashMap::new();

    for payload in WasmParser::new(0).parse_all(&data) {
        let payload = payload.map_err(|err| err.to_string())?;
        if let Payload::CodeSectionEntry(body) = payload {
            let reader = body.get_operators_reader().map_err(|err| err.to_string())?;
            for op in reader {
                let op = op.map_err(|err| err.to_string())?;
                *counts.entry(opcode_name(op)).or_insert(0) += 1;
            }
        }
    }

    Ok(counts)
}

/// Extracts the opcode's name, discarding any operator payload/immediates,
/// e.g. `I32Const { value: 5 }` becomes `I32Const`.
fn opcode_name(op: Operator) -> String {
    let op_str = format!("{op:?}");
    op_str
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_owned()
}

/// Prints a `name  count  percentage` table, sorted by descending count
/// (alphabetically on ties), optionally truncated to the top N entries.
fn print_counts(counts: &OpcodeCounts, total: usize, top: Option<usize>) {
    let mut sorted: Vec<(&String, &usize)> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    if let Some(top) = top {
        sorted.truncate(top);
    }

    let name_width = sorted.iter().map(|(name, _)| name.len()).max().unwrap_or(5);

    for (name, count) in &sorted {
        let percentage = if total > 0 {
            100.0 * **count as f64 / total as f64
        } else {
            0.0
        };
        println!("  {name:<name_width$}  {count:>10}  {percentage:>6.2}%");
    }

    println!("  {:-<width$}", "", width = name_width + 21);
    println!("  {:<name_width$}  {total:>10}", "TOTAL");
}
