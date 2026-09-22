# wasm-opcode-counter

A small CLI tool that counts the occurrences of WASM opcodes across all `.wasm`
files found (recursively) in a folder.

## Usage

```bash
cargo run --release -p wasm-opcode-counter -- <folder>
```

If `<folder>` is omitted, the current directory is scanned.

### Options

- `--per-file` — also print a per-file opcode breakdown, in addition to the combined totals.
- `--top <N>` — only print the N most frequent opcodes (applies to the totals, and to each file when `--per-file` is set).

### Example

```bash
cargo run --release -p wasm-opcode-counter -- ./output --per-file --top 10
```
